use orbyte::{Orbyte, Serialize, Deserialize};

#[derive(Orbyte)]
enum Message {
    Text(String),
    Number(u32),
    Empty,
}

fn main() {
    let message = Message::Text("Hello".to_string());
    let bytes = message.serialize();
    println!("Serialized: {:?}", bytes);

    let deserialized = Message::deserialize(&bytes).unwrap();
    match deserialized {
        Message::Text(s) => println!("Deserialized: Text({})", s),
        _ => println!("Unexpected variant"),
    }
}