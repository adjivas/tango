pub const ONE_TEXT_LINE_RS: &'static str = "//@ This is a demo without code.";
pub const ONE_TEXT_LINE_MD: &'static str = "This is a demo without code.";

pub const ONE_RUST_LINE_RS: &'static str = r#"fn main() { println!("one rust line"); }"#;

pub const ONE_RUST_LINE_MD: &'static str = r#"```rust
fn main() { println!("one rust line"); }
```
"#;

pub const HELLO_RS: &'static str = r#"//@ # Hello World
//@ This is a Hello World demo.

// Code started here (at this normal comment)
fn main() { println!("Hello World"); }
//@ And then the text resumes here.
"#;

pub const HELLO_MD: &'static str = r#"# Hello World
This is a Hello World demo.

```rust
// Code started here (at this normal comment)
fn main() { println!("Hello World"); }
```
And then the text resumes here.
"#;

pub const HELLO2_RS: &'static str = r#"//@ # Hello World
//@ This is a second Hello World demo.

// Code started here (at this normal comment)
fn main() { println!("Hello World"); }

//@ And then the text resumes here, after a line break.
"#;

pub const HELLO2_MD: &'static str = r#"# Hello World
This is a second Hello World demo.

```rust
// Code started here (at this normal comment)
fn main() { println!("Hello World"); }
```

And then the text resumes here, after a line break.
"#;

pub const HELLO3_RS: &'static str = r#"

// Code started here (at this normal comment)
fn main() { hello() }

//@ Here is some expository text in the middle
//@ It spans ...
//@ ... multiple lines

// Here is yet more code!
// (and we end with code, not doc)
fn hello() { println!("Hello World"); }
"#;

pub const HELLO3_MD: &'static str = r#"

```rust
// Code started here (at this normal comment)
fn main() { hello() }
```

Here is some expository text in the middle
It spans ...
... multiple lines

```rust
// Here is yet more code!
// (and we end with code, not doc)
fn hello() { println!("Hello World"); }
```
"#;