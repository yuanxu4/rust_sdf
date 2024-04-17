use crate::util::cfg_1TB::*;
use std::fmt;
#[derive(Clone, Copy, Debug)]
pub struct PPA {
    pub chl: u32,
    pub die: u32,
    pub pl: u32,
    pub blk: u32,
    pub pg: u32,
}


impl PPA {
    pub fn new(ppa: u32) -> Self {
        let chl = ppa & ((1 << CH_BITS) - 1);
        let die = (ppa >> (CH_BITS + EP_BITS + PL_BITS)) & ((1 << LN_BITS) - 1);
        let pl = (ppa >> (CH_BITS + EP_BITS)) & ((1 << PL_BITS) - 1);
        let blk = (ppa >> (CH_BITS + EP_BITS + PL_BITS + LN_BITS + PG_BITS)) & ((1 << BL_BITS) - 1);
        let pg = (ppa >> (CH_BITS + EP_BITS + PL_BITS + LN_BITS)) & ((1 << PG_BITS) - 1);

        Self { chl, die, pl, blk, pg }
    }

    pub fn new2(chl: u32, die: u32, pl: u32, blk: u32, pg: u32) -> Self {
        Self { chl, die, pl, blk, pg }
    }

    pub fn addr(&self) -> u32 {
        self.chl |
        (self.die << (CH_BITS + EP_BITS + PL_BITS)) |
        (self.pl << (CH_BITS + EP_BITS)) |
        (self.blk << (CH_BITS + EP_BITS + PL_BITS + LN_BITS + PG_BITS)) |
        (self.pg << (CH_BITS + EP_BITS + PL_BITS + LN_BITS))
    }

    pub fn is_valid(&self) -> bool{
        return self.chl >= 0 && self.die >= 0 && self.pl >= 0 && self.blk >= 0 && self.pg >= 0;
    }

    

    pub fn to_string(&self) -> String {
        format!(
            "<Chl: {}, Die: {}, Pl: {}, Blk: {}, Pg: {}, PPA: {}>",
            self.chl, self.die, self.pl, self.blk, self.pg, self.addr()
        )
    }
}

impl fmt::Display for PPA {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "chl: {}, die: {}, pl: {}, blk: {}, pg: {}",
            self.chl, self.die, self.pl, self.blk, self.pg
        )
    }
}