//! Integration tests for Orbyte serialization and deserialization.

#[cfg(test)]
mod tests {
    use crate::{Deserialize, OrbyteError, Serialize};

    #[test]
    fn test_vec_u32() {
        let vec = vec![1_u32, 2, 3];
        let bytes = vec.serialize();
        assert_eq!(bytes, vec![3, 0, 0, 0, 1, 0, 0, 0, 2, 0, 0, 0, 3, 0, 0, 0]); // Length (3) + three u32s
        let deserialized = Vec::<u32>::deserialize(&bytes).unwrap();
        assert_eq!(deserialized, vec);
    }

    #[test]
    fn test_vec_string() {
        let vec = vec!["a".to_string(), "bc".to_string()];
        let bytes = vec.serialize();
        assert_eq!(bytes, vec![2, 0, 0, 0, 1, b'a', 2, b'b', b'c']); // Length (2) + two strings
        let deserialized = Vec::<String>::deserialize(&bytes).unwrap();
        assert_eq!(deserialized, vec);
    }

    #[test]
    fn test_invalid_bool() {
        let bytes = [2];
        let result = bool::deserialize(&bytes);
        assert_eq!(result, Err(OrbyteError::InvalidBool));
    }

    #[test]
    fn test_invalid_length() {
        let bytes = [0];
        let result = u16::deserialize(&bytes);
        assert_eq!(result, Err(OrbyteError::InvalidLength));
    }

    #[test]
    fn test_invalid_utf8() {
        let bytes = [3, 0xFF, 0xFF, 0xFF]; // Invalid UTF-8 sequence
        let result = String::deserialize(&bytes);
        assert!(matches!(result, Err(OrbyteError::InvalidUtf8(_))));
    }
}