use orbyte::{Orbyte, Serialize, Deserialize};

#[derive(Orbyte)]
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let point = Point { x: 42, y: -17 };
    let bytes = point.serialize();
    println!("Serialized: {:?}", bytes);

    let deserialized = Point::deserialize(&bytes).unwrap();
    println!("Deserialized: x = {}, y = {}", deserialized.x, deserialized.y);
}