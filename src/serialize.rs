pub trait Serialize {
    fn serialize(&self) -> Vec<u8>;
}

impl Serialize for u8 {
    fn serialize(&self) -> Vec<u8> {
        vec![*self]
    }
}

impl Serialize for i32 {
    fn serialize(&self) -> Vec<u8> {
        self.to_le_bytes().to_vec()
    }
}

impl Serialize for String {
    fn serialize(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.push(self.len() as u8);
        bytes.extend(self.as_bytes());
        bytes
    }
}

impl Serialize for Vec<u8> {
    fn serialize(&self) -> Vec<u8> {
        self.clone()
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