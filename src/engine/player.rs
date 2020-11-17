use super::super::util::io::get_move_cli;
use super::search::find_move;
use chess::{Board, ChessMove};
use std::time::Instant;

/// A trait representing some entity that can play chess.
///
pub trait Player {
    fn choose_move(&self, board: &Board) -> ChessMove;
}

/// A player representing an AI, finding it's moves through searching the game
/// tree.
///
pub struct Bot {
    depth: u8,
}

impl Player for Bot {
    fn choose_move(&self, board: &Board) -> ChessMove {
        let start = Instant::now();
        let chosen_move = find_move(board, self.depth);
        let duration = start.elapsed();
        println!("Chosen move: {}\nTime elapsed: {:?}", chosen_move, duration);
        return chosen_move;
    }
}

/// A player representing a human, getting it's moves from strings typed
/// in a terminal.
///
pub struct Human {}

impl Player for Human {
    fn choose_move(&self, board: &Board) -> ChessMove {
        return get_move_cli(board);
    }
}
