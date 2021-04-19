# Sample boilerplate usage

```rust
mod maho;

fn main() {
    let context = maho::Context::create("Hello world", 800, 600);

    loop {
        match context.get_event() {
            maho::Event::Key(key_code) => println!("Yo! you just pressed {}", key_code),
            maho::Event::Quit => break,
            maho::Event::None => {},
        }
    }

    println!("Program succesful exit");
}
```