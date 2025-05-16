use learning_rust::crc32::crc32;
use learning_rust::utils::read_input;

fn main() {
    let input = read_input();
    let crc = crc32(input.as_bytes());
    println!("The CRC is:  {:x}", crc);
}

