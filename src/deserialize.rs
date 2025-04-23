//! Deserialization logic for converting byte slices into Rust types.

use crate::error::OrbyteError;
use crate::Serialize;

/// Trait for types that can be deserialized from a byte slice.
///
/// Implementors must define how to convert a byte slice into an instance of `Self`,
/// returning a `Result` that indicates success or an error.
pub trait Deserialize: Sized {
    /// Deserializes a byte slice into an instance of `Self`.
    ///
    /// # Errors
    ///
    /// Returns `Err` if the byte slice is invalid or insufficient for deserialization.
    fn deserialize(bytes: &[u8]) -> Result<Self, OrbyteError>;
}

impl Deserialize for u8 {
    fn deserialize(bytes: &[u8]) -> Result<Self, OrbyteError> {
        bytes.get(0).copied().ok_or(OrbyteError::InvalidLength)
    }
}

impl Deserialize for u16 {
    fn deserialize(bytes: &[u8]) -> Result<Self, OrbyteError> {
        Ok(u16::from_le_bytes(
            bytes
                .get(0..2)
                .ok_or(OrbyteError::InvalidLength)?
                .try_into()
                .map_err(|_| OrbyteError::InvalidLength)?,
        ))
    }
}

impl Deserialize for u32 {
    fn deserialize(bytes: &[u8]) -> Result<Self, OrbyteError> {
        Ok(u32::from_le_bytes(
            bytes
                .get(0..4)
                .ok_or(OrbyteError::InvalidLength)?
                .try_into()
                .map_err(|_| OrbyteError::InvalidLength)?,
        ))
    }
}

impl Deserialize for u64 {
    fn deserialize(bytes: &[u8]) -> Result<Self, OrbyteError> {
        Ok(u64::from_le_bytes(
            bytes
                .get(0..8)
                .ok_or(OrbyteError::InvalidLength)?
                .try_into()
                .map_err(|_| OrbyteError::InvalidLength)?,
        ))
    }
}

impl Deserialize for i32 {
    fn deserialize(bytes: &[u8]) -> Result<Self, OrbyteError> {
        Ok(i32::from_le_bytes(
            bytes
                .get(0..4)
                .ok_or(OrbyteError::InvalidLength)?
                .try_into()
                .map_err(|_| OrbyteError::InvalidLength)?,
        ))
    }
}

impl Deserialize for i64 {
    fn deserialize(bytes: &[u8]) -> Result<Self, OrbyteError> {
        Ok(i64::from_le_bytes(
            bytes
                .get(0..8)
                .ok_or(OrbyteError::InvalidLength)?
                .try_into()
                .map_err(|_| OrbyteError::InvalidLength)?,
        ))
    }
}

impl Deserialize for f32 {
    fn deserialize(bytes: &[u8]) -> Result<Self, OrbyteError> {
        Ok(f32::from_le_bytes(
            bytes
                .get(0..4)
                .ok_or(OrbyteError::InvalidLength)?
                .try_into()
                .map_err(|_| OrbyteError::InvalidLength)?,
        ))
    }
}

impl Deserialize for f64 {
    fn deserialize(bytes: &[u8]) -> Result<Self, OrbyteError> {
        Ok(f64::from_le_bytes(
            bytes
                .get(0..8)
                .ok_or(OrbyteError::InvalidLength)?
                .try_into()
                .map_err(|_| OrbyteError::InvalidLength)?,
        ))
    }
}

impl Deserialize for bool {
    fn deserialize(bytes: &[u8]) -> Result<Self, OrbyteError> {
        match bytes.get(0).ok_or(OrbyteError::InvalidLength)? {
            0 => Ok(false),
            1 => Ok(true),
            _ => Err(OrbyteError::InvalidBool),
        }
    }
}

impl Deserialize for char {
    fn deserialize(bytes: &[u8]) -> Result<Self, OrbyteError> {
        let code_point = u32::deserialize(bytes)?;
        std::char::from_u32(code_point).ok_or(OrbyteError::InvalidChar)
    }
}

impl Deserialize for String {
    fn deserialize(bytes: &[u8]) -> Result<Self, OrbyteError> {
        let length = *bytes.get(0).ok_or(OrbyteError::InvalidLength)? as usize;
        let str_bytes = bytes
            .get(1..1 + length)
            .ok_or(OrbyteError::InvalidLength)?;
        String::from_utf8(str_bytes.to_vec())
            .map_err(OrbyteError::InvalidUtf8)
    }
}

impl<T: Deserialize + Serialize> Deserialize for Vec<T> {
    fn deserialize(bytes: &[u8]) -> Result<Self, OrbyteError> {
        let length = u32::deserialize(bytes)? as usize;
        let mut offset = 4; // u32 length prefix
        let mut result = Vec::with_capacity(length);
        for _ in 0..length {
            let item = T::deserialize(&bytes[offset..])?;
            offset += item.serialize().len();
            result.push(item);
        }
        Ok(result)
    }
}

impl<T: Deserialize> Deserialize for Option<T> {
    fn deserialize(bytes: &[u8]) -> Result<Self, OrbyteError> {
        if bytes.is_empty() {
            Ok(None)
        } else {
            Ok(Some(T::deserialize(bytes)?))
        }
    }
}