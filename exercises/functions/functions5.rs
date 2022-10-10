// functions5.rs
// Execute `rustlings hint functions5` or use the `hint` watch subcommand for a hint.

// one liner functions don't require semicolons 
// this can also be done using closures
fn main() {
    let answer = square(3);  // using function 
    let answer = |num:u32|num*num;  //using closure
    println!("The square of 3 is {}", answer(3));   // closures don't need {:?} debugger
}

fn square(num: i32) -> i32 {
    num * num
}
    