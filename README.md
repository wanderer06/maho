# Sample boilerplate usage

```rust
mod maho;

fn main() {
    let context = maho::Context::create("Mahō", 800, 600);

    loop {
        match context.get_event() {
            maho::Event::Key(key_code) => println!("Key pressed, code: {}", key_code),
            maho::Event::Quit => break,
            maho::Event::None => (),
        }
    }

    println!("Program succesful exit");
}
```