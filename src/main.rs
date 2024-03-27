#![allow(warnings)]
mod util;
use std::os::raw::{c_char, c_int, c_uint};

    static mut BUF1: [c_char; 16385*256] = [0; 16385*256];
    static mut BUF2: [c_char; 16*4*256] = [0; 16*4*256];

fn main() {
    let buf_ptr1: *mut c_char = unsafe { BUF1.as_mut_ptr() };
    let buf_ptr2: *mut c_char = unsafe { BUF2.as_mut_ptr() };
    let ret = util::ersppa_sync(0, 1, 1666, 1, 3);
    println!("{}", ret);
    let mut channel:u32 = 0;
    let ret = util::read_nvme_reg32(0, 0x4098c, &mut channel);
    println!("{}",ret);
    println!("{}",channel);

}
