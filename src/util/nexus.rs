use crate::util::cfg_1TB;

pub const NEXUS_IOCTL_PPA_SYNC: u32 = (0x80000000 | 0x40000000) | ((std::mem::size_of::<NvmePpaCommand>() as u32 & 0x1fff) << 16) | (('N' as u32) << 8) | (0x40);

pub const CH_INC: u16 =  0;
pub const EP_INC: u16 =  1;
pub const PL_INC: u16 =  2;
pub const ADDR_FIELDS_SHIFT_CH: u16 = 0; 
pub const ADDR_FIELDS_SHIFT_EP: u16 = cfg_1TB::CH_BITS;
pub const NEXUS_DEV: &str = "/dev/nexus";

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
