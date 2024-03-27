
#![allow(non_snake_case)]

pub mod nexus;
pub mod cfg_1TB;
use std::ffi::CString;
use std::ptr;

use std::os::raw::{c_char, c_int, c_uint};
use log;
use sdf::EMULATE_SDF_ACCESS;
// use libc;

//TODO:test
pub fn str_concat(s1: &str, s2: &str) -> String {
    let mut result = String::with_capacity(s1.len() + s2.len());
    result.push_str(s1);
    result.push_str(s2);
    result
}

//TODO:test
pub fn  parse_read_file(filename: *const c_char, file_len: &mut u32) -> *mut c_void {
    let mut fd: i32 = -1;
    let mut memaddr: *mut c_void = ptr::null_mut();
    let mut MAX_SIZE: u32 = (nexus::PAGE_SIZE * 1024) as u32; 

    let mut filestat: stat = unsafe { mem::zeroed() };

    unsafe {
        if stat(filename, &mut filestat) < 0 || MAX_SIZE  < filestat.st_size as u32 {
            libc::perror("stat\0".as_ptr() as *const c_char);
            ptr::null_mut()
        } else {
            *file_len = filestat.st_size as u32;
        }
    }

    unsafe {
        memaddr = libc::malloc(*file_len as usize) as *mut c_void;
        if memaddr.is_null() {
            perror("malloc\0".as_ptr() as *const c_char);
            ptr::null_mut()
        }
    }

    fd = unsafe { libc::open(filename, O_RDONLY) };
    if fd < 0 {
        libc::perror("open fd\0".as_ptr() as *const c_char);
        unsafe { libc::free(memaddr) };
        ptr::null_mut()
    }

    // Read file
    let mut bytes_read: u32 = 0;
    unsafe {
        bytes_read = libc::read(fd, memaddr, *file_len as usize);
        if bytes_read != *file_len {
            libc::perror("read fd\0".as_ptr() as *const c_char);
            libc::free(memaddr);
            libc::close(fd);
            ptr::null_mut()
        }
    }
    unsafe {
        libc::close(fd);
    }
    memaddr
}

pub fn read_nvme_reg32(devid: u32, offset: u32, regval: *mut u32) -> c_int{
    if EMULATE_SDF_ACCESS == 1 {
        log::debug!("read_nvme_reg32");
        return 0;
    }
    let mut ret: i32 = 0;
    let mut fd: i32 = 0;
    let mut cmd: u32 = nexus::NEXUS_IOCTL_RD_REG;
    let mut cmd_para: nexus::RdmemStru = 
        nexus::RdmemStru { 
        mem_addr: offset,
        length: std::mem::size_of::<u32>() as u32,
        pdata: regval,};
    let filename = CString::new(nexus::NEXUS_DEV).expect("CString::new failed");
    unsafe {
        fd = libc::open(filename.as_ptr() as *const c_char, libc::O_RDWR);
        if fd < 0 {
            libc::perror(b"open nexus0\0".as_ptr() as *const i8);
            ret = -1;
            return ret;
        }
        ret = libc::ioctl(fd, cmd.into(), &mut cmd_para);
        if ret < 0 {
            libc::perror(b"ioctl\0".as_ptr() as *const i8);
        }
        libc::close(fd);
    }
    ret
}

pub fn write_data_ppa(devid: u32, nsid: u32, ppa: u32, qid: u16, nlb: u32, buf: *mut c_char, metabuf: *mut c_char) -> c_int {
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
    let filename = CString::new(nexus::NEXUS_DEV).expect("CString::new failed");
    unsafe {
        fd = libc::open(filename.as_ptr() as *const c_char, libc::O_RDWR);
        if fd < 0 {
            libc::perror(b"open nexus0\0".as_ptr() as *const i8);
            ret = -1;
            return ret;
        }
        ret = libc::ioctl(fd, cmd.into(), &mut cmd_para);
        libc::close(fd);
    }
    ret
}

pub fn read_data_ppa(devid: u32, nsid: u32, ppa: u32, qid: u16, nlb: u32, buf: *const c_char, metabuf: *const c_char) -> c_int{
    if EMULATE_SDF_ACCESS == 1 {
        log::debug!("read_data_ppa");
        return 0;
    }
    let mut ret:i32 = 0;
    let mut fd:i32  = 0;
    let mut cmd:u32  = nexus::NEXUS_IOCTL_PPA_SYNC;
    let mut cmd_para: nexus::NvmePpaCommand = 
        nexus::NvmePpaCommand { 
        opcode:     nexus::NexusOpcode::NvmeCmdRdppa as u8,
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
    


        let filename = CString::new(nexus::NEXUS_DEV).expect("CString::new failed");
        unsafe {
            fd = libc::open(filename.as_ptr() as *const c_char, libc::O_RDWR);
            if fd < 0 {
                libc::perror(b"open nexus0\0".as_ptr() as *const i8);
                ret = -1;
                return ret;
            }
            ret = libc::ioctl(fd, cmd.into(), &mut cmd_para);
            if ret < 0 {
                libc::perror(b"ioctl\0".as_ptr() as *const i8);
            }
            libc::close(fd);
        }
        ret
}

pub fn ersppa_sync(devid: u32, nsid: u32, ppa_addr: u32, qid: u16, nlb: u32) -> c_int {
    if EMULATE_SDF_ACCESS == 1 {
        log::debug!("ersppa_sync");
        return 0;
    }
    let mut ret: i32 = 0;
    let mut fd: i32 = 0;
    let mut cmd: u32 = nexus::NEXUS_IOCTL_PPA_SYNC;
    let mut cmd_para: nexus::NvmePpaCommand = 
        nexus::NvmePpaCommand { 
        opcode:     nexus::NexusOpcode::NvmeCmdErsppa as u8,
        flags:      0,
        command_id: 0,
        nsid:       nsid,     
        cdw2:       [0,0], 
        metadata:   0, 
        prp1:       0,
        prp2:       0,
        start_list: ppa_addr as u64,
        nlb:        nlb as u16,       
        control:    cfg_1TB::NVME_SINGLE_PLANE,   
        dsmgmt:     0,    
        reftag:     0,    
        apptag:     qid,    
        appmask:    nexus::ADDR_FIELDS_SHIFT_EP,};
    
        let filename = CString::new(nexus::NEXUS_DEV).expect("CString::new failed");
        unsafe {
            fd = libc::open(filename.as_ptr() as *const c_char, libc::O_RDWR);
            if fd < 0 {
                libc::perror(b"open nexus0\0".as_ptr() as *const i8);
                ret = -1;
                return ret;
            }
            ret = libc::ioctl(fd, cmd.into(), &mut cmd_para);
            if ret < 0 {
                libc::perror(b"ioctl\0".as_ptr() as *const i8);
            }
            libc::close(fd);
        }
        ret
}
//TODO:test
pub fn skip_maskchannel(ppa_addr: u32, channel_mask: u16) -> u32 {
    let mut status:u32 = nexus::GOOD_PPA;
    let mut ch:u16 =  (ppa_addr & 0xf0000000) >> 28 as u16;
    let mark:u16 = 1 << ch;
    if(mark & channel_mask) != 0{
        status = GOOD_PPA;
    } else {
        status = BAD_PPA;
    }
    status
}
//TODO:test
pub fn skip_badblk(ppa_addr: u32, badbin: &u16) -> u32 {
    let mut status:u32 = nexus::GOOD_PPA;
    let ch:u16 =  (ppa_addr & 0xf0000000) >> 28 as u16;
    let lun:u16 =  (ppa_addr & 0x00c00000) >> 22  as u16;
    let blk:u16 =  (ppa_addr & 0x00003ffe) >> 1 as u16;
    let mark:u16 = 1 << ch;
    
    let let Some(badmark) = badbin.get((CFG_NAND_BLOCK_NUM - 1 - blk as usize) * LUN_NUM + lun as usize) {
        if (*badmark & mark) != 0 {
            status = BAD_PPA;
        }
    }
    status
}

//TODO:test
pub fn skip_ppa(ppa_addr: u32, badbin: &u16, channel_mask: u16) -> u32 {
    let mut flag:u32 = nexus::GOOD_PPA;
    
    flag = skip_badblk(ppa_addr, badbin);
    if (flag == nexus::BAD_PPA) {
        flag
    }
    
    flag = skip_maskchannel(ppa_addr, channel_mask);
    if (flag == nexus::BAD_PPA) {
        flag
    }
    flag
}
