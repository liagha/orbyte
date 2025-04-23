//! Serialization logic for converting Rust types into byte vectors.

/// Trait for types that can be serialized into a byte vector.
///
/// Implementors must define how to convert `Self` into a `Vec<u8>` representing
/// the serialized form.
pub trait Serialize {
    /// Serializes `self` into a byte vector.
    ///
    /// The output format is type-specific, typically using little-endian for numbers
    /// and length-prefixed UTF-8 for strings.
    fn serialize(&self) -> Vec<u8>;
}

impl Serialize for u8 {
    fn serialize(&self) -> Vec<u8> {
        vec![*self]
    }
}

impl Serialize for u16 {
    fn serialize(&self) -> Vec<u8> {
        self.to_le_bytes().to_vec()
    }
}

impl Serialize for u32 {
    fn serialize(&self) -> Vec<u8> {
        self.to_le_bytes().to_vec()
    }
}

impl Serialize for u64 {
    fn serialize(&self) -> Vec<u8> {
        self.to_le_bytes().to_vec()
    }
}

impl Serialize for i32 {
    fn serialize(&self) -> Vec<u8> {
        self.to_le_bytes().to_vec()
    }
}

impl Serialize for i64 {
    fn serialize(&self) -> Vec<u8> {
        self.to_le_bytes().to_vec()
    }
}

impl Serialize for f32 {
    fn serialize(&self) -> Vec<u8> {
        self.to_le_bytes().to_vec()
    }
}

impl Serialize for f64 {
    fn serialize(&self) -> Vec<u8> {
        self.to_le_bytes().to_vec()
    }
}

impl Serialize for bool {
    fn serialize(&self) -> Vec<u8> {
        vec![*self as u8]
    }
}

impl Serialize for char {
    fn serialize(&self) -> Vec<u8> {
        (*self as u32).serialize()
    }
}

impl Serialize for String {
    fn serialize(&self) -> Vec<u8> {
        let mut bytes = Vec::with_capacity(1 + self.len());
        bytes.push(self.len() as u8);
        bytes.extend(self.as_bytes());
        bytes
    }
}

impl<T: Serialize> Serialize for Vec<T> {
    fn serialize(&self) -> Vec<u8> {
        let mut bytes = (self.len() as u32).serialize();
        for item in self {
            bytes.extend(item.serialize());
        }
        bytes
    }
}

impl<T: Serialize> Serialize for Option<T> {
    fn serialize(&self) -> Vec<u8> {
        match self {
            None => vec![],
            Some(value) => value.serialize(),
        }
    }
}