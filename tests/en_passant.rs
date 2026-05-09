use rust_chess::{
    board::Board,
    core::{
        chess_move::{Move, MoveType},
        color::Color,
        piece::{Piece, PieceType},
        square::Square::*,
    },
};

#[test]
fn test_en_passant_capture() {
    let mut board = Board::empty();

    board.set(E1, Some(Piece::new(PieceType::King, Color::White)));
    board.set(E8, Some(Piece::new(PieceType::King, Color::Black)));

    board.set(E5, Some(Piece::new(PieceType::Pawn, Color::White)));
    board.set(D7, Some(Piece::new(PieceType::Pawn, Color::Black)));

    board.apply_move(Move::new(D7, D5, None, MoveType::Quiet));

    assert_eq!(board.en_passant_square(), Some(D6));

    let moves = board.get_legal_moves(E5).unwrap();

    let en_passant_move = moves.iter().find(|m| m.move_type == MoveType::EnPassant);
    assert!(en_passant_move.is_some());
    let mv = en_passant_move.unwrap();
    assert_eq!(mv.to, D6);

    board.apply_move(*mv);

    assert_eq!(board.get(D6).unwrap().piece_type, PieceType::Pawn);
    assert_eq!(board.get(D6).unwrap().piece_color, Color::White);
    assert!(board.get(D5).is_none());
    assert!(board.get(E5).is_none());
}

#[test]
fn test_en_passant_only_available_immediately() {
    let mut board = Board::empty();

    board.set(E1, Some(Piece::new(PieceType::King, Color::White)));
    board.set(E8, Some(Piece::new(PieceType::King, Color::Black)));

    board.set(E5, Some(Piece::new(PieceType::Pawn, Color::White)));
    board.set(D7, Some(Piece::new(PieceType::Pawn, Color::Black)));
    board.set(A2, Some(Piece::new(PieceType::Pawn, Color::White)));

    board.apply_move(Move::new(D7, D5, None, MoveType::Quiet));
    assert_eq!(board.en_passant_square(), Some(D6));

    board.apply_move(Move::new(A2, A3, None, MoveType::Quiet));

    assert_eq!(board.en_passant_square(), None);

    let moves = board.get_legal_moves(E5).unwrap();

    let en_passant_move = moves.iter().find(|m| m.move_type == MoveType::EnPassant);
    assert!(en_passant_move.is_none());
}
