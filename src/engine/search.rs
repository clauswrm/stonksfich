use super::evaluation::simple::evaluate_board;
use super::tt::{TTEntry, TTFlag};
use chess::{Board, CacheTable, ChessMove, MoveGen, EMPTY};

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
        let score = -alpha_beta_search(&resulting_board, depth - 1, -20_000, 20_000, true);
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
fn alpha_beta_search(board: &Board, depth: u8, alpha: i32, beta: i32, can_null: bool) -> i32 {
    if depth == 0 {
        return quiescence_search(&board, alpha, beta);
    }
    if can_null {
        if let Some(resulting_board) = board.null_move() {
            let adjusted_depth = match depth < 4 {
                true => 1,
                false => depth - 2,
            };
            let score =
                -alpha_beta_search(&resulting_board, adjusted_depth - 1, -beta, -alpha, false);
            if score >= beta {
                return beta;
            }
        }
    }
    let mut movegen = MoveGen::new_legal(board);
    let mut new_alpha = alpha;
    let mut resulting_board = Board::default();
    let targets = board.color_combined(!board.side_to_move());

    movegen.set_iterator_mask(*targets);
    for cmove in &mut movegen {
        board.make_move(cmove, &mut resulting_board);
        let score = -alpha_beta_search(&resulting_board, depth - 1, -beta, -new_alpha, can_null);
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
        let score = -alpha_beta_search(&resulting_board, depth - 1, -beta, -new_alpha, can_null);
        if score >= beta {
            return beta;
        }
        if score > new_alpha {
            new_alpha = score;
        }
    }
    return new_alpha;
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

pub fn tt_find_move(board: &Board, depth: u8, tt: &mut CacheTable<TTEntry>) -> ChessMove {
    let mut movegen = MoveGen::new_legal(board);
    let mut best_move: Option<ChessMove> = None;
    let mut best_move_score = -20_000;
    let mut resulting_board = Board::default();
    for cmove in &mut movegen {
        board.make_move(cmove, &mut resulting_board);
        let score = -tt_alpha_beta_search(&resulting_board, depth - 1, -20_000, 20_000, true, tt);
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

fn tt_alpha_beta_search(
    board: &Board,
    depth: u8,
    alpha: i32,
    beta: i32,
    can_null: bool,
    tt: &mut CacheTable<TTEntry>,
) -> i32 {
    let mut new_alpha = alpha;
    let mut new_beta = beta;
    let hash = board.get_hash();

    if let Some(entry) = tt.get(hash) {
        if entry.zobrist_key == hash && entry.depth >= depth {
            if entry.flag == TTFlag::Exact {
                return entry.score;
            } else if entry.flag == TTFlag::LowerBound && entry.score > new_alpha {
                new_alpha = entry.score;
            } else if entry.flag == TTFlag::UpperBound && entry.score < beta {
                new_beta = entry.score;
            }

            if new_alpha >= new_beta {
                return entry.score;
            }
        }
    }
    if depth == 0 {
        return quiescence_search(&board, alpha, new_beta);
    }
    if can_null {
        if let Some(resulting_board) = board.null_move() {
            let adjusted_depth = match depth < 4 {
                true => 1,
                false => depth - 2,
            };
            let score = -tt_alpha_beta_search(
                &resulting_board,
                adjusted_depth - 1,
                -new_beta,
                -alpha,
                false,
                tt,
            );
            if score >= new_beta {
                return new_beta;
            }
        }
    }
    let mut movegen = MoveGen::new_legal(board);
    let mut resulting_board = Board::default();
    let targets = board.color_combined(!board.side_to_move());

    movegen.set_iterator_mask(*targets);
    for cmove in &mut movegen {
        board.make_move(cmove, &mut resulting_board);
        let score = -tt_alpha_beta_search(
            &resulting_board,
            depth - 1,
            -new_beta,
            -new_alpha,
            can_null,
            tt,
        );
        if score >= new_beta {
            return new_beta;
        }
        if score > new_alpha {
            new_alpha = score;
        }
    }
    movegen.set_iterator_mask(!EMPTY);
    for cmove in &mut movegen {
        board.make_move(cmove, &mut resulting_board);
        let score = -tt_alpha_beta_search(
            &resulting_board,
            depth - 1,
            -new_beta,
            -new_alpha,
            can_null,
            tt,
        );
        if score >= new_beta {
            return new_beta;
        }
        if score > new_alpha {
            new_alpha = score;
        }
    }

    let flag;
    if new_alpha <= alpha {
        flag = TTFlag::UpperBound;
    } else if new_alpha >= new_beta {
        flag = TTFlag::LowerBound;
    } else {
        flag = TTFlag::Exact;
    };

    (*tt).add(
        hash,
        TTEntry {
            zobrist_key: hash,
            depth: depth,
            score: new_alpha,
            flag: flag,
        },
    );
    return new_alpha;
}
