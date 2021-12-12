use crate::{THandler};
use std::os::raw::{c_int};

pub const O_RDONLY: c_int = 0;
pub const O_WRONLY: c_int = 1;
pub const O_CREAT: c_int = 64;
pub const O_RDWR: c_int = 2;
pub const O_NONBLOCK: c_int = 2048;

pub const SIGEV_THREAD: c_int = 2;
pub const SIGEV_SIGNAL: c_int = 0;

pub trait TOptions {
    fn new(flag: c_int) -> Self;
    fn read_only() -> Self;
    fn write_only() -> Self;
    fn read_n_write() -> Self;
    fn with_handler<T>(&mut self, handler: T) -> &mut Self where T: THandler;
    fn non_blocking(&mut self) -> &mut Self;
    fn max_messages(&mut self, msg_size: c_int) -> &mut Self;
    fn max_message_buffer_size(&mut self, size: c_int) -> &mut Self;
    fn get_flag(&self) -> c_int;
    fn get_handler(&self) -> &Option<Box<dyn THandler>>;
    fn get_max_messages(&self) -> c_int;
    fn get_max_message_buffer_size(&self) -> c_int;
}

#[derive(Debug)]
pub struct Options {
    flag: c_int,
    max_messages: c_int,
    max_message_buffer_size: c_int,
    pub handler: Option<Box<dyn THandler>>,
}

impl TOptions for Options {
    fn new(flag: c_int) -> Self {
        Options {
            flag,
            max_messages: 10,
            max_message_buffer_size: 1024,
            handler: None,
        }
    }

    fn read_only() -> Self {
        Options::new(O_RDONLY)
    }

    fn write_only() -> Self {
        Options::new(O_WRONLY)
    }

    fn read_n_write() -> Self {
        Options::new(O_RDWR)
    }

    fn with_handler<T>(&mut self, handler: T) -> &mut Self where T: THandler {
        self.handler = Some(Box::new(handler));
        self
    }

    fn non_blocking(&mut self) -> &mut Self {
        self.flag |= O_NONBLOCK;

        self
    }

    fn max_messages(&mut self, msg_size: c_int) -> &mut Self {
        self.max_messages = msg_size;

        self
    }

    fn max_message_buffer_size(&mut self, size: c_int) -> &mut Self {
        self.max_message_buffer_size = size;

        self
    }

    fn get_flag(&self) -> c_int {
        if self.flag < 0 {
            return O_RDONLY;
        }
        self.flag
    }

    fn get_handler(&self) -> &Option<Box<dyn THandler>> {
        &self.handler
    }

    fn get_max_messages(&self) -> c_int {
        self.max_messages
    }

    fn get_max_message_buffer_size(&self) -> c_int {
        self.max_message_buffer_size
    }
}