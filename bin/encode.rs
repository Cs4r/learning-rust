use std::env;
use std::fs::File;
use std::io::BufReader;
use std::io::Read;
use learning_rust::codec::Codec;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        eprintln!("Usage: {} <codec_file> <input_file>", args[0]);
        std::process::exit(1);
    }

    let codec_file = File::open(&args[1])?;
    let reader = BufReader::new(codec_file);
    let codec = Codec::from_reader(reader)?;

    let input_file = File::open(&args[2])?;
    let mut reader = BufReader::new(input_file);
    let mut buffer = [0; 1];

    while reader.read_exact(&mut buffer).is_ok() {
        let byte = buffer[0];
        
        if codec.is_byte_encoded(byte) {
            let x = codec.get_bitvector(byte);
            print!("{}", x);
        }
    }

    Ok(())
}
