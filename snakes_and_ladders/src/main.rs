
use clap::Parser;
use rand::prelude::*;
use std::time::Instant;

use snakes_and_ladders::play_game;


/// Play many, many solo games of Snakes and Ladders
#[derive(Parser, Debug)]
#[command(version, about)]
struct Args {
    /// Number of games to play
    #[arg(short, long, default_value_t=10_000_000)]
    num_games: usize,

    /// Run for at least this many seconds
    #[arg(short, long, default_value_t=10)]
    seconds: usize,
}


fn main() {
    let args = Args::parse();
    println!("{:?}", args);

    // Use strong default RNG to seed faster non-cryptographic generator.
    // We can then create multiple small RNGs, one per work-unit.
    let mut thread_rng = rand::thread_rng();
    let mut rng = SmallRng::from_rng(&mut thread_rng).unwrap();

    let timer = Instant::now();
    let num_games = 1e6 as usize;
    let mut num_rolls = 0;
    for _ in 1..=num_games {
        num_rolls = play_game(&mut rng);
    }
    println!("Played {} games in {:?}", num_games, timer.elapsed());
    println!("Finished last game in {} rolls", num_rolls);
}
