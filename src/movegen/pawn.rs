use crate::{
    board::Board,
    core::{
        chess_move::{Move, MoveType},
        color::Color,
        piece::{Piece, PieceType},
        square::Square,
    },
};
use PieceType::*;

const PROMOTABLE_PIECE_TYPES: [PieceType; 4] = [Rook, Knight, Bishop, Queen];

pub fn get_pawn_pseudo_legal_moves(board: &Board, src_square: Square, piece: Piece) -> Vec<Move> {
    let mut moves: Vec<Move> = Vec::new();
    let (file, rank) = src_square.to_file_rank();

    // Direction depends on color: white moves +1 rank, black moves -1 rank
    let (forward_dir, start_rank, promotion_rank) = match piece.piece_color {
        Color::White => (1, 1, 7),
        Color::Black => (-1, 6, 0),
    };

    // single forward
    if let Some(dst) = src_square.get_relative_square(0, forward_dir) {
        if board.get(dst).is_none() {
            // promotion when reaching last rank
            if dst.rank() == promotion_rank {
                for promotable_piece_type in PROMOTABLE_PIECE_TYPES {
                    moves.push(Move::new(
                        src_square,
                        dst,
                        Some(promotable_piece_type),
                        MoveType::Quiet,
                    ));
                }
            } else {
                moves.push(Move::new(src_square, dst, None, MoveType::Quiet));
                // double forward from start rank
                if rank == start_rank {
                    let dst2 = src_square.get_relative_square(0, forward_dir * 2).unwrap();
                    if board.get(dst2).is_none() {
                        moves.push(Move::new(src_square, dst2, None, MoveType::Quiet));
                    }
                }
            }
        }
    }

    // captures (diagonals)
    for &df in &[-1, 1] {
        if let Some(dst) = src_square.get_relative_square(df, forward_dir) {
            if let Some(target_piece) = board.get(dst) {
                if target_piece.piece_color != piece.piece_color {
                    if dst.rank() == promotion_rank {
                        for promotable_piece_type in PROMOTABLE_PIECE_TYPES {
                            moves.push(Move::new(
                                src_square,
                                dst,
                                Some(promotable_piece_type),
                                MoveType::Capture,
                            ));
                        }
                    } else {
                        moves.push(Move::new(src_square, dst, None, MoveType::Capture));
                    }
                }
            }
        }
    }

    moves
}
