use stonksfich::engine::play_game;
use stonksfich::engine::player::{Bot, Human};

fn main() {
    const DEPTH: u8 = 6;
    let white_player = Human {};
    let black_player = Bot { depth: DEPTH };

    play_game(&white_player, &black_player, None)
}
