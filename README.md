# in

A simple Rust crate that provides Python-style input functions through macros.

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
in = "1.0.1"
```

## Usage

This crate provides two macros:
- `input!()`: Get user input on the same with an optional prompt
- `inputln!()`: Same as `input!()` but user input is taken on the next line

### Examples

```rust
use in::input;
use in::inputln;

fn main() {
    let a: String = input!();
    // Enter a number: x
    let x = input!("Enter a number: ").parse::<f32>().unwrap();

    // Enter a number:
    // x
    let y = inputln!("Enter a number: ").parse::<f32>().unwrap();
    
    // x
    let z = input!().parse::<f32>().unwrap();

    println!("x = {x}");
}
```

## License

This project is licensed under the MIT License.