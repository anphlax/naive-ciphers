use num_traits::cast::ToPrimitive;
use std::str::FromStr;
use base64::DecodeError;
use base64::engine::general_purpose;
use num_bigint::BigInt;
use crate::text_encode::{num2str, str2num, DEFAULT_SYMBOLS};
use base64::prelude::*;

/// This module provides functions for encoding and decoding binary data to and from numbers using a custom symbol set.
///
/// The primary functionality includes:
/// - **bin2num**: Converts binary data (as bytes) into a number by encoding it in Base64 and then interpreting the Base64 string as a number using a custom symbol set.
/// - **num2bin**: Converts a number back into binary data (as bytes) by first converting it to a string using a custom symbol set and then decoding it from Base64.
///
/// **Symbol Set**: The encoding and decoding operations use a predefined set of symbols, where each symbol corresponds to a unique value. The default symbol set includes digits, letters, punctuation, and other special characters.
///
/// Example Usage:
/// ```rust
/// let result = bin2num(b"hello world").unwrap();
/// let decoded = num2bin(result).unwrap();
/// assert_eq!(decoded, b"hello world");
/// ```

pub fn bin2num(data: &[u8]) -> Result<BigInt, String> {
    // Encode the binary data as a Base64 string
    let b64 = general_purpose::STANDARD.encode(data);
    println!("Base64 encoded string: {}", b64);

    // Convert the Base64 string to a number using the custom symbol set
    let result = str2num(&b64, DEFAULT_SYMBOLS);
    match &result {
        Ok(num) => println!("Decoded number from Base64: {}", num),
        Err(e) => println!("Error decoding Base64 to number: {}", e),
    }

    result
}

pub fn num2bin(number: BigInt) -> Result<Vec<u8>, DecodeError> {
    // Convert the number to a string using the custom symbol set
    let string = num2str(number, DEFAULT_SYMBOLS);
    println!("String representation of the number: {}", string);

    // Decode the string into a Base64-encoded byte vector
    match general_purpose::STANDARD.decode(&string) {
        Ok(decoded) => {
            println!("Decoded Base64 to bytes: {:?}", decoded);
            Ok(decoded)
        }
        Err(e) => {
            println!("Error decoding string to Base64: {}", e);
            Err(e)
        }
    }
}

#[cfg(test)]
mod bin_enc_tests {
    use super::*;

    #[test]
    fn num2bin_test() {
        let inter = bin2num(b"hello world").unwrap();
        let result = num2bin(inter).unwrap();
        assert_eq!(result, b"hello world")
    }

    #[test]
    fn bin2num_test_simple() {
        let result = bin2num(b"hello world").unwrap();
        println!("result: {}", result);
    }

    // Test encoding and decoding the same "hello world" text
    #[test]
    fn num2bin_test_simple() {
        let inter = bin2num(b"hello world").unwrap();
        let result = num2bin(inter).unwrap();
        assert_eq!(result, b"hello world");
    }

    // Test with an empty string
    #[test]
    fn bin2num_test_empty() {
        let result = bin2num(b"").unwrap();
        assert_eq!(result.to_string(), "0");  // Empty string should result in 0
    }

    // Test encoding and decoding a string with special characters
    #[test]
    fn bin2num_test_special_chars() {
        let special_str = b"@|<+-*/=>";
        let result = bin2num(special_str).unwrap();
        println!("Encoded and decoded result: {}", result);
    }

    // Test long string
    #[test]
    fn bin2num_test_long_string() {
        let long_str = b"This is a long string used to test encoding and decoding. This is a long string used to test encoding and decoding. This is a long string used to test encoding and decoding. This is a long string used to test encoding and decoding. This is a long string used to test encoding and decoding. This is a long string used to test encoding and decoding.";
        let result = bin2num(long_str).unwrap();
        let decoded = num2bin(result).unwrap();
        assert_eq!(decoded, long_str);  // Check if the decoded result matches the original input
    }

    // Test handling invalid Base64 (should trigger DecodeError)
    #[test]
    fn num2bin_test_invalid_base64() {
        let invalid_number = BigInt::from(123456789);
        let result = num2bin(invalid_number);
        assert!(result.is_err());  // Expect an error due to invalid Base64 format
    }

    // Test encoding and decoding with non-ASCII characters
    #[test]
    fn bin2num_test_non_ascii() {
        let non_ascii_str = "你好，世界"; // "Hello, world" in Chinese
        let result = bin2num(non_ascii_str.as_bytes()).unwrap();
        println!("Encoded and decoded result: {}", result);
    }

    // Test round-trip encoding and decoding of a string with numbers and symbols
    #[test]
    fn bin2num_test_numbers_and_symbols() {
        let mixed_str = b"1234!@#$%^&*()";
        let result = bin2num(mixed_str).unwrap();
        let decoded = num2bin(result).unwrap();
        assert_eq!(decoded, mixed_str);  // Check if the decoded result matches the original input
    }
}