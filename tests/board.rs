use rust_chess::{
    board::Board,
    core::{
        chess_move::{Move, MoveType},
        color::Color,
        square::Square::*,
    },
};

#[test]
fn board_test() {
    let mut b: Board = Board::default();

    b.apply_move(Move::new(D2, D3, None, MoveType::Quiet));
    b.apply_move(Move::new(E7, E6, None, MoveType::Quiet));
    b.apply_move(Move::new(B1, A3, None, MoveType::Quiet));
    b.apply_move(Move::new(D8, G5, None, MoveType::Quiet));

    assert_eq!(b.get_all_pseudo_legal_moves(Color::White).len(), 25);
    assert_eq!(b.get_all_legal_moves(Color::White).len(), 24);
}
