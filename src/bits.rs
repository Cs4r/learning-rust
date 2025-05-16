
pub fn bytes_to_bits(input: &[u8]) -> String {
    let mut bytes = String::new();

    for (i, &c) in input.iter().enumerate() {
        let byte = byte_to_bits(c);
        bytes.push_str(&byte);
        if i != input.len() - 1 {
            bytes.push(' ');
        }
    }

    bytes
}

fn byte_to_bits(byte: u8) -> String {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bytes_to_bits_empty() {
        assert_eq!(bytes_to_bits(&[]), "");
    }

    #[test]
    fn test_bytes_to_bits_single_byte() {
        assert_eq!(bytes_to_bits(&[0]), "00000000");
        assert_eq!(bytes_to_bits(&[255]), "11111111");
        assert_eq!(bytes_to_bits(&[5]), "00000101");
    }

    #[test]
    fn test_bytes_to_bits_multiple_bytes() {
        assert_eq!(bytes_to_bits(&[5, 170]), "00000101 10101010");
        assert_eq!(bytes_to_bits(b"AB"), "01000001 01000010");
        assert_eq!(bytes_to_bits(b"ABC"), "01000001 01000010 01000011");
    }

    #[test]
    fn test_bytes_to_bits_cesar_mola_capital_c() {
        let input = b"Cesar mola";
        let expected = "01000011 01100101 01110011 01100001 01110010 00100000 01101101 01101111 01101100 01100001";
        assert_eq!(bytes_to_bits(input), expected);
    }

}

