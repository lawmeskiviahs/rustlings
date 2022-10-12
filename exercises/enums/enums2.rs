// enums2.rs
// Execute `rustlings hint enums2` or use the `hint` watch subcommand for a hint.
// we can define structs and tuples inside the enums 
// possible values of the enum-data-type can be saved in enums
#[derive(Debug)]
enum Message {
    Move { x:u32, y: u32 },
    Echo (String),
    ChangeColor (u32, u32, u32),
    Quit,
    //fn Color (String), //can't be done now (by me) for some reason
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
