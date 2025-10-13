use crate::core::{board_view::BoardView, chess_move::Move, piece::Piece, square::Square};

pub fn get_pawn_pseudo_legal_moves(
    board: &impl BoardView,
    src_square: Square,
    piece: Piece,
) -> Vec<Move> {
    unimplemented!()
}

pub fn get_bishop_pseudo_legal_moves(
    board: &impl BoardView,
    src_square: Square,
    piece: Piece,
) -> Vec<Move> {
    unimplemented!()
}

pub fn get_knight_pseudo_legal_moves(
    board: &impl BoardView,
    src_square: Square,
    piece: Piece,
) -> Vec<Move> {
    unimplemented!()
}

pub fn get_rook_pseudo_legal_moves(
    board: &impl BoardView,
    src_square: Square,
    piece: Piece,
) -> Vec<Move> {
    unimplemented!()
}

pub fn get_queen_pseudo_legal_moves(
    board: &impl BoardView,
    src_square: Square,
    piece: Piece,
) -> Vec<Move> {
    unimplemented!()
}

pub fn get_king_pseudo_legal_moves(
    board: &impl BoardView,
    src_square: Square,
    piece: Piece,
) -> Vec<Move> {
    unimplemented!()
}
