use std::os::raw::c_int;
use crate::{O_NONBLOCK, O_RDONLY, O_RDWR, O_WRONLY, THandler};

pub trait TOptions {
    fn new(flag: c_int) -> Self;
    fn read_only() -> Self;
    fn write_only() -> Self;
    fn read_n_write() -> Self;
    fn with_handler<T>(&mut self, handler: T) -> &mut Self where T: THandler;
    fn non_blocking(&mut self) -> &mut Self;
    fn get_flag(&self) -> c_int;
    fn get_handler(&self) -> &Option<Box<dyn THandler>>;
}

#[derive(Debug)]
pub struct Options {
    flag: c_int,
    pub handler: Option<Box<dyn THandler>>,
}

impl TOptions for Options {
    fn new(flag: c_int) -> Self {
        Options {
            flag,
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

    fn get_flag(&self) -> c_int {
        if self.flag < 0 {
            return O_RDONLY;
        }
        self.flag
    }

    fn get_handler(&self) -> &Option<Box<dyn THandler>> {
        &self.handler
    }
}