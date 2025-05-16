use std::{env, io};

pub fn read_input() -> String {
    let args: Vec<String> = env::args().collect();

    if args.len() == 2 {
        args[1].trim().to_owned()
    } else {
        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        input.trim().to_owned()
    }
}