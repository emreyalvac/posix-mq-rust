#### Still in development

# Rust - POSIX MQ

## Usage

### Define handler

```rust
pub struct Handler {}

impl THandler for Handler {
    fn handle_queue_event(&self, buffer: *const c_char) -> () {
        println!("{:?}", buffer);
    }
}
```

```rust
let handler = Handler {};
let options = Options::new().read_n_write();
let mut posix_mq = PosixMQ::new(options, handler);

posix_mq.create_queue(String::from("/mq_instance_1")).expect("mq_open failed");

posix_mq.receive_from_queue();
```