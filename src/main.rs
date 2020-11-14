use chess::{Board, BoardStatus, ChessMove, Color, Square};
use chess_engine::engines::simple_engine::find_move;
use chess_engine::util::print_board;
use std::io;
use std::io::Write;
use std::time::Instant;

const DEPTH: u8 = 4;

fn main() {
    let mut board = Board::default();
    loop {
        if board.status() != BoardStatus::Ongoing {
            break;
        }
        print_board(&board);

        if board.side_to_move() == Color::White {
            print!("Your move: ");
            io::stdout().flush().unwrap();
            let mut move_string = String::new();

            io::stdin()
                .read_line(&mut move_string)
                .expect("Failed to read line");
            let from_square =
                Square::from_string(move_string[..2].to_string()).expect("Not a valid square!");
            let to_square =
                Square::from_string(move_string[2..4].to_string()).expect("Not a valid square!");

            let chosen_move = ChessMove::new(from_square, to_square, None);

            board = board.make_move_new(chosen_move);
        } else {
            let start = Instant::now();
            let chosen_move = find_move(&board, DEPTH);
            let duration = start.elapsed();
            println!("Chosen move: {}\nTime elapsed: {:?}", chosen_move, duration);
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
