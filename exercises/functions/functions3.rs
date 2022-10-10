// functions3.rs
// Execute `rustlings hint functions3` or use the `hint` watch subcommand for a hint.

// functions that require an input must be provided one, otherwise they throw errors??
fn main() {
    call_me(5);
}

fn call_me(num: u32) {
    for i in 0..num {
        println!("Ring! Call number {}", i + 1);
    }
}
