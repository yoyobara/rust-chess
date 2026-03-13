use super::common::setup_board_for_white_castling;
use rust_chess::core::{
    chess_move::{Move, MoveType},
    color::Color,
    piece::{Piece, PieceType},
    square::Square::*,
};

#[test]
fn kingside_castling_blocked_by_piece() {
    let mut b = setup_board_for_white_castling();
    b.set(F1, Some(Piece::new(PieceType::Knight, Color::White)));

    let available_moves = b.get_legal_moves(E1).unwrap();
    let kingside = Move::new(E1, G1, None, MoveType::KingsideCastling);
    assert!(!available_moves.contains(&kingside));
}

#[test]
fn queenside_castling_blocked_by_piece() {
    let mut b = setup_board_for_white_castling();
    b.set(D1, Some(Piece::new(PieceType::Bishop, Color::White)));

    let available_moves = b.get_legal_moves(E1).unwrap();
    let queenside = Move::new(E1, C1, None, MoveType::QueensideCastling);
    assert!(!available_moves.contains(&queenside));
}

#[test]
fn castling_through_check() {
    let mut b = setup_board_for_white_castling();
    b.set(A6, Some(Piece::new(PieceType::Bishop, Color::Black))); // attacks F1

    let available_moves = b.get_legal_moves(E1).unwrap();
    let kingside = Move::new(E1, G1, None, MoveType::KingsideCastling);
    assert!(!available_moves.contains(&kingside));
}

#[test]
fn castling_into_check() {
    let mut b = setup_board_for_white_castling();
    b.set(G8, Some(Piece::new(PieceType::Rook, Color::Black))); // Attacks G1

    let available_moves = b.get_legal_moves(E1).unwrap();
    let kingside = Move::new(E1, G1, None, MoveType::KingsideCastling);
    assert!(!available_moves.contains(&kingside));
}

#[test]
fn castling_after_king_moved() {
    let mut b = setup_board_for_white_castling();
    b.apply_move(Move::new(E1, E2, None, MoveType::Quiet));
    b.apply_move(Move::new(E2, E1, None, MoveType::Quiet));

    let available_moves = b.get_legal_moves(E1).unwrap();
    let kingside = Move::new(E1, G1, None, MoveType::KingsideCastling);
    assert!(!available_moves.contains(&kingside));
}

#[test]
fn castling_after_rook_moved() {
    let mut b = setup_board_for_white_castling();
    b.apply_move(Move::new(H1, H2, None, MoveType::Quiet));
    b.apply_move(Move::new(H2, H1, None, MoveType::Quiet));

    let available_moves = b.get_legal_moves(E1).unwrap();
    let kingside = Move::new(E1, G1, None, MoveType::KingsideCastling);
    assert!(!available_moves.contains(&kingside));
}

#[test]
fn castling_without_rook() {
    let mut b = setup_board_for_white_castling();
    b.set(H1, None);

    let available_moves = b.get_legal_moves(E1).unwrap();
    let kingside = Move::new(E1, G1, None, MoveType::KingsideCastling);
    assert!(!available_moves.contains(&kingside));
}

#[test]
fn castling_with_king_in_check() {
    let mut b = setup_board_for_white_castling();
    b.set(E8, Some(Piece::new(PieceType::Rook, Color::Black))); // King attacked at E1

    let available_moves = b.get_legal_moves(E1).unwrap();
    let kingside = Move::new(E1, G1, None, MoveType::KingsideCastling);
    let queenside = Move::new(E1, C1, None, MoveType::QueensideCastling);
    assert!(!available_moves.contains(&kingside));
    assert!(!available_moves.contains(&queenside));
}
