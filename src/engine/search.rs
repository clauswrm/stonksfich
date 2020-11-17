use super::evaluation::simple::evaluate_board;
use chess::{Board, ChessMove, MoveGen, EMPTY};

/// Root function of Alpha-Beta search algorithm, returning the best move
/// found after a search with depth=`depth`.
///
pub fn find_move(board: &Board, depth: u8) -> ChessMove {
    let mut movegen = MoveGen::new_legal(board);
    let mut best_move: Option<ChessMove> = None;
    let mut best_move_score = -20_000;
    let mut resulting_board = Board::default();
    for cmove in &mut movegen {
        board.make_move(cmove, &mut resulting_board);
        let score = -alpha_beta_search(&resulting_board, depth - 1, -20_000, 20_000);
        println!("Move: {}, Score: {}", cmove, score);
        if score > best_move_score {
            best_move = Some(cmove);
            best_move_score = score;
        }
    }
    return match best_move {
        Some(chosen_move) => chosen_move,
        // If checkmate is inevitable, no move will have been selected
        None => MoveGen::new_legal(board)
            .next()
            .expect("No legal moves for the given board!"),
    };
}

/// Recursivley search the move-tree using a min-max strategy (NegaMax) with
/// alpha-beta pruning, returning an evaluation score for the given board
/// state.
///
/// As a simple sorting of the legal moves, capturing moves are iterated first.
///
/// See https://www.chessprogramming.org/Alpha-Beta#Negamax_Framework
///
fn alpha_beta_search(board: &Board, depth: u8, alpha: i32, beta: i32) -> i32 {
    if depth == 0 {
        return quiescence_search(&board, alpha, beta);
    } else {
        let mut movegen = MoveGen::new_legal(board);
        let mut new_alpha = alpha;
        let mut resulting_board = Board::default();
        let targets = board.color_combined(!board.side_to_move());

        movegen.set_iterator_mask(*targets);
        for cmove in &mut movegen {
            board.make_move(cmove, &mut resulting_board);
            let score = -alpha_beta_search(&resulting_board, depth - 1, -beta, -new_alpha);
            if score >= beta {
                return beta;
            }
            if score > new_alpha {
                new_alpha = score;
            }
        }
        movegen.set_iterator_mask(!EMPTY);
        for cmove in &mut movegen {
            board.make_move(cmove, &mut resulting_board);
            let score = -alpha_beta_search(&resulting_board, depth - 1, -beta, -new_alpha);
            if score >= beta {
                return beta;
            }
            if score > new_alpha {
                new_alpha = score;
            }
        }
        return new_alpha;
    }
}

/// Perform an Quiescence search, used to only evaluate "quiet" positions in
/// leaf nodes of the main search tree.
///
/// See https://www.chessprogramming.org/Quiescence_Search
///
fn quiescence_search(board: &Board, alpha: i32, beta: i32) -> i32 {
    let stand_pat = evaluate_board(&board);
    let mut new_alpha = alpha;
    if stand_pat >= beta {
        return beta;
    }
    if new_alpha < stand_pat {
        new_alpha = stand_pat;
    }

    let mut movegen = MoveGen::new_legal(board);
    let mut resulting_board = Board::default();
    let targets = board.color_combined(!board.side_to_move());

    // Only iterate captures
    movegen.set_iterator_mask(*targets);
    for cmove in &mut movegen {
        board.make_move(cmove, &mut resulting_board);
        let score = -quiescence_search(&resulting_board, -beta, -new_alpha);
        if score >= beta {
            return beta;
        }
        if score > new_alpha {
            new_alpha = score;
        }
    }
    return new_alpha;
}

// #[derive(Eq, PartialEq, PartialOrd, Ord, Copy, Clone)]
// pub struct TranspositionTableEntry {
//     pub score: i32,
//     pub best_move: ChessMove,
//     pub zobrist_key: u32,
//     pub depth: u8,
// }

// let mut tt = CacheTable::new(
//     65536,
//     TranspositionTableEntry {
//         score: 0,
//         best_move: ChessMove::new(Square::E7, Square::E5, None),
//         zobrist_key: 0,
//         depth: 0,
//     },
// );
