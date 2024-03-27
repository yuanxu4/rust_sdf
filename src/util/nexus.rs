use crate::util::cfg_1TB;

pub const NEXUS_IOCTL_PPA_SYNC: u32 = (0x80000000 | 0x40000000) | ((std::mem::size_of::<NvmePpaCommand>() as u32 & 0x1fff) << 16) | (('N' as u32) << 8) | (0x40);
pub const NEXUS_IOCTL_RD_REG:u32    = ((2_u32 | 1_u32) << 30) | (('N' as u32) << (8))| (0x80_u32 << 0)| ((std::mem::size_of::<NvmeReadMemory>() as u32) << 16);

pub const CH_INC: u16 =  0;
pub const EP_INC: u16 =  1;
pub const PL_INC: u16 =  2;
pub const ADDR_FIELDS_SHIFT_CH: u16 = 0; 
pub const ADDR_FIELDS_SHIFT_EP: u16 = cfg_1TB::CH_BITS;
pub const NEXUS_DEV: &str = "/dev/nexus0";

const META_SIZE: u32 = 16;
const META_RAWSIZE: u32 = 256 + 48; // 304
const PAGE_SIZE: u32 = 0x1000;
const BLOCK_SIZE: u32 = 0x1000;
const CQE_SIZE: u32 = 16;
const SQE_SIZE: u32 = 64;
const MAX_Q_DEPTH: u32 = 10240;

const GOOD_PPA: u32 =  1;
const BAD_PPA: u32 =  0;

#[repr(C)] 
pub struct NvmePpaCommand {
    pub opcode:     u8,
    pub flags:      u8,
    pub command_id: u16,
    pub nsid:       u32,     
    pub cdw2:       [u32; 2], 
    pub metadata:   u64, 
    pub prp1:       u64,
    pub prp2:       u64,
    pub start_list: u64,
    pub nlb:        u16,       
    pub control:    u16,   
    pub dsmgmt:     u32,    
    pub reftag:     u32,    
    pub apptag:     u16,    
    pub appmask:    u16,   
}

#[repr(C)] 
pub struct RdmemStru {
    pub mem_addr: u32,
    pub length: u32,
    pub pdata: *mut u32, 
}

#[repr(C)] 
pub struct NvmeReadMemory {
    pub mem_addr: u32,
    pub length: u32,
    pub pdata: *mut u32, 
}

#[repr(u8)]
pub enum NexusOpcode {
    NvmeCmdFlush = 0x00,
    NvmeCmdWrite = 0x01,
    NvmeCmdRead = 0x02,
    NvmeCmdWriteUncor = 0x04,
    NvmeCmdCompare = 0x05,
    NvmeCmdDsm = 0x09,
    NvmeCmdRdlbatoecpu = 0x82,
    NvmeCmdRdppatoecpu = 0x86,
    NvmeCmdErsppa = 0x90,
    NvmeCmdWrppa = 0x91,
    NvmeCmdRdppa = 0x92,
    NvmeCmdDealloc = 0x94,
    NvmeCmdWrpparaw = 0x95,
    NvmeCmdRdpparaw = 0x96,
    NvmeCmdWrmem = 0x99,
    NvmeCmdRdmem = 0x9a,
    NvmeCmdWrraid = 0x9d,
    NvmeCmdLoadraid = 0x9e,
    NvmeCmdWrxordata = 0xa1,
    NvmeCmdRdraid = 0xa2,
    NvmeCmdRdpparaid = 0xb2,
    NvmeCmdRdppatomem = 0xb6,
    NvmeCmdSvvpc = 0xd1,
    NvmeCmdLdvpc = 0xd2,
    NvmeCmdManuinit = 0xe0,
    NvmeCmdPwron = 0xe4,
    NvmeCmdPwrdwn = 0xe8,
    NvmeCmdPwrdwnf = 0xec,
    NvmeCmdLdftl = 0xea,
    NvmeCmdWrlbamem = 0xed,
    NvmeCmdRdlbamem = 0xee,
    NvmeCmdWrppamem = 0xf5,
    NvmeCmdRdppamem = 0xf6,
    NvmeCmdLdraid = 0xfe,
}
