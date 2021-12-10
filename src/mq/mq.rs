use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_int, c_long, c_uint};
use crate::{Options, TOptions};

pub const QUEUE_PERMISSIONS: c_int = 0600;

pub const O_RDONLY: c_int = 0;
pub const O_WRONLY: c_int = 1;
pub const O_CREAT: c_int = 64;
pub const O_RDWR: c_int = 2;
pub const O_NONBLOCK: c_int = 2048;

fn build_mq_attr(mq_msgsize: c_long, mq_maxmsg: c_long, mq_flags: c_long, mq_curmsgs: c_long) -> MqAttr {
    MqAttr {
        mq_flags,
        mq_curmsgs,
        mq_maxmsg,
        mq_msgsize,
    }
}

fn check_root() -> Result<bool, bool> {
    let uid = unsafe { getuid() };

    if uid != 0 {
        return Err(false);
    }

    Ok(true)
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
    pub fn getuid() -> u32;
    pub fn mq_open(server_name: *const c_char, o_flag: c_int, mode_t: u32, ...) -> c_int;
    pub fn mq_send(mqdes: c_int, buffer: *const c_char, buffer_size: c_int, prio: c_uint) -> c_int;
    pub fn mq_receive(mqdes: c_int, buffer: *const c_char, message_size: usize, prio: *const c_uint) -> c_int;
    pub fn mq_unlink(name: *const c_char) -> c_int;
    pub fn mq_close(mqd_t: c_int) -> c_int;
    pub fn mq_getattr(mqd_t: c_int, ...) -> c_int;
}

pub trait TPosixMQ<'a> {
    fn new() -> Self;
    fn with_options(&self, options: &'a Options) -> Self;
    fn create_queue(&mut self, queue_name: String) -> &Self;
    fn publish_message(&self, msg: String) -> Result<c_int, c_int>;
    fn receive(&self) -> ();
    fn get_attrs(&self) -> Result<MqAttr, MqAttr>;
    fn unlink(&self, queue_name: String) -> ();
    fn close(&self) -> Result<c_int, c_int>;
}

#[derive(Debug)]
pub struct PosixMQ<'a> {
    queue_fd: c_int,
    options: Option<&'a Options>,
}

impl<'a> TPosixMQ<'a> for PosixMQ<'a> {
    fn new() -> Self {
        if check_root().is_err() {
            panic!("Root permission needed");
        }

        Self {
            queue_fd: O_RDONLY,
            options: None,
        }
    }

    fn with_options(&self, options: &'a Options) -> Self {
        if check_root().is_err() {
            panic!("Root permission needed");
        }

        Self {
            queue_fd: -1,
            options: Some(options),
        }
    }

    fn create_queue(&mut self, queue_name: String) -> &Self {
        let options = self.options.unwrap();
        let flag = options.get_flag();
        let max_msg_size = options.get_max_messages();
        let max_msg_buffer_size = options.get_max_message_buffer_size();

        let queue_fd = unsafe {
            let queue_name = CString::new(queue_name).unwrap();
            mq_open(queue_name.as_ptr(), flag | O_CREAT, QUEUE_PERMISSIONS as u32, &build_mq_attr(max_msg_buffer_size as c_long, max_msg_size as c_long, 0, 0))
        };

        self.queue_fd = queue_fd;

        if queue_fd < 0 {
            panic!("Queue cannot create")
        }

        self
    }

    fn publish_message(&self, msg: String) -> Result<c_int, c_int> {
        if self.queue_fd < 0 {
            panic!("Queue FD not found")
        }

        let len = msg.len() as c_int;
        let msg = CString::new(msg).unwrap();

        let send_result = unsafe {
            mq_send(self.queue_fd, msg.as_ptr(), len as c_int, 10)
        };

        if send_result < 0 {
            return Err(-1);
        }

        Ok(0)
    }

    fn receive(&self) -> () {
        let mut options = self.options.unwrap();
        let handler = options.get_handler();
        let max_msg_buffer_size = options.get_max_message_buffer_size();

        if self.queue_fd < 0 {
            // Check with generic error functions
            panic!("Queue FD not found")
        }

        loop {
            let buffer = CString::default();
            let ptr = buffer.as_ptr();
            unsafe {
                mq_receive(self.queue_fd, ptr, max_msg_buffer_size as usize, std::ptr::null_mut())
            };

            if handler.is_some() {
                let readable = unsafe { CStr::from_ptr(buffer.as_ptr()) };
                handler.as_ref().unwrap().handle_queue_event(buffer.as_ptr(), readable.to_str().expect("data error"))
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

    fn unlink(&self, queue_name: String) -> () {
        let c_str = CString::new(queue_name).unwrap();
        unsafe {
            mq_unlink(c_str.as_ptr());
        }
    }

    fn close(&self) -> Result<c_int, c_int> {
        let close = unsafe {
            mq_close(self.queue_fd)
        };

        if close < 0 {
            return Err(-1);
        }

        Ok(0)
    }
}