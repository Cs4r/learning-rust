mod bits_test;

use learning_rust::parity::parity;
use std::{env, io};

fn main() {
    let args: Vec<String> = env::args().collect();

    let input = if args.len() == 2 {
        args[1].trim()
    } else {
        println!("Enter something:");
        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        &(input.to_owned())
    };

    print_parity(input);
}

fn print_parity(input: &str) {
    let bit = if parity(input) { 1 } else { 0 };
    println!("The parity bit is: {}", bit);
}
