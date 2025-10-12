use rust_chess::core::{
    color::Color,
    piece::{Piece, PieceType},
    square::Square,
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
    assert_eq!(Square::from_index(37), Some(Square::F5));

    assert_eq!(Square::F5.to_file_rank(), (5, 4));
    assert_eq!(Square::from_file_rank((5, 4)), Some(Square::F5));

    assert_eq!(Square::F5.get_relative_square(1, -2), Some(Square::G3));
    assert_eq!(Square::A1.get_relative_square(2, 5), Some(Square::C6));
}
#[test]
fn square_testing_negative() {
    assert_eq!(Square::from_index(100), None);
    assert_eq!(Square::from_file_rank((2, 8)), None);
}
