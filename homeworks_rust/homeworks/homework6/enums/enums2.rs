// enums2.rs
// Make me compile! Execute `rustlings hint enums2` for hints!

#[derive(Debug)]
enum Message {
    // TODO: define the different variants used below
    Move { x: i32, y: i32 },                // Variant with named fields
    Echo(String),                           // Variant with a single field
    ChangeColor(u8, u8, u8),                 // Variant with multiple fields
    Quit,                                   // Unit variant (no fields)
}

impl Message {
    fn call(&self) {
        println!("{:?}", &self);
    }
}

fn main() {
    let messages = [
        Message::Move { x: 10, y: 30 },
        Message::Echo(String::from("hello world")),
        Message::ChangeColor(200, 255, 255),
        Message::Quit,
    ];

    for message in &messages {
        message.call();
    }
}
