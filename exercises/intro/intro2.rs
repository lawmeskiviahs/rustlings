// intro2.rs
// Make the code print a greeting to the world.
// Execute `rustlings hint intro2` or use the `hint` watch subcommand for a hint.

// How to parse a string into a variable
fn main() {
    // let m= String::from("World");
    let m: String = String::from("World");
    // .to_string();
    println!("Hello {}!", m);
}
