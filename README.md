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

### 4. `crc32_test`

Computes the CRC32 (Cyclic Redundancy Check) checksum of the input bytes.

##### ✅ Usage

```bash
    cargo run --bin crc32_test "Your input text"
```

Or interactively:

```bash
cargo run --bin crc32_test
```

### 5. `encode`

Encodes a plain text file using a custom Huffman codec.

This program reads a file containing a Huffman encoding table (one line per character and bit sequence), then uses that codec to transform an input text file into its corresponding Huffman-encoded bitstream.

##### ✅ Usage

```bash
    cargo run --bin encode -- <codec_file> <input_file>
```

Where:

- <codec_file> is a plain text file containing ASCII character-to-bits mappings (one per line),

- <input_file> is the text file you want to encode using the codec.

For example:

```bash
    cargo run --bin encode -- assets/huffman_ascii_es.txt assets/constitution.txt > encoded.txt
```

This will produce the encoded output in standard output (STDOUT), which you can redirect to a file as shown.

##### 📥 Input format (codec_file)

The codec file must follow this format:

- The first line contains a single integer: the number of elements (character-to-code mappings) that follow.
- Each line must contain a single character followed by its Huffman encoding, separated by a space. Example:

```
3
e 010
n 0110
s 0111
```

Lines without a character (just a space followed by bits) are used to encode the space character.

##### 📤 Output

The program prints the encoded bitstream (as a sequence of '0' and '1' characters) to stdout.

## 🛠 Requirements

- Rust (edition 2024)
- Cargo

To install Rust: https://rustup.rs/



Made with ❤️ using Rust.