use learning_rust::lz_coder::LzCoder;
use learning_rust::utils::read_input;
use std::io::Cursor;

fn main() {
    let input = read_input();

    let output = compressed_string(&input);

    println!("{}", output);
}

fn compressed_string(input: &str) -> String {
    let cursor = Cursor::new(input.as_bytes());
    let mut lz_coder = LzCoder::from_reader(cursor).unwrap();

    let mut output = String::new();
    let mut next = lz_coder.next();

    while next != 256 {
        if next < 256 {
            output.push(char::from_u32(next as u32).unwrap());
        } else {
            output.push_str(&format!(
                "#REF({},{})#",
                next - 256,
                lz_coder.get_distance()
            ));
        }
        next = lz_coder.next();
    }

    output
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compressed_string_sample1() {
        assert_eq!(
            compressed_string(
                "There is a cow. The cow is white. The cow is big. The cow is a mammal."
            ),
            "There is a cow. #REF(3,16)##REF(4,9)##REF(4,18)#white#REF(13,18)#big#REF(13,16)#a mammal."
        );
    }

    #[test]
    fn test_compressed_string_sample2() {
        assert_eq!(
            compressed_string(
                "The quick brown fox jumps over the lazy dog. The quick brown fox is fast."
            ),
            "The quick brown fox jumps over t#REF(3,31)#lazy dog. #REF(20,45)#is fast."
        );
    }

    #[test]
    fn test_compressed_string_sample3() {
        assert_eq!(
            compressed_string("Hello world! Hello world! Hello everyone."),
            "Hello world! #REF(19,13)#everyone."
        );
    }

    #[test]
    fn test_compressed_string_sample4() {
        assert_eq!(
            compressed_string("There is a cat. The cat is small. The cat is quiet."),
            "There is a cat. #REF(3,16)##REF(4,9)##REF(4,18)#small#REF(13,18)#quiet."
        );
    }
}
