
#![allow(non_snake_case)]

pub mod nexus;
pub mod cfg_1TB;

use std::os::raw::{c_char, c_int, c_uint};
use log;
// use libc;

const EMULATE_SDF_ACCESS: i32 = 1;

pub fn str_concat(s1: &str, s2: &str) -> String {
    let mut result = String::with_capacity(s1.len() + s2.len());
    result.push_str(s1);
    result.push_str(s2);
    result
}

pub fn  parse_read_file(filename: *const c_char, file_len: *mut c_uint) -> c_int {
    if EMULATE_SDF_ACCESS == 1 {
        log::debug!("parse_read_file");
        return 0;
    }
    0
}

pub fn read_nvme_reg32(devid: u32, offset: u32, regval: &mut u32) -> c_int{
    if EMULATE_SDF_ACCESS == 1 {
        log::debug!("read_nvme_reg32");
        return 0;
    }
    0
}
pub fn write_data_ppa(devid: u32, nsid: u32, ppa: u32, qid: u16, nlb: u32, buf: *const c_char, metabuf: *const c_char) -> c_int {
    if EMULATE_SDF_ACCESS == 1 {
        log::debug!("write_data_ppa");
        return 0;
    }
    let mut ret: i32 = 0;
    let mut fd: i32 = 0;
    let mut cmd: u32 = nexus::NEXUS_IOCTL_PPA_SYNC;
    let mut cmd_para: nexus::NvmePpaCommand = 
                nexus::NvmePpaCommand { 
                opcode:     nexus::NexusOpcode::NvmeCmdWrppa as u8,
                flags:      0,
                command_id: 0,
                nsid:       nsid,     
                cdw2:       [0,0], 
                metadata:   metabuf as u64, 
                prp1:       buf as u64,
                prp2:       0,
                start_list: ppa as u64,
                nlb:        nlb as u16,       
                control:    cfg_1TB::NVME_SINGLE_PLANE,   
                dsmgmt:     0,    
                reftag:     0,    
                apptag:     qid,    
                appmask:    nexus::ADDR_FIELDS_SHIFT_EP,};
    
    // unsafe {
    //     fd = libc::open(name.as_ptr(), libc::O_RDWR);
    //     if fd < 0 {
    //         libc::perror(b"open nexus0\0".as_ptr() as *const i8);
    //         ret = libc::ERROR;
    //         return ret;
    //     }

    //     ret = libc::ioctl(fd, cmd, &mut cmd_para);
    //     libc::close(fd);
    // }

    ret
}

pub fn read_data_ppa(devid: u32, nsid: u32, ppa: u32, qid: u16, nlb: u32, buf: *const c_char, metabuf: *const c_char) -> c_int{
    if EMULATE_SDF_ACCESS == 1 {
        log::debug!("read_data_ppa");
        return 0;
    }
    0
}

pub fn ersppa_sync(devid: u32, nsid: u32, ppa_addr: u32, qid: u16, nlb: u32) -> c_int {
    if EMULATE_SDF_ACCESS == 1 {
        log::debug!("ersppa_sync");
        return 0;
    }
    0
}

pub fn skip_maskchannel(ppa_addr: u32, channel_mask: u16) -> c_int {
    if EMULATE_SDF_ACCESS == 1 {
        log::debug!("skip_maskchannel");
        return 0;
    }
    0
}

pub fn skip_badblk(ppa_addr: u32, badbin: &mut u16) -> c_int {
    if EMULATE_SDF_ACCESS == 1 {
        log::debug!("skip_badblk");
        return 0;
    }
    0
}

pub fn skip_ppa(ppa_addr: u32, badbin: &mut u16, channel_mask: u16) -> c_int {
    if EMULATE_SDF_ACCESS == 1 {
        log::debug!("skip_ppa");
        return 0;
    }
    0
}
