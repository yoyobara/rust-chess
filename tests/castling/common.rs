use rust_chess::{
    board::Board,
    core::{
        color::Color,
        piece::{Piece, PieceType},
        square::Square::*,
    },
};

pub fn setup_board_for_white_castling() -> Board {
    let mut b: Board = Board::empty();

    b.set(
        E1,
        Some(Piece {
            piece_color: Color::White,
            piece_type: PieceType::King,
        }),
    );
    b.set(
        H1,
        Some(Piece {
            piece_color: Color::White,
            piece_type: PieceType::Rook,
        }),
    );
    b.set(
        A1,
        Some(Piece {
            piece_color: Color::White,
            piece_type: PieceType::Rook,
        }),
    );

    b.set(
        A8,
        Some(Piece {
            piece_color: Color::Black,
            piece_type: PieceType::King,
        }),
    );

    b
}
