use std::ffi::CString;
use std::os::raw::{c_char, c_int, c_long, c_uint};

const MAX_MESSAGES: c_long = 10;
const MAX_MSG_SIZE: c_long = 10;
const QUEUE_PERMISSIONS: c_int = 0744;
const SERVER_QUEUE_NAME: &str = "/mqtest.pwPa";

mod ffi {
    use std::os::raw::{c_char, c_int, c_long, c_uint};

    pub const O_RDONLY: c_int = 0;
    pub const O_WRONLY: c_int = 1;
    pub const O_CREAT: c_int = 64;
    pub const O_NONBLOCK: c_int = 2048;
    pub const O_RDWR: c_int = 2;
    pub const O_EXCL: c_int = 128;

    pub const S_IRUSR: c_int = 256;
    pub const S_IWUSR: c_int = 128;
    pub const S_IWGRP: c_int = 16;

    #[repr(C)]
    pub struct MqAttr {
        pub mq_flags: c_long,
        pub mq_maxmsg: c_long,
        pub mq_msgsize: c_long,
        pub mq_curmsgs: c_long,
    }

    #[link(name = "c")]
    extern "C" {
        pub fn mq_open(server_name: *const c_char, o_flag: c_int, ...) -> c_int;
        pub fn mq_receive(mqdes: c_int, buffer: *const c_char, buffer_size: c_int, prio: *mut c_uint) -> c_int;
        pub fn mq_send(mqdes: c_int, buffer: *const c_char, buffer_size: c_int, prio: *mut c_uint) -> c_int;
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

fn main() {}

