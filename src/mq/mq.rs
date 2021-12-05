use std::os::raw::{c_char, c_int, c_long, c_uint};

pub const MAX_MESSAGES: c_long = 10;
pub const MAX_MSG_SIZE: c_long = 4;
pub const QUEUE_PERMISSIONS: c_int = 0600;
pub const SERVER_QUEUE_NAME: &str = "/mqtest.pwPa";

pub const O_CREAT: c_int = 64;
pub const O_RDWR: c_int = 2;
pub const O_NONBLOCK: c_int = 2048;

#[repr(C)]
pub struct MqAttr {
    pub mq_flags: c_long,
    pub mq_maxmsg: c_long,
    pub mq_msgsize: c_long,
    pub mq_curmsgs: c_long,
}

#[link(name = "c")]
extern "C" {
    pub fn mq_open(server_name: *const c_char, o_flag: c_int, mode_t: u32, ...) -> c_int;
    pub fn mq_send(mqdes: c_int, buffer: *const c_char, buffer_size: c_int, prio: c_uint) -> c_int;
    pub fn mq_receive(mqdes: c_int, buffer: *const c_char, message_size: usize, prio: *const c_uint) -> c_int;
    pub fn mq_unlink(name: *const c_char) -> c_int;
    pub fn mq_close(mqd_t: c_int) -> c_int;
}
