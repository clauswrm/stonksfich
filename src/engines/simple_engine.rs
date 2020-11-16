use chess::{Board, /*CacheTable,*/ ChessMove, Color, MoveGen, Piece, EMPTY};

// #[derive(Eq, PartialEq, PartialOrd, Ord, Copy, Clone)]
// pub struct TranspositionTableEntry {
//     pub score: i32,
//     pub best_move: ChessMove,
//     pub zobrist_key: u32,
//     pub depth: u8,
// }

pub fn find_move(
    board: &Board,
    depth: u8,
    // tt: &mut CacheTable<TranspositionTableEntry>,
) -> ChessMove {
    let mut movegen = MoveGen::new_legal(board);
    let mut best_move: Option<ChessMove> = None;
    let mut best_move_score = -20_000;
    let mut resulting_board = Board::default();
    for cmove in &mut movegen {
        board.make_move(cmove, &mut resulting_board);
        // let score = -nega_max(&resulting_board, depth - 1);
        let score = -alpha_beta_nega(&resulting_board, depth - 1, -20_000, 20_000); //, tt);
        //println!("{} {}", cmove, score);
        if score > best_move_score {
            best_move = Some(cmove);
            best_move_score = score;
        }
    }
    return best_move.unwrap();
}

/// Recursivley search the move-tree using a min-max strategy, returning an
/// evaluation score for the given board state.
///
/// See https://www.chessprogramming.org/Negamax
///
// fn nega_max(board: &Board, depth: u8) -> i32 {
//     if depth == 0 {
//         return evaluate_board(&board); // + MoveGen::new_legal(board).len() as i32;
//     } else {
//         let mut movegen = MoveGen::new_legal(board);
//         let mut best_move_score = -20_000;
//         let mut resulting_board = Board::default();
//         for cmove in &mut movegen {
//             board.make_move(cmove, &mut resulting_board);
//             let score = -nega_max(&resulting_board, depth - 1);
//             if score > best_move_score {
//                 best_move_score = score;
//             }
//         }
//         return best_move_score;
//     }
// }

/// Recursivley search the move-tree using a min-max strategy with alpha-beta
/// pruning employed, returning an evaluation score for the given board state.
///
/// As a simple sorting of the legal moves, capturing moves are iterated first.
///
/// See https://www.chessprogramming.org/Alpha-Beta#Negamax_Framework
///
fn alpha_beta_nega(
    board: &Board,
    depth: u8,
    alpha: i32,
    beta: i32,
    // tt: &mut CacheTable<TranspositionTableEntry>,
) -> i32 {
    // if let Some(item) = tt.get(board.get_hash()) {}
    if depth == 0 {
        return evaluate_board(&board);
    } else {
        let mut movegen = MoveGen::new_legal(board);
        let mut new_alpha = alpha;
        let mut resulting_board = Board::default();
        let targets = board.color_combined(!board.side_to_move());

        movegen.set_iterator_mask(*targets);
        for cmove in &mut movegen {
            board.make_move(cmove, &mut resulting_board);
            let score = -alpha_beta_nega(&resulting_board, depth - 1, -beta, -alpha); //, tt);
            if score >= beta {
                return beta;
            }
            if score > alpha {
                new_alpha = score;
            }
        }
        movegen.set_iterator_mask(!EMPTY);
        for cmove in &mut movegen {
            board.make_move(cmove, &mut resulting_board);
            let score = -alpha_beta_nega(&resulting_board, depth - 1, -beta, -alpha); //, tt);
            if score >= beta {
                return beta;
            }
            if score > alpha {
                new_alpha = score;
            }
        }
        return new_alpha;
    }
}

// const black_pawn_square : [i32; 64] =
//    [ 0,  0,  0,  0,  0,  0,  0,  0,
//     50, 50, 50, 50, 50, 50, 50, 50,
//     10, 10, 20, 30, 30, 20, 10, 10,
//      5,  5, 10, 25, 25, 10,  5,  5,
//      0,  0,  0, 20, 20,  0,  0,  0,
//      5, -5,-10,  0,  0,-10, -5,  5,
//      5, 10, 10,-20,-20, 10, 10,  5,
//      0,  0,  0,  0,  0,  0,  0,  0,];

// const white_pawn_square : [i32; 64] =
//    [ 0,  0,  0,  0,  0,  0,  0,  0,
//      5, 10, 10,-20,-20, 10, 10,  5,
//      5, -5,-10,  0,  0,-10, -5,  5,
//      0,  0,  0, 20, 20,  0,  0,  0,
//      5,  5, 10, 25, 25, 10,  5,  5,
//     10, 10, 20, 30, 30, 20, 10, 10,
//     50, 50, 50, 50, 50, 50, 50, 50,
//      0,  0,  0,  0,  0,  0,  0,  0,];

// const black_knight_square : [i32; 64] =
//    [-50,-40,-30,-30,-30,-30,-40,-50,
//     -40,-20,  0,  0,  0,  0,-20,-40,
//     -30,  0, 10, 15, 15, 10,  0,-30,
//     -30,  5, 15, 20, 20, 15,  5,-30,
//     -30,  0, 15, 20, 20, 15,  0,-30,
//     -30,  5, 10, 15, 15, 10,  5,-30,
//     -40,-20,  0,  5,  5,  0,-20,-40,
//     -50,-40,-30,-30,-30,-30,-40,-50,];

// const white_knight_square : [i32; 64] =
//     [-50,-40,-30,-30,-30,-30,-40,-50,
//      -40,-20,  0,  5,  5,  0,-20,-40,
//      -30,  5, 10, 15, 15, 10,  5,-30,
//      -30,  0, 15, 20, 20, 15,  0,-30,
//      -30,  5, 15, 20, 20, 15,  5,-30,
//      -30,  0, 10, 15, 15, 10,  0,-30,
//      -40,-20,  0,  0,  0,  0,-20,-40,
//      -50,-40,-30,-30,-30,-30,-40,-50,] ;

// const black_bishop_square : [i32; 64] =
//    [-20,-10,-10,-10,-10,-10,-10,-20,
//     -10,  0,  0,  0,  0,  0,  0,-10,
//     -10,  0,  5, 10, 10,  5,  0,-10,
//     -10,  5,  5, 10, 10,  5,  5,-10,
//     -10,  0, 10, 10, 10, 10,  0,-10,
//     -10, 10, 10, 10, 10, 10, 10,-10,
//     -10,  5,  0,  0,  0,  0,  5,-10,
//     -20,-10,-10,-10,-10,-10,-10,-20,];

// const white_bishop_square : [i32; 64] =
//     [-20,-10,-10,-10,-10,-10,-10,-20,
//      -10,  5,  0,  0,  0,  0,  5,-10,
//      -10, 10, 10, 10, 10, 10, 10,-10,
//      -10,  0, 10, 10, 10, 10,  0,-10,
//      -10,  5,  5, 10, 10,  5,  5,-10,
//      -10,  0,  5, 10, 10,  5,  0,-10,
//      -10,  0,  0,  0,  0,  0,  0,-10,
//      -20,-10,-10,-10,-10,-10,-10,-20,];

// const black_rook_square : [i32; 64] =
//    [ 0,  0,  0,  0,  0,  0,  0,  0,
//      5, 10, 10, 10, 10, 10, 10,  5,
//     -5,  0,  0,  0,  0,  0,  0, -5,
//     -5,  0,  0,  0,  0,  0,  0, -5,
//     -5,  0,  0,  0,  0,  0,  0, -5,
//     -5,  0,  0,  0,  0,  0,  0, -5,
//     -5,  0,  0,  0,  0,  0,  0, -5,
//      0,  0,  0,  5,  5,  0,  0,  0,];

// const white_rook_square : [i32; 64] =
//     [ 0,  0,  0,  5,  5,  0,  0,  0,
//      -5,  0,  0,  0,  0,  0,  0, -5,
//      -5,  0,  0,  0,  0,  0,  0, -5,
//      -5,  0,  0,  0,  0,  0,  0, -5,
//      -5,  0,  0,  0,  0,  0,  0, -5,
//      -5,  0,  0,  0,  0,  0,  0, -5,
//       5, 10, 10, 10, 10, 10, 10,  5,
//       0,  0,  0,  0,  0,  0,  0,  0,];

/// Evaluate the board as seen from the perspective of the player who's side
/// it is to move.
///
/// See https://www.chessprogramming.org/Simplified_Evaluation_Function#Piece_Values
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
