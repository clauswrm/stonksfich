use chess::{Board, ChessMove, Color, MoveGen, Piece};

pub fn find_move(board: &Board, depth: u32) -> ChessMove {
    let mut movegen = MoveGen::new_legal(board);
    let mut best_move: Option<ChessMove> = None;
    let mut best_move_score = -20_000;
    let mut resulting_board = Board::default();
    for cmove in &mut movegen {
        board.make_move(cmove, &mut resulting_board);
        let score = -nega_max(&resulting_board, depth - 1);
        println!("{} {}", cmove, score);
        if score > best_move_score {
            best_move = Some(cmove);
            best_move_score = score;
        }
    }
    return best_move.unwrap();
}
fn nega_max(board: &Board, depth: u32) -> i32 {
    if depth == 0 {
        return evaluate_board(&board);
    } else {
        let mut movegen = MoveGen::new_legal(board);
        let mut best_move_score = -20_000;
        let mut resulting_board = Board::default();
        for cmove in &mut movegen {
            board.make_move(cmove, &mut resulting_board);
            let score = -nega_max(&resulting_board, depth - 1);
            if score > best_move_score {
                best_move_score = score;
            }
        }
        return best_move_score;
    }
}
/// Evaluate the board as seen from the perspective of the player who's side it is to move.
/// See https://www.chessprogramming.org/Simplified_Evaluation_Function
///
fn evaluate_board(board: &Board) -> i32 {
    let side: i32 = match board.side_to_move() {
        Color::White => 1,
        Color::Black => -1,
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
