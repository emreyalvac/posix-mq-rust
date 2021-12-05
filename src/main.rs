mod mq;

use std::ffi::CString;
use std::os::raw::{c_char, c_int, c_long, c_uint};
use mq::*;

fn build_mq_attr(mq_flags: c_long, mq_curmsgs: c_long) -> MqAttr {
    MqAttr {
        mq_flags,
        mq_curmsgs,
        mq_maxmsg: MAX_MESSAGES,
        mq_msgsize: MAX_MSG_SIZE,
    }
}

fn send_simulation(queue_fd: c_int) -> c_int {
    unsafe {
        let data = b"Emrasdasdae".as_ptr();
        mq_send(queue_fd, data as *const c_char, 4, 10)
    }
}

fn main() {
    let queue_fd = unsafe {
        let server_queue_name = CString::new(SERVER_QUEUE_NAME).unwrap();
        mq_open(server_queue_name.as_ptr(), O_CREAT | O_RDWR, QUEUE_PERMISSIONS as u32, &build_mq_attr(0, 0))
    };

    if queue_fd < 0 {
        panic!("queue create");
    };

    std::thread::spawn(move || {
        loop {
            send_simulation(queue_fd);
            std::thread::sleep(std::time::Duration::from_millis(1000));
        }
    });

    loop {
        let mut buffer = [0u8; MAX_MSG_SIZE as usize];
        unsafe {
            mq_receive(queue_fd, buffer.as_ptr() as *const c_char, buffer.len(), std::ptr::null_mut())
        };

        println!("Receive -> {:?}", String::from_utf8(Vec::from(buffer)));
    }
}