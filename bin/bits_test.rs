use std::{env, io};
use learning_rust::bits::str_to_bits;

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

        &(input.trim().to_owned())
    };

    println!("{}", str_to_bits(input));
}