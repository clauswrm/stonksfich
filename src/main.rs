use chess::{Board, BoardStatus, /*CacheTable, ChessMove,*/ Color /*Square*/};
use stonksfich::engines::simple_engine::find_move; //{find_move, TranspositionTableEntry};
use stonksfich::util::io::get_move_cli;
use stonksfich::util::print::print_board;

use std::time::Instant;

const DEPTH: u8 = 6;

fn main() {
    let bot = Color::Black;
    let mut board = Board::default();
    // let mut tt = CacheTable::new(
    //     65536,
    //     TranspositionTableEntry {
    //         score: 0,
    //         best_move: ChessMove::new(Square::E7, Square::E5, None),
    //         zobrist_key: 0,
    //         depth: 0,
    //     },
    // );

    loop {
        if board.status() != BoardStatus::Ongoing {
            break;
        }
        print_board(&board);

        if board.side_to_move() == bot {
            let start = Instant::now();
            let chosen_move = find_move(&board, DEPTH); //, &mut tt);
            let duration = start.elapsed();
            println!("Chosen move: {}\nTime elapsed: {:?}", chosen_move, duration);
            board = board.make_move_new(chosen_move);
        } else {
            let chosen_move = get_move_cli(&board);
            board = board.make_move_new(chosen_move);
        }
    }
    print_board(&board);

    let status = board.status();
    let result_string: &str;
    if status == BoardStatus::Stalemate {
        result_string = "Stalemate";
    } else {
        result_string = match board.side_to_move() {
            Color::White => "Checkmate - Black Won",
            Color::Black => "Checkmate - White Won",
        };
    }
    println!("Game Over: {}", result_string);
}
