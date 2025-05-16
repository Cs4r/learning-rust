# 🦀 learning-rust


## 📦 Binaries

This project provides several executables:

### 1. `parity_test`

Calculates the overall parity bit of the input bytes. The parity bit is `1` if there is an odd number of bits set to `1`, otherwise `0`.

##### ✅ Usage

```bash
cargo run --bin parity_test "Your input text"
```

Or, run it interactively by omitting the argument:

```bash
cargo run --bin parity_test
```

### 2. `bits_test`

Displays the 8-bit binary representation of each byte in the input.

##### ✅ Usage

```bash
cargo run --bin bits_test "Your input text"
```

Or interactively:

```bash
cargo run --bin bits_test
```

### 3. `lrc_test`

Computes the Longitudinal Redundancy Check (LRC) of the input bytes using bitwise XOR.

##### ✅ Usage

```bash
cargo run --bin lrc_test "Your input text"
```

Or interactively:

```bash
cargo run --bin lrc_test
```


## 🛠 Requirements

- Rust (edition 2024)
- Cargo

To install Rust: https://rustup.rs/



Made with ❤️ using Rust.