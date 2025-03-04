pub trait Deserialize: Sized {
    fn deserialize(bytes: &[u8]) -> Option<Self>;
}

impl Deserialize for u8 {
    fn deserialize(bytes: &[u8]) -> Option<Self> {
        bytes.get(0).copied()
    }
}

impl Deserialize for i32 {
    fn deserialize(bytes: &[u8]) -> Option<Self> {
        Some(i32::from_le_bytes(bytes.get(0..4)?.try_into().ok()?))
    }
}

impl Deserialize for String {
    fn deserialize(bytes: &[u8]) -> Option<Self> {
        let length = *bytes.get(0)? as usize;
        let str_bytes = bytes.get(1..1 + length)?;
        Some(String::from_utf8(str_bytes.to_vec()).ok()?)
    }
}

impl Deserialize for Vec<u8> {
    fn deserialize(bytes: &[u8]) -> Option<Self> {
        if bytes.is_empty() {
            None
        } else {
            Some(Self::deserialize(bytes)?)
        }
    }
}

impl<T: Deserialize> Deserialize for Option<T> {
    fn deserialize(bytes: &[u8]) -> Option<Self> {
        if bytes.is_empty() {
            Some(None)
        } else {
            Some(Some(T::deserialize(bytes)?))
        }
    }
}