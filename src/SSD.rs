use std::collections::HashMap;
use std::vec::Vec;
use crate::sdf;
use crate::channel;
use crate::die;
use crate::request;
use std::sync::{Arc, Mutex};
use wd_log::*;


pub struct SSD{
    channels: HashMap<u32, channel::Channel>,
    dies: Vec<Arc<Mutex<die::Die>>>,
}

impl SSD {
    pub fn new(num_chls: u32, num_dies_per_chl: u32, num_blocks_per_die: u32) -> Self {
        let mut channels = HashMap::new();
        let mut dies = Vec::new();
        let mut allocated_channels = 0;

        for chl_id in 0..sdf::TOTAL_CHANNELS {
            if allocated_channels >= num_chls {
                break;
            }
            channels.insert(chl_id, channel::Channel::new(chl_id, num_dies_per_chl, num_blocks_per_die));
            
            allocated_channels += 1;
        }

        SSD { 
            channels, 
            dies, 
        }

        //todo ini write buffer
        // for i in 0..BLK_SZ {
        //     ssd.g_writebuf[i] = 'x';
        // }
        // for i in 0..BLK_SZ_META {
        //     ssd.g_metabuf[i] = 'm';
        // }
    }
    pub fn get_dies(&mut self) {
        for chl in self.channels.values_mut(){
            let new_dies: Vec<Arc<Mutex<die::Die>>> = chl.get_dies();
            self.dies.extend(new_dies);
            chl.scan_free_blocks();
        }
    }

    pub fn with_num_chls(num_chls: u32) -> Self {
        SSD::new(num_chls, 1, 1)
    }

    pub fn stop(&mut self) -> i32{
        for i in 0..self.channels.len() as u32{
            if let Some(chl) = self.channels.get_mut(&i) {
                chl.stop();
            } else {
                println!("invalid die")
            }
        }
        0
    }

    pub fn handle_request(&mut self, req: Arc<request::Request>) -> i32 {
        match req.op {
            sdf::READ_OP => {
                log_debug_ln!("ssd handling read request {}", req);
                let page_reqs = req.breakdown_into_pages();
                //todo: request ownership
                log_debug_ln!("start split read");
                for page_req in &page_reqs {
                    self.read_page(page_req);
                }
                log_debug_ln!("start waiting read");
                for page_req in &page_reqs {
                    self.wait_for(page_req);
                }
                0
            }
            sdf::WRITE_OP => {
                log_debug_ln!("ssd handling write request {}", req);
                let mut page_reqs = req.breakdown_into_pages();
                //TODO: sort dies by their queue length
                // alloc ppas from dies
                let counter = 0;
                log_debug_ln!("start split write");
                for page_req in &mut page_reqs {     
                    if let Some(channel) = self.channels.get_mut(&counter){
                        if let Some(die) = channel.dies.get_mut(&counter){
                            let mut ppa = die.lock().unwrap().alloc_ppa();
                            // log_debug_ln!("lpa: {}, ppa: {}", page_req.lpa, ppa.addr());
                            page_req.ppa = ppa;
                            self.write_page(page_req);
                        } else {
                            log_warn_ln!("no die");
                        }
                    } else {
                        log_warn_ln!("no channel");
                    }
                }
                log_debug_ln!("ssd start waiting read");
                for page_req in &page_reqs {
                    self.wait_for(page_req);
                }
                0
            }
            sdf::END_OP => {
                log_debug_ln!("ssd recieve END_OP go stop all die thread");
                self.stop();
                0
            },
            _ => -1,
        }
    }

    pub fn write_page(&mut self, req: &request::Request) -> i32 {
        assert_eq!(req.size, sdf::PAGE_SZ);
        
        if let Some(chl) = self.channels.get_mut(&req.ppa.chl) {
            log_debug_ln!("ssd write_page {}", req);
            chl.write_page(req)
        } else {
            log_warn_ln!("invalid die");
            -1
        }
    }

    pub fn read_page(&mut self, req: &request::Request) -> i32 {
        assert_eq!(req.size, sdf::PAGE_SZ);
        if let Some(chl) = self.channels.get_mut(&req.ppa.chl) {
            log_debug_ln!("ssd read_page {}", req);
            chl.read_page(req)
        } else {
            log_warn_ln!("invalid die");
            -1
        }
    }

    pub fn wait_for(&mut self, req: &request::Request) -> i32 {
        assert_eq!(req.size, sdf::PAGE_SZ);
        if let Some(chl) = self.channels.get_mut(&req.ppa.chl) {
            log_debug_ln!("ssd wait_page {}", req);
            chl.wait_for(req)
        } else {
            log_warn_ln!("invalid die");
            -1
        }
    }
}
// todo
// impl Drop for SSD {
//     fn drop(&mut self) {
//         self.stop();
//         self.channels.clear();
//     }
// }

