
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
            //~ println!("{}", count);
        }

        count += 1;
        if count >= 20 {
            break count;
        }
    };

    println!("OK, {} is probably enough", last);
}
