use rust_chess::core::{
    color::Color,
    piece::{Piece, PieceType}, square::Square,
};

#[test]
fn piece_testing() {
    let mut piece = Piece {
        piece_type: PieceType::Bishop,
        piece_color: Color::White,
    };
    assert_eq!(piece.to_ascii(), 'B');

    piece.piece_type = PieceType::Queen;
    assert_eq!(piece.to_ascii(), 'Q');

    piece.piece_color = !piece.piece_color;
    assert_eq!(piece.to_ascii(), 'q');
}

#[test]
fn square_testing() {
    assert_eq!(Square::F5.to_index(), 37);
    assert_eq!(Square::F5, Square::from_index(37));
}