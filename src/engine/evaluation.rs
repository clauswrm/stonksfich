/// A collection of simple chess board evaluaiton techniques.
///
pub mod simple {
    use chess::{BitBoard, Board, Color, Piece};

    /// Evaluate the board as seen from the perspective of the player who's side
    /// it is to move.
    ///
    /// See https://www.chessprogramming.org/Simplified_Evaluation_Function#Piece_Values
    ///
    pub fn evaluate_board(board: &Board) -> i32 {
        let side: i32 = match board.side_to_move() {
            Color::White => 1,
            Color::Black => -1,
        };
        let black_pawns = board.pieces(Piece::Pawn) & board.color_combined(Color::Black);
        let white_pawns = board.pieces(Piece::Pawn) & board.color_combined(Color::White);
        let black_knights = board.pieces(Piece::Knight) & board.color_combined(Color::Black);
        let white_knights = board.pieces(Piece::Knight) & board.color_combined(Color::White);
        let black_bishops = board.pieces(Piece::Bishop) & board.color_combined(Color::Black);
        let white_bishops = board.pieces(Piece::Bishop) & board.color_combined(Color::White);
        let black_rooks = board.pieces(Piece::Rook) & board.color_combined(Color::Black);
        let white_rooks = board.pieces(Piece::Rook) & board.color_combined(Color::White);
        let black_queens = board.pieces(Piece::Queen) & board.color_combined(Color::Black);
        let white_queens = board.pieces(Piece::Queen) & board.color_combined(Color::White);

        let mut positional_value = 0;
        positional_value -= positional_evaluation(black_pawns, BLACK_PAWN_SQUARES);
        positional_value += positional_evaluation(white_pawns, WHITE_PAWN_SQUARES);
        positional_value -= positional_evaluation(black_knights, BLACK_KNIGHT_SQUARES);
        positional_value += positional_evaluation(white_knights, WHITE_KNIGHT_SQUARES);
        positional_value -= positional_evaluation(black_bishops, BLACK_BISHOP_SQUARES);
        positional_value += positional_evaluation(white_bishops, WHITE_BISHOP_SQUARES);
        positional_value -= positional_evaluation(black_rooks, BLACK_ROOK_SQUARES);
        positional_value += positional_evaluation(white_rooks, WHITE_ROOK_SQUARES);

        return ((white_pawns.popcnt() as i32 - black_pawns.popcnt() as i32) * 100
            + (white_knights.popcnt() as i32 - black_knights.popcnt() as i32) * 320
            + (white_bishops.popcnt() as i32 - black_bishops.popcnt() as i32) * 330
            + (white_rooks.popcnt() as i32 - black_rooks.popcnt() as i32) * 500
            + (white_queens.popcnt() as i32 - black_queens.popcnt() as i32) * 900
            + positional_value)
            * side;
    }

    /// Evaluate piece positions as spesified in a Piece-Square table.
    ///
    /// See https://www.chessprogramming.org/Simplified_Evaluation_Function#Piece-Square_Tables
    ///
    #[inline]
    fn positional_evaluation(pieces: BitBoard, piece_square_table: [i32; 64]) -> i32 {
        let mut sum = 0;
        for square in pieces {
            sum += piece_square_table[square.to_index()];
        }
        return sum;
    }

    const BLACK_PAWN_SQUARES: [i32; 64] = [
        0, 0, 0, 0, 0, 0, 0, 0, 50, 50, 50, 50, 50, 50, 50, 50, 10, 10, 20, 30, 30, 20, 10, 10, 5,
        5, 10, 25, 25, 10, 5, 5, 0, 0, 0, 20, 20, 0, 0, 0, 5, -5, -10, 0, 0, -10, -5, 5, 5, 10, 10,
        -20, -20, 10, 10, 5, 0, 0, 0, 0, 0, 0, 0, 0,
    ];

    const WHITE_PAWN_SQUARES: [i32; 64] = [
        0, 0, 0, 0, 0, 0, 0, 0, 5, 10, 10, -20, -20, 10, 10, 5, 5, -5, -10, 0, 0, -10, -5, 5, 0, 0,
        0, 20, 20, 0, 0, 0, 5, 5, 10, 25, 25, 10, 5, 5, 10, 10, 20, 30, 30, 20, 10, 10, 50, 50, 50,
        50, 50, 50, 50, 50, 0, 0, 0, 0, 0, 0, 0, 0,
    ];

    const BLACK_KNIGHT_SQUARES: [i32; 64] = [
        -50, -40, -30, -30, -30, -30, -40, -50, -40, -20, 0, 0, 0, 0, -20, -40, -30, 0, 10, 15, 15,
        10, 0, -30, -30, 5, 15, 20, 20, 15, 5, -30, -30, 0, 15, 20, 20, 15, 0, -30, -30, 5, 10, 15,
        15, 10, 5, -30, -40, -20, 0, 5, 5, 0, -20, -40, -50, -40, -30, -30, -30, -30, -40, -50,
    ];

    const WHITE_KNIGHT_SQUARES: [i32; 64] = [
        -50, -40, -30, -30, -30, -30, -40, -50, -40, -20, 0, 5, 5, 0, -20, -40, -30, 5, 10, 15, 15,
        10, 5, -30, -30, 0, 15, 20, 20, 15, 0, -30, -30, 5, 15, 20, 20, 15, 5, -30, -30, 0, 10, 15,
        15, 10, 0, -30, -40, -20, 0, 0, 0, 0, -20, -40, -50, -40, -30, -30, -30, -30, -40, -50,
    ];

    const BLACK_BISHOP_SQUARES: [i32; 64] = [
        -20, -10, -10, -10, -10, -10, -10, -20, -10, 0, 0, 0, 0, 0, 0, -10, -10, 0, 5, 10, 10, 5,
        0, -10, -10, 5, 5, 10, 10, 5, 5, -10, -10, 0, 10, 10, 10, 10, 0, -10, -10, 10, 10, 10, 10,
        10, 10, -10, -10, 5, 0, 0, 0, 0, 5, -10, -20, -10, -10, -10, -10, -10, -10, -20,
    ];

    const WHITE_BISHOP_SQUARES: [i32; 64] = [
        -20, -10, -10, -10, -10, -10, -10, -20, -10, 5, 0, 0, 0, 0, 5, -10, -10, 10, 10, 10, 10,
        10, 10, -10, -10, 0, 10, 10, 10, 10, 0, -10, -10, 5, 5, 10, 10, 5, 5, -10, -10, 0, 5, 10,
        10, 5, 0, -10, -10, 0, 0, 0, 0, 0, 0, -10, -20, -10, -10, -10, -10, -10, -10, -20,
    ];

    const BLACK_ROOK_SQUARES: [i32; 64] = [
        0, 0, 0, 0, 0, 0, 0, 0, 5, 10, 10, 10, 10, 10, 10, 5, -5, 0, 0, 0, 0, 0, 0, -5, -5, 0, 0,
        0, 0, 0, 0, -5, -5, 0, 0, 0, 0, 0, 0, -5, -5, 0, 0, 0, 0, 0, 0, -5, -5, 0, 0, 0, 0, 0, 0,
        -5, 0, 0, 0, 5, 5, 0, 0, 0,
    ];

    const WHITE_ROOK_SQUARES: [i32; 64] = [
        0, 0, 0, 5, 5, 0, 0, 0, -5, 0, 0, 0, 0, 0, 0, -5, -5, 0, 0, 0, 0, 0, 0, -5, -5, 0, 0, 0, 0,
        0, 0, -5, -5, 0, 0, 0, 0, 0, 0, -5, -5, 0, 0, 0, 0, 0, 0, -5, 5, 10, 10, 10, 10, 10, 10, 5,
        0, 0, 0, 0, 0, 0, 0, 0,
    ];
}
