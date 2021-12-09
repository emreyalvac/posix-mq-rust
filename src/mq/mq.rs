use std::ffi::CString;
use std::io::Error;
use std::os::raw::{c_char, c_int, c_long, c_uint};
use crate::{Handler, Options, THandler, TOptions};

pub const MAX_MESSAGES: c_long = 10;
pub const MAX_MSG_SIZE: c_long = 2048;
pub const QUEUE_PERMISSIONS: c_int = 0600;

pub const O_RDONLY: c_int = 0;
pub const O_WRONLY: c_int = 1;
pub const O_CREAT: c_int = 64;
pub const O_RDWR: c_int = 2;
pub const O_NONBLOCK: c_int = 2048;

fn build_mq_attr(mq_flags: c_long, mq_curmsgs: c_long) -> MqAttr {
    MqAttr {
        mq_flags,
        mq_curmsgs,
        mq_maxmsg: MAX_MESSAGES,
        mq_msgsize: MAX_MSG_SIZE,
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct MqAttr {
    pub mq_flags: c_long,
    pub mq_maxmsg: c_long,
    pub mq_msgsize: c_long,
    pub mq_curmsgs: c_long,
}

impl MqAttr {
    fn new() -> Self {
        MqAttr {
            mq_flags: 0,
            mq_maxmsg: 0,
            mq_msgsize: 0,
            mq_curmsgs: 0,
        }
    }
}

#[link(name = "c")]
extern "C" {
    pub fn mq_open(server_name: *const c_char, o_flag: c_int, mode_t: u32, ...) -> c_int;
    pub fn mq_send(mqdes: c_int, buffer: *const c_char, buffer_size: c_int, prio: c_uint) -> c_int;
    pub fn mq_receive(mqdes: c_int, buffer: *const c_char, message_size: usize, prio: *const c_uint) -> c_int;
    pub fn mq_unlink(name: *const c_char) -> c_int;
    pub fn mq_close(mqd_t: c_int) -> c_int;
    pub fn mq_getattr(mqd_t: c_int, ...) -> c_int;
}

pub trait TPosixMQ<'a> {
    fn new(options: &'a Options) -> Self;
    fn create_queue(&mut self, queue_name: String) -> &Self;
    fn publish_message(&self, msg: String) -> Result<bool, bool>;
    fn receive(&self) -> ();
    fn get_attrs(&self) -> Result<MqAttr, MqAttr>;
}

#[derive(Debug)]
pub struct PosixMQ<'a> {
    queue_fd: c_int,
    options: &'a Options,
}

impl<'a> TPosixMQ<'a> for PosixMQ<'a> {
    fn new(options: &'a Options) -> Self {
        Self {
            queue_fd: -1,
            options,
        }
    }

    fn create_queue(&mut self, queue_name: String) -> &Self {
        let flag = self.options.get_flag().expect("Flag not found");

        let queue_fd = unsafe {
            let queue_name = CString::new(queue_name).unwrap();
            unsafe {
                mq_open(queue_name.as_ptr(), flag | O_CREAT, QUEUE_PERMISSIONS as u32, &build_mq_attr(0, 0))
            }
        };

        self.queue_fd = queue_fd;

        if queue_fd < 0 {
            panic!("Queue cannot create")
        }

        self
    }

    fn publish_message(&self, msg: String) -> Result<bool, bool> {
        if self.queue_fd < 0 {
            panic!("Queue FD not found")
        }

        let len = msg.len() as c_int;
        let msg = CString::new(msg).unwrap();

        let send_result = unsafe {
            mq_send(self.queue_fd, msg.as_ptr(), len as c_int, 10)
        };

        if send_result < 0 {
            return Err(false);
        }

        Ok(true)
    }

    fn receive(&self) -> () {
        let handler = &self.options.handler;

        if self.queue_fd < 0 {
            // Check with generic error functions
            panic!("Queue FD not found")
        }

        loop {
            let buffer = CString::default();
            let ptr = buffer.as_ptr();
            unsafe {
                mq_receive(self.queue_fd, ptr, MAX_MSG_SIZE as usize, std::ptr::null_mut())
            };

            if handler.is_some() {
                handler.as_ref().unwrap().handle_queue_event(buffer.as_ptr())
            }
        }
    }

    fn get_attrs(&self) -> Result<MqAttr, MqAttr> {
        if self.queue_fd < 0 {
            panic!("Queue FD Not Found")
        }

        let attr_s = MqAttr::new();

        let attr = unsafe {
            mq_getattr(self.queue_fd, &attr_s)
        };

        if attr < 0 {
            return Err(attr_s);
        }

        Ok(attr_s)
    }
}