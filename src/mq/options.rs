use std::os::raw::c_int;
use crate::{O_RDONLY, O_RDWR, O_WRONLY, PosixMQ};


pub trait TOptions {
    fn new() -> Self;
    fn read_only(&self) -> Self;
    fn write_only(&self) -> Self;
    fn read_n_write(&self) -> Self;
    fn get_o_flag(&self) -> Option<c_int>;
}

pub struct Options {
    o_flag: Option<c_int>,
}

impl TOptions for Options {
    fn new() -> Self {
        Options {
            o_flag: None
        }
    }

    fn read_only(&self) -> Self {
        Options {
            o_flag: Some(O_RDONLY)
        }
    }

    fn write_only(&self) -> Self {
        Options {
            o_flag: Some(O_WRONLY)
        }
    }

    fn read_n_write(&self) -> Self {
        Options {
            o_flag: Some(O_RDWR)
        }
    }

    fn get_o_flag(&self) -> Option<c_int> {
        if self.o_flag.is_none() {
            return Some(O_RDONLY);
        }
        self.o_flag
    }
}