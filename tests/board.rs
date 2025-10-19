use rust_chess::{
    board::Board,
    core::{chess_move::Move, color::Color, square::Square::*},
};

#[test]
fn board_test() {
    let mut b: Board = Board::new();

    b.apply_move(Move::new(D2, D3, None, None));
    b.apply_move(Move::new(E7, E6, None, None));
    b.apply_move(Move::new(B1, A3, None, None));
    b.apply_move(Move::new(D8, G5, None, None));

    assert_eq!(b.get_all_pseudo_legal_moves(Color::White).len(), 25);
    assert_eq!(b.get_all_legal_moves(Color::White).len(), 24);
}
