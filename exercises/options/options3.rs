// options3.rs
// Execute `rustlings hint options3` or use the `hint` watch subcommand for a hint.

// I AM NOT DON

struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let y: Option<Point> = Some(Point { x: 100, y: 200 });
    // let y: Option<Point> = None;

    // references to option type variables can be passed into the match statement and it still works fine somehow
    match y {
        // we can pass references into match statements by using the ref keyword
        // the ref keyword gives a reference to the unpacked value
        Some(ref p) => println!("Co-ordinates are {},{} ", p.x, p.y),
        // for any default, we may use _
        _ => println!("no match"),
    }
    y; // Fix without deleting this line.
}
