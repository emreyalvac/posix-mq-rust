use std::io::Error;
use std::os::raw::c_int;
use crate::{Handler, O_RDONLY, O_RDWR, O_WRONLY, PosixMQ, THandler};
use crate::mq::TPosixMQ;


pub trait TOptions {
    fn new(flag: c_int) -> Self;
    fn read_only() -> Self;
    fn write_only() -> Self;
    fn read_n_write() -> Self;
    fn with_handler<T>(&mut self, handler: T) -> PosixMQ where T: THandler;
    fn get_flag(&self) -> Option<c_int>;
    fn open(&self) -> PosixMQ;
}

#[derive(Debug)]
pub struct Options {
    flag: Option<c_int>,
    pub handler: Option<Box<dyn THandler>>,
}

impl TOptions for Options {
    fn new(flag: c_int) -> Self {
        Options {
            flag: Some(flag),
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

    fn with_handler<T>(&mut self, handler: T) -> PosixMQ where T: THandler {
        self.handler = Some(Box::new(handler));
        PosixMQ::new(self)
    }

    fn get_flag(&self) -> Option<c_int> {
        if self.flag.is_none() {
            return Some(O_RDONLY);
        }
        self.flag
    }

    fn open(&self) -> PosixMQ {
        PosixMQ::new(self)
    }
}