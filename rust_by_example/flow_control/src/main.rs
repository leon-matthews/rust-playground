
/// Flow Control
/// Rust by Example, Chapter 8
/// https://doc.rust-lang.org/rust-by-example/flow_control.html


fn main() {
    // If/else
    ///////////

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


    // Loops also expressions. Wild!
    /////////////////////////////////
    println!("Let's count until infinity!");

    let mut count = 0_u32;
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

    // TODO: For with a container
}
