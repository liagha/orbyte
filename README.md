# Orbyte

Orbyte is a Rust library that provides a procedural macro to derive serialization and deserialization implementations for structs and enums. It enables developers to easily convert Rust data structures to and from byte arrays, making it suitable for applications requiring compact data representation, such as network protocols, file storage, or embedded systems.

## Features

- **Procedural Macro**: Automatically derive `Serialize` and `Deserialize` traits for structs and enums using the `#[derive(Orbyte)]` attribute.
- **Supported Types**: Includes built-in support for primitive types (`u8`, `u16`, `u32`, `u64`, `i32`, `i64`, `f32`, `f64`, `bool`, `char`), `String`, `Vec<u8>`, and `Option<T>`.
- **Custom Structs and Enums**: Seamlessly serialize and deserialize complex data structures with named, unnamed, or unit fields/variants.
- **Little-Endian Encoding**: Uses little-endian byte order for consistent serialization of numeric types.
- **Lightweight and Safe**: Designed to be efficient with minimal dependencies, leveraging Rust's type safety.

## Installation

Add Orbyte to your project by including it in your `Cargo.toml`:

```toml
[dependencies]
orbyte = "0.1.0"
```

Orbyte requires Rust 1.65 or later. Ensure you have the latest stable Rust toolchain installed.

## Usage

### Deriving `Orbyte` for Structs

```rust
use orbyte::{Orbyte, Serialize, Deserialize};

#[derive(Orbyte)]
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let point = Point { x: 42, y: -17 };
    let bytes = point.serialize();
    println!("Serialized: {:?}", bytes);

    let deserialized = Point::deserialize(&bytes).unwrap();
    println!("Deserialized: x = {}, y = {}", deserialized.x, deserialized.y);
}
```

### Deriving `Orbyte` for Enums

```rust
use orbyte::{Orbyte, Serialize, Deserialize};

#[derive(Orbyte)]
enum Message {
    Text(String),
    Number(u32),
    Empty,
}

fn main() {
    let message = Message::Text("Hello".to_string());
    let bytes = message.serialize();
    println!("Serialized: {:?}", bytes);

    let deserialized = Message::deserialize(&bytes).unwrap();
    match deserialized {
        Message::Text(s) => println!("Deserialized: Text({})", s),
        _ => println!("Unexpected variant"),
    }
}
```

### Serialization Format

- **Structs**: Fields are serialized sequentially in declaration order.
- **Enums**: A single byte indicates the variant index, followed by the serialized fields (if any).
- **Primitive Types**: Use little-endian encoding for numbers, single-byte for `bool` (0 for `false`, 1 for `true`), and UTF-8 for `String` with a length prefix.
- **Option<T>**: `None` is an empty byte array; `Some(value)` serializes the inner value.

## Examples

Check the `examples/` directory for more detailed usage:

- `point.rs`: Demonstrates struct serialization/deserialization.
- `message.rs`: Shows enum handling with different variant types.

Run examples with:

```bash
cargo run --example point
cargo run --example message
```

## Building and Testing

Clone the repository and build the project:

```bash
git clone https://github.com/<your-username>/orbyte.git
cd orbyte
cargo build
```

Run tests to verify functionality:

```bash
cargo test
```

## Contributing

Contributions are welcome! To contribute:

1. Fork the repository.
2. Create a new branch (`git checkout -b feature/your-feature`).
3. Make your changes and commit (`git commit -m "Add your feature"`).
4. Push to your branch (`git push origin feature/your-feature`).
5. Open a pull request.

Please ensure your code follows the project's coding style and includes tests for new functionality.

### Development Setup

To work on Orbyte's procedural macro:

```toml
[dependencies]
syn = "2.0"
quote = "1.0"
proc-macro2 = "1.0"
broccli = "0.1" # For debug logging
```

Enable the `debug` feature for detailed macro processing logs:

```bash
cargo build --features debug
```

## Roadmap

- Support for more complex types (e.g., `Vec<T>`, arrays, tuples).
- Custom serialization attributes (e.g., skip fields, custom formats).
- Improved error handling with detailed error types.
- Serialization format versioning for backward compatibility.
- Zero-copy deserialization for performance-critical applications.

## License

Orbyte is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

## Contact

For questions or feedback, open an issue on GitHub or reach out to the maintainers at `<your-email>`.

---

Happy serializing with Orbyte!