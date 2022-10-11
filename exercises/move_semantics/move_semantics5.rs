// move_semantics5.rs
// Make me compile only by reordering the lines in `main()`, but without
// adding, changing or removing any of them.
// Execute `rustlings hint move_semantics5` or use the `hint` watch subcommand for a hint.

fn main() {
    // a mutable variable x is created
    let mut x = 100;

    // y is give a mutable reference to the variable x
    let y = &mut x;
    // let mut y = x;

    // in order to change the value of x using the variable y, we de-reference the variable y 
    *y += 100;

    // println!("x = {}", x);
    // println!("y = {}", y);

    // now, we borrow the mutable reference to the variable x into z
    // because y is a reference to the variable x, it can be moved into another variable and this leads to y dropping the reference and giving it to z
    // let z = &mut x;
    let z = y;

    // this line gives us an error because we are tryint to use the value of y after moving it to z
    // *y += 100;

    // to change the value of 
    *z += 1000;
    assert_eq!(x, 1200);
}
