// enums3.rs
// Address all the TODOs to make the tests pass!
// Execute `rustlings hint enums3` or use the `hint` watch subcommand for a hint.

// logs not getting printed here

// an enum with different variants is created
enum Message {
    ChangeColor ((u8, u8, u8)),
    Move (Point),
    Echo (String),
    Quit
}

struct Point {
    x: u8,
    y: u8,
}

struct State {
    color: (u8, u8, u8),
    position: Point,
    quit: bool,
}

impl State {
    fn change_color(&mut self, color: (u8, u8, u8)) {
        self.color = color;
        println!("{:?}", color)
    }

    fn quit(&mut self) {
        self.quit = true;
    }

    fn echo(&self, s: String) {
        println!("{}", s);
    }

    fn move_position(&mut self, p: Point) {
        self.position = p;
    }

    // we can write some functionality for each variant of the enum 
    // the values passes inside (if any) can be given any variable name
    // these variables can be used in any way desired
    fn process(&mut self, message: Message) {
        match message{
            Message::ChangeColor(color) => self.change_color(color),
            // Message::ChangeColor(color) => {println!("{:?}", color)}, // I want to check if this is possible, however I think it might be because we can write seperate code blocks for every variant and inside that block any statement or expression can be added
            Message::Echo(s) => self.echo(s),
            Message::Move(p) => self.move_position(p),
            Message::Quit => self.quit(),
        }
    }
}

fn main(){
    println!("abc");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_match_message_call() {
        let mut state = State {
            quit: false,
            position: Point { x: 0, y: 0 },
            color: (0, 0, 0),
        };
        println!("abc");
        state.process(Message::ChangeColor((255, 0, 255)));
        state.process(Message::Echo(String::from("hello world")));
        state.process(Message::Move(Point { x: 10, y: 15 }));
        state.process(Message::Quit);

        assert_eq!(state.color, (255, 0, 255));
        assert_eq!(state.position.x, 10);
        assert_eq!(state.position.y, 15);
        assert_eq!(state.quit, true);
    }
}
