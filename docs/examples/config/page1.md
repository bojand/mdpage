# Page 1

Page 1 content.

If the `"label"` property **is not** set in the config file it will be infered from the heading of the content within this file.

We only bring in `TOML` language support so the following `Rust` code is not highlighted:

```rust
#[derive(Debug)]
struct Person<'a> {
    name: &'a str,
    age: u8
}

fn main() {
    let name = "Peter";
    let age = 27;
    let peter = Person { name, age };

    // Pretty print
    println!("{:#?}", peter);
}

```