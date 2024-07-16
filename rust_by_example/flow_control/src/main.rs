#![allow(dead_code)]
#![allow(unused_variables)]

/// Flow Control
/// Rust by Example, Chapter 8
/// https://doc.rust-lang.org/rust-by-example/flow_control.html


fn main() {
    //~ if_else();
    //~ while_loops();
    //~ while_loops();
    //~ for_loops();
    //~ matching();
    //~ match_destructuring();
    //~ match_guards();
    //~ match_binding();
    if_let();
}


/// If/else
fn if_else() {
    let n = 50;
    if n < 0 {
        print!("{} is negative", n);
    } else if n > 0 {
        print!("{} is positive", n);
    } else {
        print!("Number is zero");
    }

    // If/else is an expression too?!
    let big_n =
        if n < 10 && n > -10 {
            println!(", and is a small number. Increase 10-fold!");
            n * 10
        } else {
            println!(", and is a big number, halve the number");
            n / 2
        };

    println!("{} -> {}", n, big_n);
}


/// Loops
fn loop_loops() {
    println!("Let's count until infinity!");
    let mut count = 0_u32;

    // Loops also expressions. Wild!
    let last = loop {
        if count == 3 {
            println!("three");
        } else {
            println!("{}", count);
        }

        count += 1;
        if count >= 20 {
            break count;
        }
    };

    println!("OK, {} is probably enough", last);
}


/// While
fn while_loops() {

    // While
    /////////
    let mut n = 1;
    while n <= 15 {
        if n % 15 == 0 {
            println!("fizzbuzz");
        } else if n % 3 == 0 {
            println!("fizz");
        } else if n % 5 == 0 {
            println!("buzz");
        } else {
            println!("{}", n);
        }
        n += 1;
    }
}


/// For
fn for_loops() {
    // For with a range
    for n in 2..101 {
        if n % 2 == 0 && n != 2 {
            continue;
        } else if n % 3 == 0 && n != 3{
            continue;
        } else if n % 5 == 0 && n != 5 {
            continue;
        } else if n % 7 == 0 && n != 7 {
            continue;
        }
        println!("{} is a prime number", n);
    }

    // For
    ///////

    // For with a container
    let names = vec!["Bob", "Frank", "Ferris"];

    // iter() borrows each element of the collection through each iteration.
    for name in names.iter() {
        match name {
            &"Ferris" => println!("It's {name}'s day off"),
            _ => println!("Hello {name}"),
        }
    }
    println!("{:?}", names);

    // into_iter() consumes the collection, but returns original content.
    // No need for reference in match, but collection left unusable.
    for name in names.into_iter() {
        match name {
            "Ferris" => println!("It's {name}'s day off"),
            _ => println!("Hello {name}"),
        }
    }

    // iter_mut() mutably borrows each element of the collection
    let mut names = vec!["Bob", "Frank", "Ferris"];
    for name in names.iter_mut() {
        *name = match name {
            &mut "Bob" => "Bobby",
            _ => "Hello Anon",
        }
    }
    println!("{:?}", names);
}


/// Match
fn matching() {
    let number = 13;
    println!("Tell me about {}", number);
    match number {
        1 => println!("One!"),
        2 | 3 | 5 | 7 | 11 | 13 => println!("This is a prime"),
        13..=19 => println!("A teen"),
        _ => println!("Ain't special"),
    }

    // Match is an expression too
    let boolean = true;
    let binary = match boolean {
        // The arms of a match must cover all the possible values
        false => 0,
        true => 1,
    };

    println!("{} -> {}", boolean, binary);
}


/// Destructuring in match
fn match_destructuring() {
    // Tuples
    //////////
    let triple = (3, -2, 4);
    println!("Tell me about {:?}", triple);
    match triple {
        (0, y, z) => println!("First is `0`, `y` is {:?}, and `z` is {:?}", y, z),
        (1, ..)  => println!("First is `1` and the rest doesn't matter"),
        (.., 2)  => println!("last is `2` and the rest doesn't matter"),
        (3, .., 4)  => println!("First is `3`, last is `4`, and the rest doesn't matter"),
        _ => (),
    }

    // Arrays and slices
    /////////////////////
    let array = [3, -2, 6];
    match array {
        [0, second, third] =>
            println!("array[0] = 0, array[1] = {}, array[2] = {}", second, third),

        [1, _, third] =>
            println!("array[0] = 1, array[2] = {} and array[1] was ignored", third),

        // Capture rest into their own array
        [first, tail @ ..] => println!("First element is {}, the rest are {:?}", first, tail),
    }

    // Enums
    /////////
    #[derive(Debug)]
    enum Colour {
        // Specified solely by their name.
        Red,
        Blue,
        Green,

        // Tie 3-tuple to name
        RGB(u32, u32, u32),
    }

    let colour = Colour::RGB(122, 17, 40);
    println!("What colour is it?");
    match colour {
        Colour::Red => println!("The color is Red!"),
        Colour::Blue => println!("The color is Blue!"),
        Colour::Green => println!("The color is Green!"),
        Colour::RGB(r, g, b) =>
            println!("Red: {}, green: {}, and blue: {}!", r, g, b),
    }

    // Pointers/ref
    ////////////////
    let reference = &4;                 // Reference of type `i32`
    match reference {
        // If `reference` is pattern matched against `&val`, it results
        // in a comparison like: `&i32` == `&val`
        &val => println!("Got a value via destructuring: {:?}", val),
    }

    // To avoid the `&`, you dereference before matching.
    match *reference {
        val => println!("Got a value via dereferencing: {:?}", val),
    }

    // Structs
    ///////////
    #[derive(Debug)]
    struct Foo {
        x: (u32, u32),
        y: u32,
    }

    let foo = Foo { x: (3, 2), y: 3 };
    match foo {
        Foo { x: (1, b), y } =>
            println!("First of x is 1, b={},  y={} ", b, y),

        // You can rename variables and order is not important
        Foo { y: 2, x: i } =>
            println!("y is 2, i = {:?}", i),

        Foo { y, .. } => println!("y = {}, we don't care about x", y),
    }

    // You do not need a match block to destructure structs:
    let faa = Foo { x: (1, 2), y: 3 };
    let Foo { x : x0, y: y0 } = faa;
    println!("Outside: x0 = {x0:?}, y0 = {y0}");

    // Destructuring works with nested structs as well:
    struct Bar {
        foo: Foo,
    }

    let bar = Bar { foo: faa };
    let Bar { foo: Foo { x: nested_x, y: nested_y } } = bar;
    println!("Nested: nested_x = {nested_x:?}, nested_y = {nested_y:?}");
}


/// Match guards
fn match_guards() {
    enum Temperature {
        Celsius(i32),
        Fahrenheit(i32),
    }

    let temperature = Temperature::Celsius(30);

    match temperature {
        Temperature::Celsius(t) if t > 30 => println!("{}C is above 30 Celsius", t),
        Temperature::Celsius(t) => println!("{}C is equal to or below 30 Celsius", t),
        Temperature::Fahrenheit(t) if t > 86 => println!("{}F is above 86 Fahrenheit", t),
        Temperature::Fahrenheit(t) => println!("{}F is equal to or below 86 Fahrenheit", t),
    }

    // Note that guards defeat the compiler's ability to check for every case,
    // so we have to manually mark the guard as unreachable.
    let number: u8 = 4;
    match number {
        i if i == 0 => println!("Zero"),
        i if i > 0 => println!("Greater than zero"),
        _ => unreachable!("Should never happen."),
    }
}


/// Match binding
fn match_binding () {
    // `match` provides the @ sigil for binding values to names
    fn age() -> u32 {
        48
    }

    match age() {
        0 => println!("I haven't celebrated my first birthday yet"),
        n @ 1 ..= 12 => println!("I'm a child of age {:?}", n),
        n @ 13 ..= 19 => println!("I'm a teen of age {:?}", n),
        n => println!("I'm an old person of age {:?}", n),
    }

    // You can also use binding to "destructure" enum variants, such as Option
    fn some_number() -> Option<u32> {
        Some(42)
    }

    match some_number() {
        // Match against `Some(42)` and asign value to n
        Some(n @ 42) => println!("The Answer: {}!", n),

        // Match every other valid `Some`
        Some(n) => println!("Not interesting... {}", n),

        // Match `None` variant
        _ => (),
    }
}


/// if let
fn if_let() {
    // When matching enums match can be awkward:
    let optional = Some(7);             // type `Option<i32>`
    match optional {
        Some(i) => println!("This is a really long string and `{:?}`", i),
        _ => {},
    };

    // "If let" handles a single match more cleanly
    if let Some(number) = optional {
        println!("Matched {:?}!", number);
    }

    // All have type `Option<i32>`
    let number = Some(7);
    let letter: Option<i32> = None;
    let emoticon: Option<i32> = None;

    // if `let` destructures `number` into `Some(i)`, evaluate the block.
    if let Some(i) = number {
        println!("Matched {:?}!", i);
    }

    // If you need to specify a failure, use an else:
    if let Some(i) = letter {
        println!("Matched {:?}!", i);
    } else {
        println!("Didn't match a number. Let's go with a letter!");
    }

    // Provide an altered failing condition.
    let i_like_letters = false;
    if let Some(i) = emoticon {
        println!("Matched {:?}!", i);
    } else if i_like_letters {
        println!("Didn't match a number. Let's go with a letter!");
    } else {
        println!("I don't like letters. Let's go with an emoticon :)!");
    }

    // "if let" can be used to match any enum value
    enum Foo {
        Bar,
        Baz,
        Qux(u32)
    }

    let a = Foo::Bar;
    let b = Foo::Baz;
    let c = Foo::Qux(100);

    if let Foo::Bar = a {
        println!("a is foobar");
    }
}

