# Rust Serialization Examples

This repository demonstrates two popular serialization libraries in Rust: **Borsh** (used in Solana) and **Serde** (general-purpose JSON serialization).

## 📁 Project Structure

```
src/
├── main.rs          # Serde JSON serialization example
└── bin/
    └── borsh.rs     # Borsh binary serialization example
```

## 🚀 Quick Start

### Prerequisites
- Rust installed (https://rustup.rs/)
- Add dependencies to your `Cargo.toml`:

```toml
[dependencies]
borsh = {version = "1.5.7" , features=["derive"]}
serde = {version = "1.0.219", features=["derive"]}
serde_json = "1.0.140"
```

### Running the Examples

```bash
# Run Serde example (main.rs)
cargo run

# Run Borsh example (bin/borsh.rs)
cargo run --bin borsh
```

## 📊 Borsh vs Serde Comparison

| Feature | Borsh | Serde |
|---------|-------|-------|
| **Use Case** | Blockchain, Solana | Web APIs, JSON |
| **Output Format** | Binary bytes | Human-readable JSON |
| **Size** | Compact | Larger (text-based) |
| **Performance** | Very fast | Fast |
| **Schema Evolution** | Strict | Flexible |

## 🔧 Borsh Example (`src/bin/borsh.rs`)

**What it does:**
- Serializes a struct to binary bytes
- Deserializes bytes back to struct
- Perfect for Solana account data storage

```rust
use borsh::{BorshDeserialize, BorshSerialize};

#[derive(BorshDeserialize, BorshSerialize, Debug, PartialEq)]
struct MyStruct { 
    id: u64,
    data: String,
    v: Vec<u32>
}
```

**Output:**
```
Binary bytes: [42, 0, 0, 0, 0, 0, 0, 0, 17, 0, 0, 0, 72, 101, 108, 108, 111, 44, 70, 114, 111, 109, 32, 83, 116, 114, 117, 99, 116, 4, 0, 0, 0, 1, 0, 0, 0, 4, 0, 0, 0, 6, 0, 0, 0, 7, 0, 0, 0]
Deserialized: MyStruct { id: 42, data: "Hello,From Struct", v: [1, 4, 6, 7] }
```

## 📝 Serde Example (`src/main.rs`)

**What it does:**
- Converts structs to/from JSON strings
- Great for APIs and configuration files

```rust
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
struct User {
    username: String,
    password: String
}
```

**Output:**
```json
{"username":"Siddharth","password":"Siddharth"}
```

## 💡 Key Differences

### Borsh Benefits:
- **Compact**: Binary format saves space
- **Fast**: Minimal overhead
- **Deterministic**: Same input = same output
- **Solana-native**: Built for blockchain

### Serde Benefits:
- **Human-readable**: JSON is easy to debug
- **Flexible**: Many format options (JSON, YAML, XML)
- **Ecosystem**: Huge community support
- **Schema evolution**: Easier to add/remove fields

## 🎯 When to Use Each

### Use Borsh when:
- Building Solana programs
- Need maximum performance
- Working with binary protocols
- Storage space is critical

### Use Serde when:
- Building web APIs
- Need human-readable data
- Working with configuration files
- Interoperating with other systems

## 🔍 Binary Data Breakdown (Borsh)

The binary output `[42, 0, 0, 0, 0, 0, 0, 0, 17, 0, 0, 0, ...]` represents:

```
Bytes 0-7:   [42, 0, 0, 0, 0, 0, 0, 0]  → u64 id = 42
Bytes 8-11:  [17, 0, 0, 0]              → String length = 17
Bytes 12-28: [72, 101, 108, ...]        → "Hello,From Struct"
Bytes 29-32: [4, 0, 0, 0]               → Vec length = 4
Bytes 33-48: [1, 0, 0, 0, 4, 0, 0, 0, ...] → [1, 4, 6, 7]
```

## 🚀 Next Steps

1. **Experiment**: Try changing struct fields and see the output
2. **Extend**: Add more complex data types (nested structs, enums)
3. **Optimize**: Compare serialization speeds with benchmarks
4. **Learn**: Explore Solana account data patterns

## 📚 Resources

- [Borsh Documentation](https://borsh.io/)
- [Serde Documentation](https://serde.rs/)
- [Solana Cookbook](https://solanacookbook.com/)
- [Rust Serialization Guide](https://doc.rust-lang.org/book/)

---

*Happy serializing! 🦀*
