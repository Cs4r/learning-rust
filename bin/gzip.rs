use learning_rust::bitvector::v2::BitVector;
use learning_rust::code_table::CodeTable;
use learning_rust::lz_coder::LzCoder;
use learning_rust::lz77::Lz77;
use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::io::Write;
use std::{env, io};

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: {} <file_name>", args[0]);
        std::process::exit(1);
    }

    let filename = &args[1];
    let file = File::open(filename)?;
    let reader = BufReader::new(file);

    let mut lz_coder = Lz77::from_reader(reader)?;

    let mut output = File::create(filename.to_string() + ".gz")?;

    /* gzip header */
    let header = [0x1F, 0x8B, 0x08, 0x18, 0x00, 0x00, 0x00, 0x00, 0x04, 0xFF];
    output.write_all(&header)?;

    output.write_all(filename.to_string().as_bytes())?;
    output.write_all(b"\0")?;

    let comment = "Cesar Aguilera";
    output.write_all(comment.as_bytes())?;
    output.write_all(b"\0")?;

    let mut data = BitVector::new();

    deflate(&mut lz_coder, &mut data);

    for i in 0..data.n_bytes() {
        let byte = data.get_byte(i);
        output.write_all(&[byte])?
    }

    let crc32 = lz_coder.get_crc32();

    output.write_all(&crc32.to_le_bytes())?;

    let size = lz_coder.get_size() as u32;

    output.write_all(&size.to_le_bytes())?;

    output.flush()?;

    Ok(())
}

fn deflate(lz_coder: &mut Lz77, data: &mut BitVector) {
    // Deflate header
    data.add_bit(true);
    data.add_bit(true);
    data.add_bit(false);

    let table = CodeTable::new();
    let mut next_code = lz_coder.next();
    let mut code;

    while next_code != 256 {
        if next_code < 256 {
            code = table.get_huffman_length(next_code as usize);
            code.revert();
            data.append(&code);
        } else {
            let (k, bits_adc) = table.get_lz_length((next_code - 256) as usize); // k stores the LZ code of next_code - 256 and bits_adc its additional bits
            code = table.get_huffman_length(k as usize);
            code.revert();
            data.append(&code);
            data.append(&bits_adc); // Add additional bits

            let distance = lz_coder.get_distance();
            let (k, bits_adc) = table.get_lz_distance(distance as usize); // k stores the LZ distance code and bits_adc the additional bits
            code = table.get_huffman_distance(k as usize);
            code.revert();
            data.append(&code);
            data.append(&bits_adc);
        }

        next_code = lz_coder.next();
    }

    // Add the end-of-block indicator
    data.append(&table.get_huffman_length(256));
}
