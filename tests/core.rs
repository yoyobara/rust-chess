use rust_chess::core::{
    color::Color,
    piece::{Piece, PieceType},
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
