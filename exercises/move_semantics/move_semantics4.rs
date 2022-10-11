// move_semantics4.rs
// Refactor this code so that instead of passing `vec0` into the `fill_vec` function,
// the Vector gets created in the function itself and passed back to the main
// function.
// Execute `rustlings hint move_semantics4` or use the `hint` watch subcommand for a hint.

fn main() {
    
    // a new vec is being created in the fill_vec function and then it's value is put into the vec1 variable
    let mut vec1 = fill_vec();
    
    println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);
    
    vec1.push(88);
    
    println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);
}

// `fill_vec()` no longer takes `vec: Vec<i32>` as argument
// here we are creating a new vec in the fill_vec function and returning it directly, hence there is no need to explain
fn fill_vec() -> Vec<i32> {
    let mut vec0 = Vec::new();
    // let mut vec = vec;

    vec0.push(22);
    vec0.push(44);
    vec0.push(66);

    vec0
}
