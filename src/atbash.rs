fn atbash_cipher(text:&str) -> String {
    // retrieve chars
    text.chars()
        .map(|char| {
            if char.is_ascii_uppercase() {
                // Map the character to its reverse counterpart in the alphabet, by applying a difference to the binary literal representations as boundaries.
                (b'Z' - (char as u8 - b'A')) as char
            } else if char.is_ascii_lowercase() {
                (b'z' - (char as u8 - b'a')) as char
            } else {
                // If the character is not alphabetic, just return it
                char
            }
        }).collect()
}

#[cfg(test)]
mod atbash_tests {
    use super::*;

    #[test]
    fn test_atbash_basic_mapping() {
        // Test uppercase letters
        let input_upper = "ABC";
        let expected_upper = "ZYX";
        assert_eq!(atbash_cipher(input_upper), expected_upper);

        // Test lowercase letters
        let input_lower = "abc";
        let expected_lower = "zyx";
        assert_eq!(atbash_cipher(input_lower), expected_lower);
    }

    #[test]
    fn test_atbash_mixed_case_non_alphabetic() {
        let input = "Hello, World!";
        let expected = "Svool, Dliow!";
        assert_eq!(atbash_cipher(input), expected);
    }

    #[test]
    fn test_atbash_full_alphabet() {
        let input = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz";
        let expected = "ZYXWVUTSRQPONMLKJIHGFEDCBAzyxwvutsrqponmlkjihgfedcba";
        assert_eq!(atbash_cipher(input), expected);
    }

}