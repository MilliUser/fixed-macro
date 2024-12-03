# ToFixed Attribute Macro for Rust

This project provides a Rust procedural macro (`to_fixed`) to convert fixed-point numbers. The macro takes a specific constant array and converts each element into a fixed-point representation using types from the `fixed::types` crate.

## Features

- Works with fixed-point types like `U{int_bits}F{frac_bits}`.
- When applied to a constant array, each element is converted to a fixed-point type.
- Currently works only with `f32` values.

## Installation

## Usage
```rust
use fixed::types::U2F14;

#[to_fixed]
const SIN_TABLE: [U2F14; 256] = [
    0.0, 0.1, 0.2, 0.3, // ... continue
];

fn main() {
    println!("{:?}", SIN_TABLE);
}
```

## Contributing

Contributions to the project are welcome. If you think something is missing or needs improvement, feel free to contribute by submitting a pull request or opening an issue.

### Contribution Guidelines

- Changes should aim to make the project more stable and useful.
- Please consider adding tests for new features or fixes.
- Before submitting a pull request, ensure that you provide a clear description of what has been changed and why.

### Future Plans

The project is expected to evolve over time with more features and improvements. These may include:

- Support for additional fixed-point types.
- Performance enhancements.
- More tests and debugging.

### License

This project is licensed under the MIT License. For more information, please refer to the LICENSE file.

