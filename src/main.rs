use orbyte::Deserialize;
use orbyte::Orbyte;
use orbyte::Serialize;

#[derive(Orbyte, Clone, PartialEq, Debug)]
pub struct ChatMessage {
    pub sender: String,

    pub content: Option<Content>,

    pub timestamp: Option<i32>,
}

#[derive(Orbyte, Clone, PartialEq, Debug)]
pub enum Content {
    Text(String),

    File(FileData),

    Signal(u8),
}

#[derive(Orbyte, Clone, PartialEq, Debug)]
pub enum Content3 {
    Text,

    File,

    Signal,
}

#[derive(Orbyte, Clone, PartialEq, Debug)]
pub struct FileData {
    pub data: Vec<u8>,

    pub name: String,
}

impl std::fmt::Display for Content {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Content::Text(text) => write!(f, "{}", text),
            Content::File(_) => write!(f, "[File content]"),
            Content::Signal(code) => write!(f, "Signal: code {}", code),
        }
    }
}

fn main() {
    let signal = ChatMessage {
        sender: "ali".to_string(),
        content: Some(Content::Signal(0)),
        timestamp: Some(100000000),
    };

    let empty = ChatMessage {
        sender: "ali".to_string(),
        content: None,
        timestamp: Some(100000000),
    }
    .serialize();

    let empty2 = ChatMessage {
        sender: "ali".to_string(),
        content: Some(Content::Signal(0)),
        timestamp: None,
    }
    .serialize();

    let empty3 = ChatMessage {
        sender: "ali".to_string(),
        content: None,
        timestamp: None,
    }
    .serialize();

    let bytes = signal.serialize();
    let signal_bytes = Content::Signal(0).serialize();
    println!("Bytes Size: {}", bytes.len());
    println!("Data Size: {}", empty.len());
    println!("Data2 Size: {}", empty2.len());
    println!("Data3 Size: {}", empty3.len());
    println!("Raw Text Size: {}", "ali".to_string().as_bytes().len());
    println!("Signal Size: {}", signal_bytes.len());
    let deserialized_person = ChatMessage::deserialize(&bytes).unwrap();

    println!("{:?}", deserialized_person);
}
