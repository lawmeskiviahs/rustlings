// options2.rs
// Execute `rustlings hint options2` or use the `hint` watch subcommand for a hint.

// I AM NOT DON

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_option() {
        let target = "rustlings";
        let optional_target = Some(target);
        let optional_target2 = Some(optional_target);

        // TODO: Make this an if let statement whose value is "Some" type

        // if 'let' destructures 'optional target' into 'Some(word)' execute the block 
        // if option_target is of type Some(word) then execute the following block
        // we'd have to unwrap all the options to get the value out of it
        if let Some(Some(word)) = optional_target2 {
            assert_eq!(word, target);
        } // else if {
            // This else if block will be executed in case the above if statement fails
        // } else {
            // this else block would be trated as default in case all of the above if and else ifs' fail
        // }
    }

    #[test]
    fn layered_option() {
        let mut range = 10;
        let mut optional_integers: Vec<Option<i8>> = Vec::new();
        for i in 0..(range + 1) {
            optional_integers.push(Some(i));
        }

        // TODO: make this a while let statement - remember that vector.pop also adds another layer of Option<T>
        // You can stack `Option<T>`'s into while let and if let
        while let Some(integer) = optional_integers.pop() {
            assert_eq!(integer.unwrap(), range);
            range -= 1;
        }
    }
}
