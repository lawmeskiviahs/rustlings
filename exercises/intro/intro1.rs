// intro1.rs
// About this `I AM NOT DONE` thing:
// We sometimes encourage you to keep trying things on a given exercise, even
// after you already figured it out. If you got everything working and feel
// ready for the next exercise, remove the `I AM NOT DONE` comment below.
// Execute `rustlings hint intro1` or use the `hint` watch subcommand for a hint.
//
// If you're running this using `rustlings watch`: The exercise file will be reloaded
// when you change one of the lines below! Try adding a `println!` line, or try changing
// what it outputs in your terminal. Try removing a semicolon and see what happens!

//I AM NOT DON

use std::collections::HashMap;

#[derive(Hash, PartialEq, Eq, Debug)]
enum Fruit {
    Apple,
    Banana,
    Mango,
    Lychee,
    Pineapple,
    Pear,
}

fn get_fruit_basket() -> HashMap<Fruit, u32> {
    let mut basket = HashMap::<Fruit, u32>::new();
    basket.insert(Fruit::Apple, 4);
    basket.insert(Fruit::Mango, 2);
    basket.insert(Fruit::Lychee, 5);

    basket
}

fn fruit_basket(basket: &mut HashMap<Fruit, u32>) {
    let fruit_kinds = vec![
        Fruit::Apple,
        Fruit::Banana,
        Fruit::Mango,
        Fruit::Lychee,
        Fruit::Pineapple,
        Fruit::Pear,
    ];

    for fruit in fruit_kinds {
        println!("{:?}", fruit);
        if basket.get(&fruit).unwrap_or(&0) == &0 {
            basket.insert(fruit, 2);
        }
    }
}

fn main() {
    let mut basket = get_fruit_basket();
    fruit_basket(&mut basket);
    println!("{}", basket.len());
    println!("{}", basket.values().sum::<u32>());
}


// fn trim_me(mut input: &str) {
//     // TODO: Remove whitespace from both ends of a string!
//         // let mut input = input;
//         // let a = input.trim().to_string();
//         // *input = input[..];
//         // input.push_str("hola")
//         // input = x;
//         // println!("{}x", input.to_string());
//         // todo!()
// }

// fn compose_me(input: &str) -> String {
//     // TODO: Add " world!" to the string! There's multiple ways to do this!
//     input.to_string() + " world!"
// }

// fn replace_me(input: &str) -> String {
//     // TODO: Replace "cars" in the string with "balloons"!
//     input.to_string()
// }

// fn main(){
//     let mut x = "Hello!     ";
//         let y=trim_me(x);
//         println!("{}x", x);
// }



// introoo
// fn main() {
//     println!("Hello and");
//     println!(r#"       welcome to...                      "#);
//     println!(r#"                 _   _ _                  "#);
//     println!(r#"  _ __ _   _ ___| |_| (_)_ __   __ _ ___  "#);
//     println!(r#" | '__| | | / __| __| | | '_ \ / _` / __| "#);
//     println!(r#" | |  | |_| \__ \ |_| | | | | | (_| \__ \ "#);
//     println!(r#" |_|   \__,_|___/\__|_|_|_| |_|\__, |___/ "#);
//     println!(r#"                               |___/      "#);
//     println!();
//     println!("This exercise compiles successfully. The remaining exercises contain a compiler");
//     println!("or logic error. The central concept behind Rustlings is to fix these errors and");
//     println!("solve the exercises. Good luck!");
//     println!();
//     println!("The source for this exercise is in `exercises/intro/intro1.rs`. Have a look!");
//     println!("Going forward, the source of the exercises will always be in the success/failure output.");
//     println!("Hello heelo hello");
// }


// fn main() {
//     let mut vec0 = Vec::new();
//     // let vec0 = Vec::new();

//     // let mut vec1 = fill_vec(vec0);
//     let mut vec1 = fill_vec(&mut vec0);

//     // Do not change the following line!
//     println!("{} has length {} content `{:?}`", "vec0", vec0.len(), vec0);

//     vec1.push(88);

//     println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);
// }

// fn fill_vec(vec: &mut Vec<i32>) -> Vec<i32> {
// // fn fill_vec(vec: Vec<i32>) -> Vec<i32> {
//     let mut vec = vec;

//     vec.push(22);
//     vec.push(44);
//     vec.push(66);

//     // vec
//     vec.to_vec()
// }
