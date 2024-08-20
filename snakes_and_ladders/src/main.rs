
use clap::Parser;

use std::time::Instant;

use snakes_and_ladders;


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

    let timer = Instant::now();
    let num_games: usize = 1_000_000;
    let num_rolls = snakes_and_ladders::play_games(num_games);
    let elapsed = timer.elapsed();
    println!("Played {} games in {:?}", num_games, elapsed);
    println!("Games for second {}", num_games as f64 / elapsed.as_secs_f64());
    println!("Finished last game in {} rolls", num_rolls);
}
