#### Still in development

# Rust - POSIX MQ

## Mount

```bash
sudo mkdir /dev/mqueue
sudo mount -t mqueue none /dev/mqueue/

```

## Usage

### Define handler

```rust
pub struct Handler {}

impl THandler for Handler {
    fn handle_queue_event(&self, buffer: *const c_char, data: &str) -> () {
        println!("{:?} {:?}", buffer, data);
    }
}
```

```rust
fn main() {
    let handler = Handler {};

    let mut options = Options::read_n_write();
    options.with_handler(handler).max_messages(30).max_message_buffer_size(124);

    let mut posix_mq = PosixMQ::new().with_options(&options);
    posix_mq.create_queue(String::from("/mqtest"));

    let attrs = posix_mq.get_attrs();

    posix_mq.receive();
}
```
