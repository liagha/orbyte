trait ToFormattedString {
    fn to_formatted_string(&self) -> String;
}

impl ToFormattedString for String {
    fn to_formatted_string(&self) -> String {
        format!("\"{}\"", self)
    }
}

impl ToFormattedString for i32 {
    fn to_formatted_string(&self) -> String {
        self.to_string()
    }
}

impl ToFormattedString for bool {
    fn to_formatted_string(&self) -> String {
        self.to_string()
    }
}

fn main() {
    let test = InnerStruct {
        z: "test1".to_string(),
        w: "test2".to_string(),
    };

    let test2 = InnerStruct {
        z: "test1".to_string(),
        w: "test2".to_string(),
    };

    let test3 = InnerEnum::Integer(2);
    let test4 = InnerEnum::Str("test".to_string());

    let test5 = OuterStruct {
        first_inner: test,
        second_inner: test2,
        first_enum: test3,
        second_enum: test4,
    };

    println!("{}", test5.to_formatted_string());
}

#[derive(orbyte::ToFormattedString)]
struct InnerStruct {
    z: String,
    w: String,
}

#[derive(orbyte::ToFormattedString)]
struct OuterStruct {
    first_inner: InnerStruct,
    second_inner: InnerStruct,
    first_enum: InnerEnum,
    second_enum: InnerEnum,
}

#[derive(orbyte::ToFormattedString)]
enum InnerEnum {
    Str(String),
    Integer(i32),
}