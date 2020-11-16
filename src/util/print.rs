use chess::Board;
use std::fmt::Write;

pub fn print_board_ascii(board: &Board) {
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

    write!(&mut res, "   +------------------------+\n 8 |").unwrap();
    let mut rank = 7;
    for ch in pieces.chars() {
        if ch == '/' {
            write!(&mut res, "|\n {} |", rank).unwrap();
            rank -= 1;
        } else if ch.is_digit(10) {
            let num = ch.to_digit(10).unwrap();
            write!(&mut res, "{}", " . ".repeat(num as usize)).unwrap();
        } else {
            write!(&mut res, " {} ", ch).unwrap();
        }
    }
    write!(
        &mut res,
        "|\n   +------------------------+\n     a  b  c  d  e  f  g  h\n"
    )
    .unwrap();

    println!("{}", res);
}

pub fn print_board(board: &Board) {
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

    write!(&mut res, "   ┌────────────────────────┐\n 8 │").unwrap();
    let mut rank = 7;
    for ch in pieces.chars() {
        if ch == '/' {
            write!(&mut res, "│\n {} │", rank).unwrap();
            rank -= 1;
        } else if ch.is_digit(10) {
            let num = ch.to_digit(10).unwrap();
            write!(&mut res, "{}", " . ".repeat(num as usize)).unwrap();
        } else {
            let piece_ch = match ch {
                'K' => "♔",
                'Q' => "♕",
                'R' => "♖",
                'B' => "♗",
                'N' => "♘",
                'P' => "♙",
                'k' => "♚",
                'q' => "♛",
                'r' => "♜",
                'b' => "♝",
                'n' => "♞",
                'p' => "♟",
                _ => "?",
            };
            write!(&mut res, " {} ", piece_ch).unwrap();
        }
    }
    write!(
        &mut res,
        "│\n   └────────────────────────┘\n     a  b  c  d  e  f  g  h\n"
    )
    .unwrap();

    println!("{}", res);
}
