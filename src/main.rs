use std::ffi::CString;
use std::os::raw::{c_char, c_int, c_long, c_uint};

const MAX_MESSAGES: c_long = 10;
const MAX_MSG_SIZE: c_long = 1024;
const QUEUE_PERMISSIONS: c_int = 0600;
const SERVER_QUEUE_NAME: &str = "/mqtest.pwPa";

mod ffi {
    use std::os::raw::{c_char, c_int, c_long, c_uint};

    pub const O_CREAT: c_int = 64;
    pub const O_EXCL: c_int = 128;
    pub const O_WRONLY: c_int = 1;
    pub const O_RDWR: c_int = 2;

    pub const O_NONBLOCK: c_int = 2048;

    pub type mode_t = u32;

    #[repr(C)]
    pub struct MqAttr {
        pub mq_flags: c_long,
        pub mq_maxmsg: c_long,
        pub mq_msgsize: c_long,
        pub mq_curmsgs: c_long,
    }

    #[link(name = "c")]
    extern "C" {
        pub fn mq_open(server_name: *const c_char, o_flag: c_int, mode_t: mode_t, ...) -> c_int;
        pub fn mq_send(mqdes: c_int, buffer: *const c_char, buffer_size: c_int, prio: c_uint) -> c_int;
        pub fn mq_receive(mqdes: c_int, buffer: *const c_char, message_size: usize, prio: *const c_uint) -> c_int;
        pub fn mq_unlink(name: *const c_char) -> c_int;
        pub fn mq_close(mqd_t: c_int) -> c_int;
    }
}

fn build_mq_attr(mq_flags: c_long, mq_curmsgs: c_long) -> ffi::MqAttr {
    ffi::MqAttr {
        mq_flags,
        mq_curmsgs,
        mq_maxmsg: MAX_MESSAGES,
        mq_msgsize: MAX_MSG_SIZE,
    }
}

fn send_simulation(queue_fd: c_int) -> c_int {
    unsafe {
        let data = b"Emre".as_ptr();
        ffi::mq_send(queue_fd, data as *const c_char, 5, 10)
    }
}

fn main() {
    unsafe {
        let server_queue_name = CString::new(SERVER_QUEUE_NAME).unwrap();
        ffi::mq_unlink(server_queue_name.as_ptr())
    };

    let queue_fd = unsafe {
        let server_queue_name = CString::new(SERVER_QUEUE_NAME).unwrap();
        ffi::mq_open(server_queue_name.as_ptr(), ffi::O_CREAT | ffi::O_RDWR, QUEUE_PERMISSIONS as ffi::mode_t, &build_mq_attr(0, 0))
    };

    if queue_fd < 0 {
        panic!("queue create");
    };

    // std::thread::spawn(move || {
    //     loop {
    //         send_simulation(queue_fd);
    //         std::thread::sleep(std::time::Duration::from_secs(2));
    //     }
    // });

    loop {
        let mut buffer = [0u8; 6];
        let receive = unsafe {
            ffi::mq_receive(queue_fd, buffer.as_ptr() as *const c_char, buffer.len(), std::ptr::null_mut())
        };

        println!("Receive -> {:?}", String::from_utf8(Vec::from(buffer)));
    }
}