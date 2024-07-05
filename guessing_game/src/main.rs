
use rand::Rng;
use std::cmp::Ordering;
use std::io;
use std::io::Write;


fn main() {
    println!("Guess the number!");
    let secret_number = rand::thread_rng().gen_range(1..=1000);

    loop {
        print!("Please input your guess: ");
        io::stdout().flush().unwrap();

        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // Convert and compare
        let guess: u64 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        print!("You guessed {guess}, ");
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("but that's too small."),
            Ordering::Greater => println!("but that's too big"),
            Ordering::Equal => {
                println!("which is correct! You win!");
                break;
            }
        }
    }
}
