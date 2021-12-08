use std::ffi::{CStr, CString};
use std::os::raw::c_char;

pub trait THandler {
    fn handle_queue_event(&self, buffer: *const c_char) -> ();
}