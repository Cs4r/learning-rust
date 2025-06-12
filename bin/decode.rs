use std::env;
use std::fs::File;
use std::io::{BufReader, Read};
use learning_rust::bitvector::v1::BitVector;
use learning_rust::codec::Codec;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        eprintln!("Usage: {} <codec_file> <encoded_file>", args[0]);
        std::process::exit(1);
    }

    let codec_file = BufReader::new(File::open(&args[1])?);
    let codec = Codec::from_reader(codec_file)?;

    let input_file = BufReader::new(File::open(&args[2])?);
    let mut bit_vector = BitVector::default();

    for byte_result in input_file.bytes() {
        let byte = byte_result?;

        match byte {
            b'1' => bit_vector.add_bit(true),
            b'0' => bit_vector.add_bit(false),
            _ => continue,
        }

        if codec.is_bitvector_encoded(&bit_vector) {
            let decoded = codec.get_byte(&bit_vector);
            print!("{}", decoded as char);
            bit_vector.clear();
        }
    }

    Ok(())
}
