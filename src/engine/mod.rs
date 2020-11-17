use super::util::print::print_board;
use chess::{Board, Color, Game};
use player::Player;

mod evaluation;
pub mod player;
pub mod search;

pub fn play_game(
    white_player: &impl Player,
    black_player: &impl Player,
    starting_board: Option<Board>,
) {
    let mut game = match starting_board {
        Some(initial_board) => Game::new_with_board(initial_board),
        None => Game::new(),
    };

    while game.result().is_none() {
        let board = game.current_position();
        let chosen_move = match board.side_to_move() {
            Color::White => white_player.choose_move(&board),
            Color::Black => black_player.choose_move(&board),
        };
        game.make_move(chosen_move);
        print_board(&board, Some(&chosen_move));
    }
    println!("Game Over: {:?}", game.result());
}
