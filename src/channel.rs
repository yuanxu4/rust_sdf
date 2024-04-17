use std::collections::HashMap;
use wd_log::log_warn_ln;

use crate::request;
use crate::die;
use std::sync::{Arc, Mutex};

pub struct Channel{
    pub chl_id: u32,
    pub num_dies: u32,
    pub dies: HashMap<u32, Arc<Mutex<die::Die>>>,
    // std::vector<u32> free_block_list;
}

impl Channel{
    pub fn new(chl_id: u32, num_dies: u32, num_blocks_per_die: u32) -> Self {
        let mut dies = HashMap::new();
        for die_id in 0..num_dies {
            let die = Arc::new(Mutex::new(die::Die::new(chl_id, die_id, num_blocks_per_die)));
            dies.insert(die_id, die);
        }
        Self {
            chl_id,
            num_dies,
            dies,
        }
    }

    pub fn get_dies(&self) -> Vec<Arc<Mutex<die::Die>>> {
        self.dies.values().map(|die| die.clone()).collect()
    }
    // u16* read_badbin(){

    // }
    //todo
    pub fn scan_free_blocks(&mut self) {
        for die_id in 0..self.num_dies {
            self.dies[&die_id].lock().unwrap().scan_free_blocks();
        }
    }
    
    pub fn write_page(&mut self, req: &request::Request) -> i32 {
        assert_eq!(req.ppa.chl, self.chl_id as u32);
        if let Some(die) = self.dies.get_mut(&req.ppa.die) {
            die.lock().unwrap().write_page(req);
        } else {
            log_warn_ln!("invalid die");
        }
        0
    }
    
    pub fn read_page(&mut self, req: &request::Request) -> i32 {
        assert_eq!(req.ppa.chl, self.chl_id as u32);
        if let Some(die) = self.dies.get_mut(&req.ppa.die) {
            die.lock().unwrap().read_page(req);
        } else {
            log_warn_ln!("invalid die");
        }
        0
    }
    
    pub fn wait_for(&mut self, req: &request::Request) -> i32 {
        assert_eq!(req.ppa.chl, self.chl_id as u32);
        if let Some(die) = self.dies.get_mut(&req.ppa.die) {
            die.lock().unwrap().wait_for(req);
        } else {
            log_warn_ln!("invalid die")
        }
        0
    }
    
    pub fn stop(&mut self) -> i32 {
        for i in 0..self.num_dies {
            if let Some(die) = self.dies.get_mut(&i) {
                die.lock().unwrap().stop();
            } else {
                log_warn_ln!("invalid die")
            }
        }
        0
    }
}