use stonksfich::engine::play_game;
use stonksfich::engine::player::{Human, TTBot};

fn main() {
    const DEPTH: u8 = 6;
    const TT_SIZE: usize = 65536;
    let white_player = Human {};
    let black_player = TTBot::new(DEPTH, TT_SIZE);

    let result = play_game(&white_player, &black_player, None);
    println!("Game Over: {:?}", result);
}
