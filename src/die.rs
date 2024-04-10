use std::sync::{Arc, Mutex};
use std::collections::VecDeque;
use std::thread;
use crate::request;
use crate::ppa;
use crate::sdf;
use crate::util;
use std::marker::PhantomData;

pub struct Die<'a> {
    pub chl_id: u32,
    pub die_id: u32,
    pub num_blocks: u32,
    pub io_thread: thread::JoinHandle<()>,
    pub req_queue: Arc<Mutex<VecDeque<request::Request>>>,
    pub completion_queue: Arc<Mutex<VecDeque<request::Request>>>,
    pub free_block_list: Vec<ppa::PPA>,
    pub open_block: Option<ppa::PPA>,
    pub open_block_write_ptr: u32,
    phantom: PhantomData<&'a ()>,
}

impl<'a> Die <'a> {
    pub fn new(chl_id: u32, die_id: u32, num_blocks: u32) -> Self {
        let req_queue:Arc<Mutex<VecDeque<request::Request>>> = Arc::new(Mutex::new(VecDeque::new()));
        let completion_queue:Arc<Mutex<VecDeque<request::Request>>> = Arc::new(Mutex::new(VecDeque::new()));
        let free_block_list = Vec::new();
        let req_queue_clone = req_queue.clone();
        let completion_queue_clone = completion_queue.clone();
        let io_thread = thread::spawn(move || Die::launch_io_thread(chl_id, die_id, req_queue_clone, completion_queue_clone));

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
            phantom: PhantomData::default(),
        }
    }

    pub fn scan_free_blocks(&mut self){
        // Implement scan_free_blocks logic
    }

    pub fn write_page(&mut self, req: &request::Request) -> i32 {
        assert_eq!(req.ppa.die, self.die_id as u32);
        let mut queue = self.req_queue.lock().unwrap();
        queue.push_front(req.clone());
        0
    }

    pub fn read_page(&mut self, req: &request::Request) -> i32 {
        assert_eq!(req.ppa.die, self.die_id as u32);
        let mut queue = self.req_queue.lock().unwrap();
        queue.push_front(req.clone());
        0
    }
    // todo check why we need request in argument
    pub fn wait_for(&mut self, req: &request::Request) -> i32 {
        let mut queue = self.completion_queue.lock().unwrap();
        let ret_req = queue.pop_back();
        ret_req.unwrap().ret
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
        let req = request::Request::new1(0, 0, ppa::PPA::new(0), 0, request::END_OP);
        let mut queue = self.req_queue.lock().unwrap();
        queue.push_front(req);
        let mut io_thread = thread::spawn(|| {});
        std::mem::swap(&mut self.io_thread, &mut io_thread);
        io_thread.join().expect("Failed to join IO thread");
        0
    }
    //todo
    pub fn get_free_block_list(&self) -> &Vec<ppa::PPA> {
        &self.free_block_list
    }

    pub fn launch_io_thread(chl_id: u32, die_id: u32, req_queue: Arc<Mutex<VecDeque<request::Request>>>, completion_queue: Arc<Mutex<VecDeque<request::Request>>>) {
        while true {
            if let Some(mut req) = req_queue.try_lock().unwrap().pop_back() {
                match req.op {
                    sdf::WRITE_OP => {
                        println!(
                            "IO thread launched for chl {}, die {} write ppa {}",
                            chl_id,
                            die_id,
                            req.ppa
                        );
                        req.ret = util::write_data_ppa(0, 1, req.ppa.addr(), 1, 3, &req.buf, &req.metabuf);
                    }
                    sdf::READ_OP => {
                        println!(
                            "IO thread launched for chl {}, die {} write ppa {}",
                            chl_id,
                            die_id,
                            req.ppa
                        );
                        req.ret = util::read_data_ppa(0, 1, req.ppa.addr(), 1, 3, &req.buf, &req.metabuf);
                    }
                    sdf::END_OP => {
                        return;
                    }
                    _ => {
                        println!("invalid operation")
                    }
                }
                completion_queue.lock().unwrap().push_front(req);
            } else {
                continue;
            } 
        }
    }
}