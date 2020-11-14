use chess::{Board, BoardStatus, ChessMove, Color, Square};
use chess_engine::engines::simple_engine::find_move;
use chess_engine::util::print_board;
use std::io;
use std::io::Write;

const DEPTH: u32 = 4;

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
            let chosen_move = find_move(&board, DEPTH);
            board = board.make_move_new(chosen_move);
        }
    }
    print_board(&board);

    println!("The result is {:#?}!", board.status())
}
