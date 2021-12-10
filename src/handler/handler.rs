use std::fmt::Debug;
use std::os::raw::c_char;

pub trait THandler: 'static + Debug + Send + Sync {
    fn handle_queue_event(&self, buffer: *const c_char, data: &str) -> ();
}