#![allow(warnings)]
pub mod util;
pub mod sdf;
pub mod ssd;
pub mod channel;
pub mod die;
pub mod request;
pub mod ppa;
pub mod workload;

use std::rc::Rc;
use std::cell::RefCell;
use std::os::raw::{c_char, c_int, c_uint};
use std::sync::{Arc, Mutex};
use std::time::Duration;
use std::thread;
use log::{info, warn};
use wd_log::*;

use workload::Workload;


fn main() {
    set_level(wd_log::DEBUG);
    let num_channels: u32 = 4;
    let num_dies_per_chl: u32 = 2;
    let num_blocks_per_die: u32 = 1;
    // let num_vssds: u32 = 1;
    let num_threads: i32 = 2;


    log_info_ln!("Start create SSD");
    let mut ssd: ssd::SSD = ssd::SSD::new(num_channels, num_dies_per_chl, num_blocks_per_die); //todo add vssd
    ssd.get_dies();
    log_info_ln!("SSD created");    
    
    log_info_ln!("Start Workload");
    let mut workload: workload::Workload = workload::Workload::new(ssd.ssd_queue.clone(), ssd.completion_queue.clone(), num_threads);
    log_info_ln!("Start SSD Thread");
    let mut ssd_thread_handle = ssd.start_ssd_thread();
    log_info_ln!("Start Workload Thread");
    workload.start_all_thread();
    thread::sleep(Duration::from_millis(1000)); // wait all the thread start
    workload.run();
    workload.stop_all_thread();
    log_info_ln!("Workload End");
    ssd::stop_ssd_thread(ssd_thread_handle);
}

//     static mut BUF1: [c_char; 16385*256] = [0; 16385*256];
//     static mut BUF2: [c_char; 16385*256] = [0; 16385*256];
//     static mut BUF3: [c_char; 16*4*256] = [0; 16*4*256];
//     static mut BUF4: [c_char; 16*4*256] = [0; 16*4*256];

// fn main() {
//     let buf_ptr1: *mut c_char = unsafe { BUF1.as_mut_ptr() };
//     let buf_ptr2: *mut c_char = unsafe { BUF2.as_mut_ptr() };
//     let buf_ptr3: *mut c_char = unsafe { BUF3.as_mut_ptr() };
//     let buf_ptr4: *mut c_char = unsafe { BUF4.as_mut_ptr() };

//     let ret = util::write_data_ppa(0, 1, 1666, 1, 3, buf_ptr1, buf_ptr2);
//     println!("{}", ret);
//     let mut channel:u32 = 0;
//     let ret = util::read_data_ppa(0, 1, 1666, 1, 3, buf_ptr3, buf_ptr4);
//     println!("{}",ret);
    

// }



