use std::sync::{Arc, Mutex};
use std::collections::VecDeque;
use std::thread;
use crate::request;
use crate::ppa;
use crate::sdf;
use crate::util;
use wd_log::*;

fn read_badbin() -> Vec<u16> {
    // let path = util::str_concat(sdf::USR_DIR, "badblock.bin");
    let path = "/Users/yuan/program_project/rust_sdf/src/badblock.bin";
    let badbin = match util::parse_read_file(&path) {
        Ok(data) => data,
        Err(_) => {
            log_error_ln!("read file failed");
            std::process::exit(1);
        }
    };

    if ((badbin.len() * 2) as u32) < (util::cfg_1TB::CFG_NAND_BLOCK_NUM * util::cfg_1TB::CFG_NAND_LUN_NUM * util::cfg_1TB::CFG_NAND_CHANNEL_NUM / 8) {
        log_error_ln!("file length is too short.filelen: {} real len{}", badbin.len(), util::cfg_1TB::CFG_NAND_BLOCK_NUM * util::cfg_1TB::CFG_NAND_LUN_NUM * util::cfg_1TB::CFG_NAND_CHANNEL_NUM / 8);
        std::process::exit(1);
    }
    
    badbin
}

pub struct Die {
    pub chl_id: u32,
    pub die_id: u32,
    pub num_blocks: u32,
    pub io_thread: Option<thread::JoinHandle<()>>,
    pub req_queue: Arc<Mutex<VecDeque<request::Request>>>,
    pub completion_queue: Arc<Mutex<VecDeque<request::Request>>>,
    pub free_block_list: Vec<ppa::PPA>,
    pub open_block: Option<ppa::PPA>,
    pub open_block_write_ptr: u32,
}

impl Die {
    pub fn new(chl_id: u32, die_id: u32, num_blocks: u32) -> Self {
        let req_queue:Arc<Mutex<VecDeque<request::Request>>> = Arc::new(Mutex::new(VecDeque::new()));
        let completion_queue:Arc<Mutex<VecDeque<request::Request>>> = Arc::new(Mutex::new(VecDeque::new()));
        let free_block_list = Vec::new();
        let req_queue_clone = req_queue.clone();
        let completion_queue_clone = completion_queue.clone();
        let io_thread = Some(thread::Builder::new().name(format!("die thread,chl: {} die: {}", chl_id, die_id).to_string()).
                                                                spawn(move || Die::launch_io_thread(chl_id, die_id, req_queue_clone, completion_queue_clone)).unwrap());

        Die {
            chl_id,
            die_id,
            num_blocks,
            io_thread,
            req_queue,
            completion_queue,
            free_block_list,
            open_block: None,
            open_block_write_ptr: 0,
        }
    }

    pub fn scan_free_blocks(&mut self){
        // Implement scan_free_blocks logic
        let devid:u32 = 0;
        let nsid:u32 = 1;
        let qid:u16 = 1;
        let mut flag = util::nexus::GOOD_PPA;

        let mut channel: u32 = 0;
        let mut channel_mask:u16 = 0xffff;

        let badbin:Vec<u16> = read_badbin();
        util::read_nvme_reg32(devid, util::nexus::CHANNEL_COUNT, &mut channel);

        assert!(self.num_blocks <= (util::cfg_1TB::CFG_NAND_BLOCK_NUM - sdf::RESERVED_SUPERBLK) * util::cfg_1TB::CFG_NAND_PLANE_NUM);

        let mut blocks_allocated = 0;
        for i_blk in 0..=(util::cfg_1TB::CFG_NAND_BLOCK_NUM - sdf::RESERVED_SUPERBLK - 1) {
            if blocks_allocated >= self.num_blocks {
                break;
            }
            for pl_val in 0..util::cfg_1TB::CFG_NAND_PLANE_NUM {
                if blocks_allocated >= self.num_blocks {
                    break;
                }
                let ppa = ppa::PPA::new2(self.chl_id, self.die_id, pl_val as u32, i_blk, 0);
                flag = util::skip_ppa(ppa.addr(), &badbin, channel_mask);
                if flag == util::nexus::BAD_PPA {
                    continue;
                } else {
                    util::ersppa_sync(devid, nsid, ppa.addr(), qid, 0);
                    self.free_block_list.push(ppa);
                    blocks_allocated += 1;
                }
            }
        }

        self.open_block = self.free_block_list.first().cloned();
        self.open_block_write_ptr = 0;
    }
    

    pub fn write_page(&self, req: &request::Request) -> i32 {
        assert_eq!(req.ppa.die, self.die_id as u32);
        let mut queue = self.req_queue.lock().unwrap();
        log_debug_ln!("die write push request {}", req);
        queue.push_front(req.clone());
        0
    }

    pub fn read_page(&self, req: &request::Request) -> i32 {
        assert_eq!(req.ppa.die, self.die_id as u32);
        let mut queue = self.req_queue.lock().unwrap();
        log_debug_ln!("die read push request {}", req);
        queue.push_front(req.clone());
        0
    }
    // todo check why we need request in argument
    pub fn wait_for(&self, req: &request::Request) -> i32 {
        loop{
            let mut queue = self.completion_queue.lock().unwrap();
            if queue.len() > 0 {
                let ret_req = queue.pop_back().unwrap();
                let ret = ret_req.ret;
                log_debug_ln!("die wait pop request {}", ret_req);
                return ret;
            }
        }
    }

    pub fn alloc_ppa(&mut self) -> ppa::PPA {
        let ppa = self.open_block.expect("No open block available").clone();
        if self.open_block_write_ptr < (sdf::PAGES_PER_BLK - 1) {
            self.open_block_write_ptr += 1;
        } else {
            self.free_block_list.pop();
            assert!(self.free_block_list.len() > 0);
            self.open_block = Some(self.free_block_list[0]);
            self.open_block_write_ptr = 0;
        }
        let mut alloc_ppa = ppa;
        alloc_ppa.pg = self.open_block_write_ptr;
        alloc_ppa
    }

    pub fn stop(&mut self) -> i32 {
        {
            let req = request::Request::new1(0, 0, ppa::PPA::new(0), 0, request::END_OP);
            let mut queue = self.req_queue.lock().unwrap();
            queue.push_front(req);
        }
        // let mut io_thread = thread::spawn(|| {});
        // std::mem::swap(&mut self.io_thread, &mut io_thread);
        // io_thread.join().expect("Failed to join IO thread");
        self.io_thread.take().expect("Called stop on non-running thread").join().expect("Could not join spawned thread");
        log_info_ln!("Stop die thread,chl id: {} die id: {} ", self.chl_id, self.die_id);
        0
    }
    //todo
    pub fn get_free_block_list(&self) -> &Vec<ppa::PPA> {
        &self.free_block_list
    }

    pub fn launch_io_thread(chl_id: u32, die_id: u32, req_queue: Arc<Mutex<VecDeque<request::Request>>>, completion_queue: Arc<Mutex<VecDeque<request::Request>>>) {
        log_info_ln!("Start die thread,chl id: {} die id: {}", chl_id, die_id);
        while true {
            if let Some(mut req) = req_queue.lock().unwrap().pop_back() {
                match req.op {
                    sdf::WRITE_OP => {
                        log_info_ln!(
                            "die IO thread launched for chl {}, die {} write ppa {}",
                            chl_id,
                            die_id,
                            req.ppa
                        );
                        req.ret = util::write_data_ppa(0, 1, req.ppa.addr(), 1, 3, &req.buf, &req.metabuf);
                    }
                    sdf::READ_OP => {
                        log_info_ln!(
                            "die IO thread launched for chl {}, die {} write ppa {}",
                            chl_id,
                            die_id,
                            req.ppa
                        );
                        req.ret = util::read_data_ppa(0, 1, req.ppa.addr(), 1, 3, &req.buf, &req.metabuf);
                    }
                    sdf::END_OP => {
                        log_info_ln!("die thread Recieve ENDOP");
                        return;
                    }
                    _ => {
                        log_warn_ln!("invalid operation")
                    }
                }
                log_debug_ln!("die push to completion queue req {}", req);
                completion_queue.lock().unwrap().push_front(req);
            } else {
                continue;
            } 
        }
    }
}