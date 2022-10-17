// modules2.rs
// You can bring module paths into scopes and provide new names for them with the
// 'use' and 'as' keywords. Fix these 'use' statements to make the code compile.
// Execute `rustlings hint modules2` or use the `hint` watch subcommand for a hint.

// I AM NOT DONE

// values are by default private in rust, so to make things visible to the parent module, we need to mention the pub keyword
mod delicious_snacks {
    
    // the submodules is being referred by the names we have given after the 'as' keyword
    // the submodule will be accessed by using module::submodule
    // which means that we have to specify which mod to look for the particular submodule
    
    // pub use self::fruits::PEAR as nashpati;
    // pub use self::veggies::CUCUMBER as kheera;
    // pub use self::fruits::APPLE as seb;
    // pub use self::veggies::CARROT as gajar;
    pub const PASTA: & 'static str = "Pasta";

    pub mod fruits {
        pub const PEAR: &'static str = "Pear";
        pub const APPLE: &'static str = "Apple";
    }

    pub mod veggies {
        pub const CUCUMBER: &'static str = "Cucumber";
        pub const CARROT: &'static str = "Carrot";
    }

    pub mod pizza {

    }
}

// no need to use the pub keyword here bc nothing is being exported out of scope here

// pub use delicious_snacks::fruits::PEAR as nashpati;
// pub use delicious_snacks::veggies::CUCUMBER as kheera;
// pub use delicious_snacks::fruits::APPLE as seb;
// pub use delicious_snacks::veggies::CARROT as gajar;

// The following 'use' statements means that we are directly specifying the name for the submodule we wish to use after 'as' keyword
// the submod van be directly be accessed by using the name given 
// no mod::submod waali bakchodi

// use delicious_snacks::fruits::PEAR as nashpati;
// use delicious_snacks::veggies::CUCUMBER as kheera;
use delicious_snacks::fruits::APPLE as seb;
use delicious_snacks::veggies::CARROT as gajar;
use delicious_snacks::pizza;
use delicious_snacks::PASTA;

fn main() {
// cuz we declared the mod in the global scope, we can call it anywhere by the specified name
    println!(
        "favorite snacks: {} and {} yoo {}",
        seb,
        gajar,
        PASTA
    );
}
