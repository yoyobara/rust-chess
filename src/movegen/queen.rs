use crate::core::{board_view::BoardView, chess_move::Move, piece::Piece, square::Square};

use super::{get_bishop_pseudo_legal_moves, get_rook_pseudo_legal_moves};

pub fn get_queen_pseudo_legal_moves(
    board: &impl BoardView,
    src_square: Square,
    piece: Piece,
) -> Vec<Move> {
    let mut moves = Vec::new();

    moves.append(&mut get_bishop_pseudo_legal_moves(board, src_square, piece));
    moves.append(&mut get_rook_pseudo_legal_moves(board, src_square, piece));

    moves
}
