// use chess::ChessMove;

#[derive(Eq, PartialEq, PartialOrd, Ord, Copy, Clone)]
pub struct TTEntry {
    pub score: i32,
    // pub best_move: Option<ChessMove>,
    pub zobrist_key: u64,
    pub depth: u8,
    pub flag: TTFlag,
}

#[derive(Eq, PartialEq, PartialOrd, Ord, Copy, Clone)]
pub enum TTFlag {
    Exact = 0,
    LowerBound = 1,
    UpperBound = 2,
}
