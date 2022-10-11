// move_semantics6.rs
// Execute `rustlings hint move_semantics6` or use the `hint` watch subcommand for a hint.
// You can't change anything except adding or removing references.

fn main() {
    let mut data = "Rust is great!".to_string();

    get_char(&data);

    string_uppercase(data);

    // this line will give an error because the ownership of the 'data' variable has been transfered to the
    // string_uppercase function and was dropped as nothinh was returned and no reference to the variable was used
    // hence, when the string_uppercase function finishes it's execution, the value of the data variable is dropped
    // println!("{}", data);
}

// only takes an immutable reference to the data variable and hence, the value of the variable is not dropped
// Should not take ownership
fn get_char(data: &String) -> char {
    data.chars().last().unwrap()
}

// the ownership of the data variable is tranfered to this function, and hence after the execution of this function the value of the data variable is dropped
// Should take ownership
fn string_uppercase(mut data: String) {
    data = data.to_uppercase();

    println!("{}", data);
}
