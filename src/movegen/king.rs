use crate::{
    board::Board,
    core::{
        chess_move::{Move, MoveType},
        piece::Piece,
        square::Square,
    },
};

pub fn get_king_pseudo_legal_moves(board: &Board, src_square: Square, piece: Piece) -> Vec<Move> {
    let mut moves = Vec::new();

    for i in -1..=1 {
        for j in -1..=1 {
            if i == 0 && j == 0 {
                continue;
            }

            let Some(dest) = src_square.get_relative_square(i, j) else {
                continue;
            };

            match board.get(dest) {
                None => {
                    moves.push(Move::new(src_square, dest, None, MoveType::Quiet));
                }
                Some(target_piece) if target_piece.piece_color != piece.piece_color => {
                    moves.push(Move::new(src_square, dest, None, MoveType::Capture));
                }
                _ => {}
            }
        }
    }

    moves
}
