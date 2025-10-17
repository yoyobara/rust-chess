use crate::core::{board_view::BoardView, chess_move::Move, piece::Piece, square::Square};

pub fn get_king_pseudo_legal_moves(
    board: &impl BoardView,
    src_square: Square,
    piece: Piece,
) -> Vec<Move> {
    let mut moves = Vec::new();

    for i in -1..=1 {
        for j in -1..=1 {
            if i == 0 && j == 0 {
                continue;
            }

            if let Some(dst) = src_square.get_relative_square(i, j) {
                if let Some(target_piece) = board.get(dst) {
                    if target_piece.piece_color != piece.piece_color {
                        moves.push(Move::new(
                            src_square,
                            dst,
                            Some(target_piece.piece_type),
                            None,
                        ));
                    }
                } else {
                    moves.push(Move::new(src_square, dst, None, None));
                }
            }
        }
    }

    moves
}
