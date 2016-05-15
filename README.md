# Stepper

Rust library for range iterators that step by values other than one. Designed to
be used until `std::iter::StepBy` becomes stable.

## Example

```rust
extern crate stepper;

#[macro_use]
use stepper::Stepper;

fn main() {
    // Prints "0", "2", "4"
    for i in step!(0 => 5; 2) {
        println!("{}", i);
    }

    // Prints "8", "5", "2"
    for i in step!(8 => 0; -3) {
        println!("{}", i);
    }
}
```

## Numerical Types

Currently only supports `i64`. If you want to use Stepper with some other numerical type, open an issue and I'll implement it when I get a chance.
