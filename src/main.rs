fn main() {
    use chess::Board;
    use chess::MoveGen;
    use chess::EMPTY;

    // create a board with the initial position
    let board = Board::default();

    // create an iterable
    let mut iterable = MoveGen::new_legal(&board);

    // make sure .len() works.
    assert_eq!(iterable.len(), 20); // the .len() function does *not* consume the iterator

    // lets iterate over targets.
    let targets = board.color_combined(!board.side_to_move());
    iterable.set_iterator_mask(*targets);

    // count the number of targets
    let mut count = 0;
    for _ in &mut iterable {
        count += 1;
        // This move captures one of my opponents pieces (with the exception of en passant)
    }

    // now, iterate over the rest of the moves
    iterable.set_iterator_mask(!EMPTY);
    for _ in &mut iterable {
        count += 1;
        // This move does not capture anything
    }

    // make sure it works
    assert_eq!(count, 20);
}
