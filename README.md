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

### 6. `decode`

Reads a file containing a Huffman-encoded bitstream (as ASCII '0' and '1' characters), decodes it using a given Huffman codec, and prints the decoded text to standard output.

##### 📥 Input file format

- The codec file must follow the format described in the encode program (first line is the number of elements, then each line contains a character and its Huffman code).
- The encoded file should contain a sequence of ASCII characters '0' and '1' representing the Huffman-encoded bitstream.

##### ✅ Usage

Run the program providing the codec file and the encoded input file:

```bash
   cargo run --bin decode -- <codec_file> <encoded_file> > <decoded_file>
```

Example:

```bash
   cargo run --bin decode -- assets/huffman_ascii_es.txt assets/encoded.txt > decoded.txt
```

### 7. `lz77`

Reads an input string, compresses it using an LZ77-based algorithm, and prints the compressed output to standard output. The compression identifies repeated substrings and encodes them as references, otherwise outputs literal characters.

##### 📥 Input format
 - The input is read from standard input (stdin) as a UTF-8 string. 
 - The input can contain any valid UTF-8 characters, including ASCII and Unicode.

##### 📤 Output format
- Literal characters are printed as-is. 
- Repeated sequences of length ≥ 3 are encoded as #REF(length,distance)# where:
  - length is the length of the repeated substring plus 256, 
  - distance is how far back the match starts relative to the current position.
- The special code 256 signals end of input and is not printed.

##### ✅ Usage

Run the program, then enter the text to compress:

```bash
   cargo run --bin lz77 < <input_file> > <output_file>
```

Example:

```bash
   cargo run --bin lz77 -- < assets/sample.txt > assets/sample_compressed.txt
```

## 🛠 Requirements

- Rust (edition 2024)
- Cargo

To install Rust: https://rustup.rs/



Made with ❤️ using Rust.