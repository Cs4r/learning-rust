use learning_rust::parity::parity;
use learning_rust::utils::read_input;

fn main() {
    let input = read_input();
    let bit = if parity(input.as_bytes()) { 1 } else { 0 };
    println!("The parity bit is: {}", bit);
}