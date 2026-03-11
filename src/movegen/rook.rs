use crate::{
    board::Board,
    core::{
        chess_move::{Move, MoveType},
        piece::Piece,
        square::Square,
    },
};

pub fn get_rook_pseudo_legal_moves(board: &Board, src_square: Square, piece: Piece) -> Vec<Move> {
    let mut moves = Vec::new();

    for (i, j) in [(1, 0), (-1, 0), (0, 1), (0, -1)] {
        let mut n = 1;

        while let Some(dst) = src_square.get_relative_square(i * n, j * n) {
            if let Some(target_piece) = board.get(dst) {
                if target_piece.piece_color != piece.piece_color {
                    moves.push(Move::new(src_square, dst, None, MoveType::Capture));
                }
                break;
            } else {
                moves.push(Move::new(src_square, dst, None, MoveType::Quiet));
            }

            n += 1;
        }
    }

    moves
}
