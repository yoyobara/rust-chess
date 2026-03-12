use rust_chess::{
    board::Board,
    core::{
        chess_move::{Move, MoveType},
        color::Color,
        piece::{Piece, PieceType},
        square::Square::*,
    },
};

fn setup_board_for_white_castling() -> Board {
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

#[test]
fn simple_kingside_castling_test() {
    let mut b = setup_board_for_white_castling();

    let available_moves = b.get_legal_moves(E1).unwrap();
    let castling_move = Move::new(E1, G1, None, MoveType::KingsideCastling);
    assert!(available_moves.contains(&castling_move));

    b.apply_move(castling_move);

    assert_eq!(b.get(E1), None);
    assert_eq!(b.get(F1), Some(Piece::new(PieceType::Rook, Color::White)));
    assert_eq!(b.get(G1), Some(Piece::new(PieceType::King, Color::White)));
    assert_eq!(b.get(H1), None);
}

#[test]
fn simple_queenside_castling_test() {
    let mut b = setup_board_for_white_castling();

    let available_moves = b.get_legal_moves(E1).unwrap();
    let castling_move = Move::new(E1, C1, None, MoveType::QueensideCastling);
    assert!(available_moves.contains(&castling_move));

    b.apply_move(castling_move);

    assert_eq!(b.get(E1), None);
    assert_eq!(b.get(D1), Some(Piece::new(PieceType::Rook, Color::White)));
    assert_eq!(b.get(C1), Some(Piece::new(PieceType::King, Color::White)));
    assert_eq!(b.get(B1), None);
    assert_eq!(b.get(A1), None);
}
