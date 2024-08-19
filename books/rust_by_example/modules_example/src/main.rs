#![allow(dead_code)]


fn main() {
    my_mod::indirect_access();
    my_mod::nested::indirect_access();
}


mod my_mod {
    fn private_function() {
        println!("called `my_mod::private_function()`");
    }

    pub fn public_function() {
        println!("called `my_mod::function()`");
    }

    pub fn indirect_access() {
        print!("called `my_mod::indirect_access()`, that\n> ");
        private_function();
    }

    pub mod nested {
        pub fn function() {
            println!("called `my_mod::nested::function()`");
        }

        fn private_function() {
            println!("called `my_mod::nested::private_function()`");
        }

        pub fn indirect_access() {
            print!("called `my_mod::nested::indirect_access()`, that\n> ");
            private_function();
        }
    }
}
