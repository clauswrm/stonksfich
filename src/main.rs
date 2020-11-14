use chess::*;
use std::fmt::Write;
use std::io;
use std::io::Write as IOWrite;

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
            let chosen_move = find_move(&board);
            board = board.make_move_new(chosen_move);
        }
    }
    print_board(&board);

    println!("The result is {:#?}!", board.status())
}

fn find_move(board: &Board) -> ChessMove {
    let mut movegen = MoveGen::new_legal(board);
    let mut best_move: Option<ChessMove> = None;
    let mut best_move_score = -20_000;

    for cmove in &mut movegen {
        let mut resulting_board = Board::default();
        board.make_move(cmove, &mut resulting_board);
        let score = evaluate_board(&resulting_board);
        //println!("{} {}", cmove, score);
        if score > best_move_score {
            best_move = Some(cmove);
            best_move_score = score;
        }
    }

    return best_move.unwrap();
}

/// Evaluate the board as seen from the perspective of the player who's side it is to move.
/// See https://www.chessprogramming.org/Simplified_Evaluation_Function
///
fn evaluate_board(board: &Board) -> i32 {
    let side: i32 = match board.side_to_move() {
        Color::White => -1,
        Color::Black => 1,
    };
    let black_pawns =
        (board.pieces(Piece::Pawn) & board.color_combined(Color::Black)).popcnt() as i32;
    let white_pawns =
        (board.pieces(Piece::Pawn) & board.color_combined(Color::White)).popcnt() as i32;
    let black_knights =
        (board.pieces(Piece::Knight) & board.color_combined(Color::Black)).popcnt() as i32;
    let white_knights =
        (board.pieces(Piece::Knight) & board.color_combined(Color::White)).popcnt() as i32;
    let black_bishops =
        (board.pieces(Piece::Bishop) & board.color_combined(Color::Black)).popcnt() as i32;
    let white_bishops =
        (board.pieces(Piece::Bishop) & board.color_combined(Color::White)).popcnt() as i32;
    let black_rooks =
        (board.pieces(Piece::Rook) & board.color_combined(Color::Black)).popcnt() as i32;
    let white_rooks =
        (board.pieces(Piece::Rook) & board.color_combined(Color::White)).popcnt() as i32;
    let black_queens =
        (board.pieces(Piece::Queen) & board.color_combined(Color::Black)).popcnt() as i32;
    let white_queens =
        (board.pieces(Piece::Queen) & board.color_combined(Color::White)).popcnt() as i32;
    return ((white_pawns - black_pawns) * 100
        + (white_knights - black_knights) * 320
        + (white_bishops - black_bishops) * 330
        + (white_rooks - black_rooks) * 500
        + (white_queens - black_queens) * 900)
        * side;
}

fn print_board(board: &Board) {
    let fen_string = format!("{}", board);
    let mut fen_itr = fen_string.split_whitespace();

    let pieces = fen_itr.next().unwrap();
    let player = fen_itr.next().unwrap();
    let _casteling_rights = fen_itr.next().unwrap();
    let _en_passant_rights = fen_itr.next().unwrap();
    let _half_move_clock = fen_itr.next().unwrap();
    let full_move_number = fen_itr.next().unwrap();

    let mut res = String::new();

    write!(
        &mut res,
        "Turn {} - {} to move.\n\n",
        full_move_number, player
    )
    .unwrap();

    for ch in pieces.chars() {
        if ch == '/' {
            write!(&mut res, "\n").unwrap();
        } else if ch.is_digit(10) {
            let num = ch.to_digit(10).unwrap();
            write!(&mut res, "{}", ".".repeat(num as usize)).unwrap();
        } else {
            write!(&mut res, "{}", ch).unwrap();
        }
    }
    write!(&mut res, "\n").unwrap();

    println!("{}", res);
}
