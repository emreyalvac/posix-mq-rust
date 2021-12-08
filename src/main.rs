#![allow(warnings)]

mod mq;
mod handler;

use std::ffi::CString;
use std::os::raw::{c_char, c_int};
use std::sync::{Arc, Mutex};
use mq::*;
use handler::*;

fn send_simulation(queue_fd: c_int) -> c_int {
    unsafe {
        let data = b"Emrasdasdae".as_ptr();
        mq_send(queue_fd, data as *const c_char, 4, 10)
    }
}

pub struct Handler {}

impl THandler for Handler {
    fn handle_queue_event(&self, buffer: *const c_char) -> () {
        println!("{:?}", buffer);
    }
}

fn main() {
    let handler = Handler {};
    let options = Options::new().read_n_write();
    let mut posix_mq = PosixMQ::new(options, handler);

    posix_mq.create_queue(String::from("/mq_instance_1")).expect("mq_open failed");
    posix_mq.receive_from_queue();
}