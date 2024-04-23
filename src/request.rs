use crate::sdf;
use crate::ppa;
use std::os::raw::c_char;
use log::{info, warn, debug};
use wd_log::log_debug_ln;
use std::fmt;


pub const READ_OP:i32 = 0;
pub const WRITE_OP:i32 = 1;
pub const END_OP:i32 = -1;
#[derive(Clone, Debug)]
pub struct Request {
    pub id: u32,
    pub lpa: u32,
    pub ppa: ppa::PPA,
    pub size: u32,
    pub ret: i32,
    pub op: i32,
    pub buf: Option<Vec<u8>>,
    pub metabuf: Option<Vec<u8>>,
}
//TODO: change the Option to Box

//TODO
impl Request {
    pub fn new(id: u32, offset: u32, size: u32, op: i32) -> Self {
        Request {
            id,
            lpa: offset/sdf::PAGE_SZ,
            ppa:ppa::PPA::new(0),
            size,
            ret: 0,
            op,
            buf:None,
            metabuf:None,
        }
    }

    pub fn new1(id: u32, lpa: u32, ppa: ppa::PPA, size: u32, op: i32) -> Self {
        let (buf, metabuf) = match op {
            WRITE_OP => (Some(vec!['x' as u8; sdf::BLK_SZ as usize]), Some(vec!['m' as u8; sdf::BLK_SZ_META as usize])),
            READ_OP => (Some(vec![0; sdf::BLK_SZ as usize]), Some(vec![0; sdf::BLK_SZ_META as usize])),
            END_OP => (None, None),
            _ => panic!("Invalid OP"),
        };
        Request {
            id,
            lpa,
            ppa,
            size,
            ret: 0,
            op,
            buf,
            metabuf,
        }
    }

    pub fn breakdown_into_pages(&self) -> Vec<Request> {
        
        let mut pages = Vec::new();
        log_debug_ln!("Start break request{}", self);
        for i in 0..(self.size / sdf::PAGE_SZ) {
            let page = Request::new1(
                self.id,
                self.lpa + sdf::PAGE_SZ,
                ppa::PPA::new2(
                    self.ppa.chl,
                    self.ppa.die,
                    self.ppa.pl,
                    self.ppa.blk,
                    self.ppa.pg + i,
                ),
                sdf::PAGE_SZ,
                self.op,
            );
            // log_debug_ln!("break to new reqeust { }", page);
            pages.push(page);
        }
        pages
    }
}

impl fmt::Display for Request {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.op == READ_OP {        
            write!(f, "id: {}, lpa: {}, ppa: [{}], size: {}, ret: {}, op: READ_OP", 
                self.id, self.lpa, self.ppa, self.size, self.ret)
        } else if self.op == WRITE_OP{
            write!(f, "id: {}, lpa: {}, ppa: [{}], size: {}, ret: {}, op: WRITE_OP", 
                self.id, self.lpa, self.ppa, self.size, self.ret)
        } else {
            write!(f, "id: {}, lpa: {}, ppa: [{}], size: {}, ret: {}, op: END_OP", 
                self.id, self.lpa, self.ppa, self.size, self.ret)
        }
    }
}

