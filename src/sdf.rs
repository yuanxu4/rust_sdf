use crate::util;

pub const USR_DIR: &str = "/home/js39/software/rlsdf/";
pub const RESERVED_SUPERBLK:u32 = 100;
pub const PAGES_PER_BLK:u32 = 256;
pub const TOTAL_CHANNELS:u32 = 16;
pub const READ_OP:i32 = 0;
pub const WRITE_OP:i32 = 1;
pub const END_OP:i32 = -1;
pub const PAGE_SZ:u32 = util::cfg_1TB::CFG_NAND_PAGE_SIZE;
pub const BLK_SZ:u32 = PAGE_SZ*PAGES_PER_BLK;
pub const BLK_SZ_META:u32 = util::nexus::META_SIZE*4*PAGES_PER_BLK;
pub const EMULATE_SDF_ACCESS:u32 = 1;
