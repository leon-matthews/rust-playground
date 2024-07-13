
fn main() {
    // Basic if/else
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
