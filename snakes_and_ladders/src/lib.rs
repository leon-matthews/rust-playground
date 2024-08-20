
use std::collections::HashMap;
use std::time::Duration;

use rand::RngCore;
use rand::rngs::SmallRng;
use rand::prelude::*;


/// What a player rolled, and where they ended up
#[derive(Debug)]
struct Turn {
    roll: u8,
    result: u8,
}


/// Entire game history.
/// Last turn must be an exact roll to land on 100.
#[derive(Debug)]
struct Game {
    turns: Vec<Turn>,
}


impl Game {
    fn new() -> Self {
        Self {
            turns: Vec::new(),
        }
    }

    fn add_turn(&mut self, roll: u8, result: u8) {
        self.turns.push(Turn { roll, result });
    }

    fn clear(&mut self) {
        self.turns.clear();
    }
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


pub fn play_games(num_games: usize) -> usize {
    // Use strong default RNG to seed faster non-cryptographic generator.
    // We can then create multiple small RNGs, one per work-unit.
    let mut thread_rng = rand::thread_rng();
    let mut rng = SmallRng::from_rng(&mut thread_rng).unwrap();

    let mut game = Game::new();
    let mut num_rolls = 0;
    for _ in 1..=num_games {
        num_rolls = play_game(&mut rng, &mut game);
        game.clear();
    }

    dbg!(game.turns.capacity());

    num_rolls
}


/// Play a single game of Snakes and Ladders solitaire
fn play_game(rng: &mut SmallRng, game: &mut Game) -> usize {
    let mut num_rolls: usize = 0;
    let mut place: u8 = 0;

    loop {
        // Roll the dice
        //~ let roll = rng.gen_range(1..=6);
        let roll: u8 = (rng.next_u64() % 6 + 1) as u8;
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

        // Save turn
        game.add_turn(roll, place);

        if place == 100 { break; }
    };

    num_rolls
}
