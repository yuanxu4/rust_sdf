use std::thread;
use std::time::{Duration, SystemTime};
use std::sync::{Arc, Mutex};
use std::collections::VecDeque;
use std::time::Instant;

use crate::ssd;
use crate::sdf;
use crate::request;

pub struct Workload<'a> {
    ssd: ssd::SSD<'a>,
    num_threads: i32,
    threads: Vec<thread::JoinHandle<()>>,
    reqs: VecDeque<request::Request>,
    req_queues: Vec<Arc<Mutex<VecDeque<&'a request::Request>>>>,
    completion_queue: Arc<Mutex<VecDeque<&'a request::Request>>>,
}

impl<'a> Workload<'a> {
    pub fn new(ssd: ssd::SSD, num_threads: i32) -> Self {
        let mut threads = Vec::new();
        let mut req_queues = Vec::new();
        let mut completion_queue = Arc::new(Mutex::new(VecDeque::<&request::Request>::new())); 
        let reqs = VecDeque::<request::Request>::new();

        for thr_id in 0..num_threads {
            let req_queue = Arc::new(Mutex::new(VecDeque::<&request::Request>::new())); 
            req_queues.push(req_queue);
            let handle = thread::spawn(move || Workload::launch_io_thread(&ssd, thr_id, req_queues[thr_id as usize].clone(), completion_queue.clone()));
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

    pub fn run(&self) {
        println!("Running workload");

        // Request parsing
        let tracefile = "tracefile";
        self.request_parsing();

        // Request submission
        for (i, req) in self.reqs.iter().enumerate() {
            self.req_queues[i % self.num_threads as usize].lock().unwrap().push_front(req);
        }

        let start_time = Instant::now();

        // Dequeueing requests
        let _ = self.completion_queue.lock().unwrap().pop_back();
        let _ = self.completion_queue.lock().unwrap().pop_back();

        let end_time = Instant::now();

        println!(
            "Time taken: {:?}",
            end_time.duration_since(start_time)
        );

        // Printing ret value of the last dequeued request
        if let Some(req) = self.completion_queue.lock().unwrap().pop_back() {
            println!("ret: {}", req.ret);
        }
    }

    fn launch_io_thread(ssd: &ssd::SSD, thr_id: i32, req_queue: Arc<Mutex<VecDeque<&request::Request>>>, comple_queue: Arc<Mutex<VecDeque<&request::Request>>>) {
        loop {
            if let Some(req) = req_queue.lock().unwrap().pop_back() {
                if req.op == sdf::END_OP /* END_OP */ {
                    return;
                }
                ssd.handle_request(req);
                comple_queue.lock().unwrap().push_front(req);
                // Placeholder enqueue into completion_queue
            } else {
                println!("queue is empty")
                // Handle queue being empty
                // You might want to introduce a way to gracefully handle this
            }
        }
    }
    fn request_parsing(&mut self){ //tracefile: &str
        // TODO: Parse the tracefile and create requests
        let mut reqs_new = VecDeque::new();    
        let req = request::Request::new(1, 0, sdf::PAGE_SZ * 4, sdf::WRITE_OP);
        reqs_new.push_back(req);
        let req = request::Request::new(2, 0, sdf::PAGE_SZ * 4, sdf::READ_OP);
        reqs_new.push_back(req);
        let req = request::Request::new(3,sdf::PAGE_SZ * 4, sdf::PAGE_SZ * 4, sdf::WRITE_OP);
        reqs_new.push_back(req);
        self.reqs = reqs_new;
    }
    
    
}

