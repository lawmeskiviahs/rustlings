// strings3.rs
// Execute `rustlings hint strings3` or use the `hint` watch subcommand for a hint.

// I AM NOT DON
// Strings are stored on stack as a variable comprising of 3 feilds,
// 1. A pointer to the byte data
// 2. Space used
// 3. Space allocated
// This byte data can always be changed as it is a pointer to the heap
// the size of a string always stays constant on the stack, hence it's size is always known at compile time
fn trim_string(input: &mut String) {
    // TODO: Remove whitespace from both ends of a string!
    *input = input.trim().to_string();
}

// We cannot change the size of any data allocated on the stack in rust
// the compiler needs to know the size of data at compile time to make sure that no change in size was made
// str is stored on the stack and hence the size of a str cannot be changed, so the compiler does not allow us to change the value of a str
// str is generally used as a reference (&str)
// str's size is not known at compile time cuz it is already a reference, so there is no way for the compiler to guess it's size
// hence, the compiler does not let us mutate an &str no matter what
// fn trim_str(input : &mut str) {
//     // let mut input = input;
//     input.trim();
// }

fn compose_me(input: &str) -> String {
    // TODO: Add " world!" to the string! There's multiple ways to do this!
    // let x_trim_string = trim_string(&x_string);
        // let y_trim_string = trim_string(y_string);
        // let z_trim_string = trim_string(z_string);
    // concatenating a string at the end of an owned string and returning the result
    // input.to_string() + " world!"

    // using the format macro to return strings
    format!("{} world!", input)
}


fn replace_me(input: &str) -> String {
    // TODO: Replace "cars" in the string with "balloons"!
    input.to_string().replace("cars", "balloons")
}

// fn main(){
    //     let mut x = "Hello!     ";
    //         trim_str(x);
    // }
    
    #[cfg(test)]
    mod tests {
        use super::*;
        
        #[test]
        fn trim_a_string() {
        
        // let x_str = "Hello!     ";
        // let y_str = "  What's up!";
        // let z_str = "   Hola!  ";
        // let x_trim_str = trim_str(&mut *x_str);
        // let y_trim_str = trim_str(&mut *y_str);
        // let z_trim_str = trim_str(&mut *z_str);

        let mut x_string = "Hello!     ".to_string();
        let mut y_string = "  What's up!".to_string();
        let mut z_string = "   Hola!  ".to_string();

        trim_string(&mut x_string);
        trim_string(&mut y_string);
        trim_string(&mut z_string);
        assert_eq!(x_string, "Hello!");
        assert_eq!(y_string, "What's up!");
        assert_eq!(z_string, "Hola!");

    }

    #[test]
    fn compose_a_string() {
        assert_eq!(compose_me("Hello"), "Hello world!");
        assert_eq!(compose_me("Goodbye"), "Goodbye world!");
    }

    #[test]
    fn replace_a_string() {
        assert_eq!(replace_me("I think cars are cool"), "I think balloons are cool");
        assert_eq!(replace_me("I love to look at cars"), "I love to look at balloons");
    }
}
