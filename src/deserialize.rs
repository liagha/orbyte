pub trait Deserialize: Sized {
    fn deserialize(bytes: &[u8]) -> Option<Self>;
}

impl Deserialize for u8 {
    fn deserialize(bytes: &[u8]) -> Option<Self> {
        bytes.get(0).copied()
    }
}

impl Deserialize for u16 {
    fn deserialize(bytes: &[u8]) -> Option<Self> {
        Some(u16::from_le_bytes(bytes.get(0..2)?.try_into().ok()?))
    }
}

impl Deserialize for u32 {
    fn deserialize(bytes: &[u8]) -> Option<Self> {
        Some(u32::from_le_bytes(bytes.get(0..4)?.try_into().ok()?))
    }
}

impl Deserialize for u64 {
    fn deserialize(bytes: &[u8]) -> Option<Self> {
        Some(u64::from_le_bytes(bytes.get(0..8)?.try_into().ok()?))
    }
}

impl Deserialize for i32 {
    fn deserialize(bytes: &[u8]) -> Option<Self> {
        Some(i32::from_le_bytes(bytes.get(0..4)?.try_into().ok()?))
    }
}

impl Deserialize for i64 {
    fn deserialize(bytes: &[u8]) -> Option<Self> {
        Some(i64::from_le_bytes(bytes.get(0..8)?.try_into().ok()?))
    }
}

impl Deserialize for f32 {
    fn deserialize(bytes: &[u8]) -> Option<Self> {
        Some(f32::from_le_bytes(bytes.get(0..4)?.try_into().ok()?))
    }
}

impl Deserialize for f64 {
    fn deserialize(bytes: &[u8]) -> Option<Self> {
        Some(f64::from_le_bytes(bytes.get(0..8)?.try_into().ok()?))
    }
}

impl Deserialize for bool {
    fn deserialize(bytes: &[u8]) -> Option<Self> {
        match bytes.get(0)? {
            0 => Some(false),
            1 => Some(true),
            _ => None,
        }
    }
}

impl Deserialize for char {
    fn deserialize(bytes: &[u8]) -> Option<Self> {
        let code_point = u32::deserialize(bytes)?;
        std::char::from_u32(code_point)
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