use std::collections::HashMap;
use crate::request;
use crate::die;

//??
use std::marker::PhantomData;

pub struct Channel<'a>{
    chl_id: u32,
    num_dies: u32,
    dies: HashMap<u32, die::Die<'a>>,
    phantom: PhantomData<&'a ()>,
    // std::vector<u32> free_block_list;
}

impl<'a> Channel<'a>{
    pub fn new(chl_id: u32, num_dies: u32, num_blocks_per_die: u32) -> Self {
        let mut dies = HashMap::new();
        for die_id in 0..num_dies {
            let die = die::Die::new(chl_id, die_id, num_blocks_per_die);
            dies.insert(die_id, die);
        }
        Self {
            chl_id,
            num_dies,
            dies,
            phantom: PhantomData::default(),
        }
    }

    pub fn get_dies(&'a self) -> Vec<&'a die::Die<'a>> {
        self.dies.values().collect()
    }

    //todo
    pub fn scan_free_blocks(&self) {
    }
    
    pub fn write_page(&mut self, req: &request::Request) -> i32 {
        assert_eq!(req.ppa.chl, self.chl_id as u32);
        if let Some(die) = self.dies.get_mut(&req.ppa.die) {
            die.write_page(req);
        } else {
            println!("invalid die")
        }
        0
    }
    
    pub fn read_page(&mut self, req: &request::Request) -> i32 {
        assert_eq!(req.ppa.chl, self.chl_id as u32);
        if let Some(die) = self.dies.get_mut(&req.ppa.die) {
            die.read_page(req);
        } else {
            println!("invalid die")
        }
        0
    }
    
    pub fn wait_for(&mut self, req: &request::Request) -> i32 {
        assert_eq!(req.ppa.chl, self.chl_id as u32);
        if let Some(die) = self.dies.get_mut(&req.ppa.die) {
            die.wait_for(req);
        } else {
            println!("invalid die")
        }
        0
    }
    
    pub fn stop(&mut self) -> i32 {
        for i in 0..self.num_dies {
            if let Some(die) = self.dies.get_mut(&i) {
                die.stop();
            } else {
                println!("invalid die")
            }
        }
        0
    }
}