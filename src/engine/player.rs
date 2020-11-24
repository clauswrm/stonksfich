use super::super::util::io::get_move_cli;
use super::search::{find_move, tt_find_move};
use super::tt::{TTEntry, TTFlag};
use chess::{Board, CacheTable, ChessMove};
use std::time::Instant;

/// A trait representing some entity that can play chess.
///
pub trait Player {
    fn choose_move(&mut self, board: &Board) -> ChessMove;
}

/// A player representing an AI, finding it's moves through searching the game
/// tree.
///
pub struct Bot {
    pub depth: u8,
}

impl Player for Bot {
    fn choose_move(&mut self, board: &Board) -> ChessMove {
        let start = Instant::now();
        let chosen_move = find_move(board, self.depth);
        let duration = start.elapsed();
        println!(
            "Chosen move: {}\nTime elapsed: {:?}\n",
            chosen_move, duration
        );
        return chosen_move;
    }
}

/// A player representing an AI, finding it's moves through searching the game
/// tree.
///
pub struct TTBot {
    pub depth: u8,
    pub tt: CacheTable<TTEntry>,
}

impl TTBot {
    pub fn new(depth: u8, tt_size: usize) -> Self {
        return TTBot {
            depth: depth,
            tt: CacheTable::new(
                tt_size,
                TTEntry {
                    score: 0,
                    // best_move: None,
                    zobrist_key: 0,
                    depth: 0,
                    flag: TTFlag::Exact,
                },
            ),
        };
    }
}

impl Player for TTBot {
    fn choose_move(&mut self, board: &Board) -> ChessMove {
        let start = Instant::now();
        let chosen_move = tt_find_move(board, self.depth, &mut self.tt);
        let duration = start.elapsed();
        println!(
            "Chosen move: {}\nTime elapsed: {:?}\n",
            chosen_move, duration
        );
        return chosen_move;
    }
}

/// A player representing a human, getting it's moves from strings typed
/// in a terminal.
///
pub struct Human {}

impl Player for Human {
    fn choose_move(&mut self, board: &Board) -> ChessMove {
        return get_move_cli(board);
    }
}
