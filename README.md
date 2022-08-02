# 52-weeks-rust
Trying out Rust

## How to Install

* [Install Steps](https://doc.rust-lang.org/book/ch01-01-installation.html)
* Pro-tip, run `rustup update` to update your installation.
* Check version do this: `rustc --version`

## How the heck do I compile?

* `rustc myfunc.rs` in this case, the `<yourfile>.rs` is what is compiled.
* Next you run it:  ./myfunc.rs

## How do I use Cargo?

* Do you have it? `cargo --version`?
* `cargo new hello_cargo && cd hello_cargo`


## Episode 3

Make a function:

* `cargo new hello_func && cd hello_func` (put some code in src/main like this)

```rust
//Function 1
fn main() {
    println!("Hello, world!");

    another_function();
}

//Function 2
fn another_function() {
    println!("Another function.");
}
```

* `cargo build`

## Episode 2
## Trying to learn Rust

Quick example:
```rust
use ferris_says::say; // from the previous step
use std::io::{stdout, BufWriter};

fn main() {
    let stdout = stdout();
    let message = String::from("Hello fellow Rustaceans!");
    let width = message.chars().count();

    let mut writer = BufWriter::new(stdout.lock());
    say(message.as_bytes(), width, &mut writer).unwrap();
}
```

* [learn by example guide](https://doc.rust-lang.org/stable/rust-by-example/)
