use chess::{Board, ChessMove, Square};
use std::io;
use std::io::Write;

/// Prompt the user to type a legal move in the command-line, given the
/// current board state, in the form of "\<square_from\>\<square_to\>" in UCI
/// format. E.g "e2e4".
///
pub fn get_move_cli(board: &Board) -> ChessMove {
    loop {
        print!("Your move: ");
        io::stdout().flush().unwrap();
        let mut move_string = String::new();
        match io::stdin().read_line(&mut move_string) {
            Ok(_) => {
                if move_string.trim().len() == 4 {
                    let from_square = Square::from_string(move_string[..2].to_string());
                    let to_square = Square::from_string(move_string[2..4].to_string());
                    match (from_square, to_square) {
                        (Some(from), Some(to)) => {
                            let chosen_move = ChessMove::new(from, to, None);
                            if board.legal(chosen_move) {
                                return chosen_move;
                            } else {
                                println!("Illegal move specified, try again.")
                            }
                        }
                        _ => println!(
                            "Failed to parse input as two squares in UCI-format (e.g. 'e2e4')."
                        ),
                    }
                } else {
                    println!("Input should be 4 characters long (e.g. 'e2e4').")
                }
            }
            Err(_) => println!("Failed to read input, try again."),
        };
    }
}
