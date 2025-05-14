
pub fn str_to_bits(input: &str) -> String {
    let mut bytes = String::new();

    for (i, c) in input.chars().enumerate() {
        let byte = char_to_bits(c as u8);
        bytes.push_str(&byte);
        if i != input.len() - 1 {
            bytes.push(' ');
        }
    }

    bytes
}

fn char_to_bits(byte: u8) -> String {
    let mut bits = String::new();

    for i in (0..8).rev() {
        if byte & (1 << i) != 0 {
            bits.push('1');
        } else {
            bits.push('0');
        }
    }

    bits
}

fn int_to_bits(num: u32) -> String {
    let mut bits = String::new();

    for i in (0..32).rev() {
        if num & (1 << i) != 0 {
            bits.push('1');
        } else {
            bits.push('0');
        }
    }

    bits
}
