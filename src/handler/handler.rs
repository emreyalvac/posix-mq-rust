pub trait THandler {
    fn new() -> Self;
    fn handle_queue_event(&self, buffer: Vec<u8>) -> ();
}

pub struct Handler {}

impl THandler for Handler {
    fn new() -> Self {
        Handler {}
    }

    fn handle_queue_event(&self, buffer: Vec<u8>) -> () {
        let byte_to_string = String::from_utf8(buffer);

        println!("Handle {:?}", byte_to_string);
    }
}