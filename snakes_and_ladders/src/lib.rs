
use std::collections::HashMap;
use std::time::Duration;

use rand::RngCore;
use rand::rngs::SmallRng;


/// What a player rolled, and where they ended up
struct Turn {
    roll: u8,
    result: u8,
}


/// Entire game history.
/// Last turn must be an exact roll to land on 100.
struct Game {
    turns: Vec<Turn>,
}


/// Aggregate results from many, many games
struct BenchmarkResult {
    elapsed: Duration,
    num_games: usize,

    /// Mapping of game length against number of games.
    counts: HashMap<usize, usize>,

    /// Full roll and position history of shortest and longest games played.
    shortest: Game,
    longest: Game,
}


/// Play a single game of Snakes and Ladders solitaire
pub fn play_game(rng: &mut SmallRng) -> i32 {
    let mut num_rolls = 0;
    let mut place = 0;

    loop {
        // Roll the dice
        //~ let roll = rng.gen_range(1..=6);            // 227ms for 1e6 games
        let roll = rng.next_u64() % 6 + 1;              // 176ms for 1e6 games
        num_rolls += 1;

        // Where did you end up?
        let landed = place + roll;

        // Where did you *really* end up?
        place = match landed {
            // Ladders
            1 => 38,
            4 => 14,
            9 => 31,
            21 => 42,
            28 => 84,
            36 => 44,
            51 => 67,
            71 => 91,
            80 => 100,

            // Snakes
            98 => 78,
            95 => 75,
            93 => 73,
            87 => 24,
            64 => 60,
            62 => 19,
            56 => 53,
            49 => 11,
            48 => 26,
            16 => 6,

            // Too high? Stay where you are.
            n if n > 100 => place,

            // Normal move
            _ => landed,
        };

        if place == 100 { break; }
    };

    num_rolls
}
