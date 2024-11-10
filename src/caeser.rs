fn caesar_encrypt(text: &str, shift: u8) -> String {
    // Get an Iterator for all chars within text
    text.chars()
        .map(|char| {
            // Only operate on a...z and A...Z
            if char.is_ascii_alphabetic() {
                // The starting letter differs based on case
                let a = if char.is_ascii_uppercase() {
                    b'A' // This is a byte literal, it has type u8, 8 bits, and represents the ascii value 65u8.
                } else {
                    b'a'
                };

                // The caeser shift operation
                // 1. Convert the char to its ASCII byte representation (8bits)
                // 2. Calculate its relative position to a base character (either a or A)
                // 3. Apply a shift value
                // 4. Ensure the result "wraps" around the alphabet by using the modulo operation.
                // 5. Convert the resulting byte back to a char.
                (((char as u8 - a + shift) % 26) + a) as char
            } else {
                // When the char is not alphabetic, just return it raw for now.
                char
            }
        })
        .collect()
}

fn caesar_decrypt(text: &str, shift: u8) -> String {
    // Decryption is achieved by shifting in the opposite direction.
    // Since shifting forward by (26 - shift) is equivalent to shifting backward by shift, we reuse the caesar_encrypt function.
    caesar_encrypt(text, 26- (shift % 26))
}

#[cfg(test)]
mod caeser_test {
    use crate::caeser::{caesar_decrypt, caesar_encrypt};

    #[test]
    fn test_caesar_encrypt_wrap_around() {
        assert_eq!(caesar_encrypt("XYZ", 3), "ABC");
        assert_eq!(caesar_encrypt("xyz", 3), "abc");
    }

    #[test]
    fn test_caesar_decrypt_wrap_around() {
        assert_eq!(caesar_decrypt("ABC", 3), "XYZ");
        assert_eq!(caesar_decrypt("abc", 3), "xyz");
    }

    #[test]
    fn test_caesar_encrypt_mixed_case() {
        let plaintext = "Hello, World!";
        let expected = "Khoor, Zruog!";
        assert_eq!(caesar_encrypt(plaintext, 3), expected);
    }

    #[test]
    fn test_caesar_decrypt_mixed_case() {
        let encrypted = "Khoor, Zruog!";
        let expected = "Hello, World!";
        assert_eq!(caesar_decrypt(encrypted, 3), expected);
    }

    #[test]
    fn test_caesar_non_alphabetic() {
        let plaintext = "123!@#";
        let expected = "123!@#";
        assert_eq!(caesar_encrypt(plaintext, 5), expected);
        assert_eq!(caesar_decrypt(plaintext, 5), expected);
    }

    #[test]
    fn test_caesar_shift_zero() {
        let plaintext = "No Change!";
        assert_eq!(caesar_encrypt(plaintext, 0), plaintext);
        assert_eq!(caesar_decrypt(plaintext, 0), plaintext);
    }

    #[test]
    fn test_caesar_shift_26() {
        let plaintext = "Full Circle!";
        assert_eq!(caesar_encrypt(plaintext, 26), plaintext);
        assert_eq!(caesar_decrypt(plaintext, 26), plaintext);
    }

    #[test]
    fn test_caesar_shift_greater_than_26() {
        let plaintext = "Shift by 29!";
        let encrypted = caesar_encrypt(plaintext, 29); // Equivalent to shift 3
        let expected = "Vkliw eb 29!";
        assert_eq!(encrypted, expected);

        let decrypted = caesar_decrypt(&encrypted, 29);
        assert_eq!(decrypted, plaintext);
    }
}