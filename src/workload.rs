use std::thread::{self, spawn};
use std::time::{Duration, SystemTime};
use std::sync::{Arc, Mutex};
use std::collections::VecDeque;
use std::time::Instant;
use wd_log::*;

use crate::ssd;
use crate::ppa;
use crate::sdf;
use crate::request::{self, END_OP};

//delete all the reference 
pub struct Workload {
    ssd: Arc<Mutex<ssd::SSD>>,
    num_threads: i32,
    threads: Vec<Option<thread::JoinHandle<()>>>,
    reqs: VecDeque<Arc<request::Request>>,
    req_queues: Vec<Arc<Mutex<VecDeque<Arc<request::Request>>>>>,
    completion_queue: Arc<Mutex<VecDeque<Arc<request::Request>>>>,
}

impl Workload {
    pub fn new(ssd: Arc<Mutex<ssd::SSD>>, num_threads: i32) -> Self {
        let mut threads = Vec::new();
        let mut req_queues = Vec::new();
        let mut completion_queue = Arc::new(Mutex::new(VecDeque::<Arc<request::Request>>::new())); 
        let reqs = VecDeque::<Arc<request::Request>>::new();

        for thr_id in 0..num_threads {
            let req_queue = Arc::new(Mutex::new(VecDeque::<Arc<request::Request>>::new())); 
            req_queues.push(req_queue);
            let handle = None;
            threads.push(handle);
        }

        Workload {
            ssd,
            num_threads,
            threads,
            reqs,
            req_queues,
            completion_queue,
        }
    }
    pub fn start_all_thread(&mut self) {
        for i in 0..self.num_threads {
            self.start_thread(i);
        }
    }
    fn start_thread(&mut self, thread_id: i32) {
        // todo: more clever check 
        let req_queue_clone:Arc<Mutex<VecDeque<Arc<request::Request>>>> = self.req_queues[thread_id as usize].clone();
        let completion_queue_clone:Arc<Mutex<VecDeque<Arc<request::Request>>>> = self.completion_queue.clone();
        let ssd_clone = self.ssd.clone();
        if (thread_id > self.num_threads) {
            log_warn_ln!("invalid thread ID");
        } else {
            self.threads[thread_id as usize] = Some(thread::Builder::new().name(format!("workload thread, id: {}", thread_id).to_string()).spawn(move || {
                log_info_ln!("Start workload thread: {}", thread_id);
                loop {
                    if let Some(req) = req_queue_clone.lock().unwrap().pop_back() {
                        if req.op == sdf::END_OP {
                            log_info_ln!("Workload thread {} get END_OP", thread_id);
                            return;
                        }
                        ssd_clone.lock().unwrap().handle_request(req.clone());
                        completion_queue_clone.lock().unwrap().push_front(req.clone());
                        
                    } else {
                        // println!("queue is empty")
                    }
                }
                log_warn_ln!("Workload thread {} go to wrong line", thread_id);

            }).unwrap());
        }
    }

    pub fn stop_all_thread(&mut self) {
        self.stop_ssd_thread();
        for i in 0..self.num_threads {
            self.stop_workload_thread(i);
        }
    }

    fn stop_ssd_thread(&mut self){
        let req = Arc::new(request::Request::new1(0, 0, ppa::PPA::new(0), 0, request::END_OP));
        self.ssd.lock().unwrap().handle_request(req.clone());
    }
    fn stop_workload_thread(&mut self, thread_id: i32) -> i32{
        let req = Arc::new(request::Request::new1(0, 0, ppa::PPA::new(0), 0, request::END_OP));
        self.req_queues[thread_id as usize].lock().unwrap().push_front(req);
        self.threads[thread_id as usize].take().expect("Called stop on non-running thread").join().expect("Could not join spawned thread");
        0
    }
    
    pub fn run(&mut self) {
        // Request parsing
        let tracefile = "tracefile";
        self.request_parsing();

        // Request submission
        for (i, req) in self.reqs.iter().enumerate() {
            self.req_queues[i % self.num_threads as usize].lock().unwrap().push_front(req.clone());
        }
        let ret = 0;
        let start_time = Instant::now();
        loop{
            let mut queue = self.completion_queue.lock().unwrap();
            if queue.len() > 0 {
                let ret_req = queue.pop_back().unwrap();
                let ret = ret_req.ret;
                log_debug_ln!("workload completion queue get pop request {}", ret_req);
                if ret_req.id == 3 {
                    break;
                }
            }
        }

        let end_time = Instant::now();
        println!( "Time taken: {:?}", end_time.duration_since(start_time));
    }

    fn request_parsing(&mut self){ //tracefile: &str
        // TODO: Parse the tracefile and create requests
        let mut reqs_new = VecDeque::<Arc<request::Request>>::new();  
        reqs_new.push_back(Arc::new(request::Request::new(1, 0, sdf::PAGE_SZ * 4, sdf::WRITE_OP)));
        reqs_new.push_back(Arc::new(request::Request::new(2, 0, sdf::PAGE_SZ * 4, sdf::READ_OP)));
        reqs_new.push_back(Arc::new(request::Request::new(3, 0, sdf::PAGE_SZ * 4, sdf::WRITE_OP)));
        // reqs_new.push_back(Arc::new(request::Request::new(4, 0, sdf::PAGE_SZ * 4, sdf::END_OP)));
        self.reqs = reqs_new;
    }
    
    
}

// fn launch_io_thread(mut ssd: Arc<ssd::SSD>, thr_id: i32, req_queue: Arc<Mutex<VecDeque<&request::Request>>>, comple_queue: Arc<Mutex<VecDeque<&request::Request>>>) {
//     loop {
//         if let Some(req) = req_queue.lock().unwrap().pop_back() {
//             if req.op == sdf::END_OP /* END_OP */ {
//                 return;
//             }
//             ssd.handle_request(req);
//             comple_queue.lock().unwrap().push_front(req);
//             // Placeholder enqueue into completion_queue
//         } else {
//             println!("queue is empty")
//             // Handle queue being empty
//             // You might want to introduce a way to gracefully handle this
//         }
//     }
// }

