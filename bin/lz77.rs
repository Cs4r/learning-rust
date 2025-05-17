use std::io::Cursor;
use learning_rust::lz_coder::LzCoder;
use learning_rust::utils::read_input;

fn main() {
    let input = read_input();

    let reader = Cursor::new(input.as_bytes());

    let mut lz_coder = LzCoder::from_reader(reader).unwrap();

    println!("{}", lz_coder.compressed_string());
}