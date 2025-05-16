use learning_rust::bits::bytes_to_bits;
use learning_rust::utils::read_input;

fn main() {
    let input = read_input();
    println!("{}", bytes_to_bits(input.as_bytes()));
}