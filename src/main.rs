#![allow(warnings)]
pub mod util;
pub mod sdf;
pub mod ssd;
pub mod channel;
pub mod die;
pub mod request;
pub mod ppa;
pub mod workload;
use std::os::raw::{c_char, c_int, c_uint};
use std::sync::{Arc, Mutex};
use std::time::Duration;
use std::thread;
use log::{info, warn};
use wd_log::*;

use workload::Workload;

fn main() {
    set_level(wd_log::DEBUG);
    let num_channels: u32 = 1;
    let num_dies_per_chl: u32 = 1;
    let num_blocks_per_die: u32 = 1;
    // let num_vssds: u32 = 1;
    let num_threads: i32 = 1;
    log_info_ln!("Start create SSD");
    let mut ssd: Arc<Mutex<ssd::SSD>> = Arc::new(Mutex::new(ssd::SSD::new(num_channels, num_dies_per_chl, num_blocks_per_die))); //todo add vssd
    ssd.lock().unwrap().get_dies();
    log_info_ln!("SSD created");
    // Create a workload with SSD and number of threads
    log_info_ln!("Start Workload");
    let mut workload: workload::Workload = workload::Workload::new(ssd, num_threads);
    workload.start_thread(0);
    thread::sleep(Duration::from_millis(100));
    // Run the workload
    workload.run();
    workload.stop_thread(0);
    log_info_ln!("Workload End");
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



