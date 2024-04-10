use std::collections::HashMap;
use std::vec::Vec;
use crate::sdf;
use crate::channel;
use crate::die;
use crate::request;

pub struct SSD<'a> {
    channels: HashMap<u32, channel::Channel<'a>>,
    dies: Vec<&'a die::Die<'a>>,
}

impl<'a> SSD<'a> {
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
    pub fn get_dies(&'a mut self) {
        for chl in self.channels.values(){
            let new_dies: Vec<&'a die::Die<'a>> = chl.get_dies();
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

    pub fn handle_request(&mut self, req: &request::Request) -> i32 {
        match req.op {
            sdf::READ_OP => {
                println!("handling read request");
                let page_reqs = req.breakdown_into_pages();
                //todo: request ownership
                for page_req in &page_reqs {
                    self.read_page(page_req);
                }
                for page_req in &page_reqs {
                    self.wait_for(page_req);
                }
                0
            }
            sdf::WRITE_OP => {
                println!("handling write request");
                let page_reqs = req.breakdown_into_pages();
                //TODO: sort dies by their queue length
                // alloc ppas from dies
                let mut it = self.dies.iter_mut();
                for page_req in &page_reqs {
                    if let Some((&mut next_die)) = it.next() {
                        let ppa = next_die.alloc_ppa();
                        println!("lpa: {}, ppa: {}", page_req.lpa, ppa.addr());
                        page_req.ppa = ppa;
                        self.write_page(page_req);
                    } else {
                        println!("no valid die")
                    }
                }
                for page_req in &page_reqs {
                    self.wait_for(page_req);
                }
                0
            }
            sdf::END_OP => self.stop(),
            _ => -1,
        }
    }

    pub fn write_page(&mut self, req: &request::Request) -> i32 {
        assert_eq!(req.size, sdf::PAGE_SZ);
        if let Some(chl) = self.channels.get_mut(&req.ppa.chl) {
            chl.write_page(req)
        } else {
            println!("invalid die");
            -1
        }
    }

    pub fn read_page(&mut self, req: &request::Request) -> i32 {
        assert_eq!(req.size, sdf::PAGE_SZ);
        if let Some(chl) = self.channels.get_mut(&req.ppa.chl) {
            chl.write_page(req)
        } else {
            println!("invalid die");
            -1
        }
    }

    pub fn wait_for(&mut self, req: &request::Request) -> i32 {
        assert_eq!(req.size, sdf::PAGE_SZ);
        if let Some(chl) = self.channels.get_mut(&req.ppa.chl) {
            chl.write_page(req)
        } else {
            println!("invalid die");
            -1
        }
    }
}
// todo
impl<'a> Drop for SSD<'a> {
    fn drop(&mut self) {
        self.stop();
        self.channels.clear();
    }
}

