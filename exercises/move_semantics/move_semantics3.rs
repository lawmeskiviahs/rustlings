// move_semantics3.rs
// Make me compile without adding new lines-- just changing existing lines!
// (no lines with multiple semicolons necessary!)
// Execute `rustlings hint move_semantics3` or use the `hint` watch subcommand for a hint.

// reference and borrowing
fn main() {
    // if a value is to be borrowed mutably it must be set to mutable first
    let mut vec0 = Vec::new();

    // mutable reference is passed as &mut
    // & -> reference, mut -> mutable, hence mutable reference
    // if we need to pass an immutable reference, we can do this directly by using &
    let mut vec1 = fill_vec(&mut vec0);

    println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);

    vec1.push(88);

    println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);
}

// a borrowed value must be specified as borrowed while recieving the borrow
fn fill_vec(vec: &mut Vec<i32>) -> Vec<i32> {
    vec.push(22);
    vec.push(44);
    vec.push(66);

    // .to_vec() to convert the mutable vec reference into a vector to return it to the calling function
    vec.to_vec()
}
