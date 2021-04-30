use stonksfich::engine::play_game;
use stonksfich::engine::player::{Human, TTBot};

fn main() {
    const DEPTH: u8 = 6;
    const TT_SIZE: usize = 1048576;
    let mut white_player = Human {};
    let mut black_player = TTBot::new(DEPTH, TT_SIZE);

    let result = play_game(&mut white_player, &mut black_player, None);
    println!("Game Over: {:?}", result);
}
