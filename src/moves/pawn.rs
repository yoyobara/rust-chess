use crate::{
    board::Board,
    core::{chess_move::Move, piece::Piece, square::Square},
};

pub fn get_pawn_moves(piece: Piece, src_square: Square, board: &Board) -> Vec<Move> {
    let mut moves = Vec::new();

    // Determine direction and starting rank for double move
    use crate::core::{color::Color, piece::PieceType};

    let (forward_rank_delta, start_rank, promotion_rank) = match piece.piece_color {
        Color::White => (1, 1, 7),
        Color::Black => (-1, 6, 0),
    };

    // single forward
    if let Some(dst) = src_square.get_relative_square(0, forward_rank_delta) {
        if board.get(dst).is_none() {
            // promotion if moving to last rank
            if dst.to_index() as i8 == promotion_rank * 8 + (dst.to_file_rank().0) {
                // add all promotion piece types except Pawn and King
                for promo in [PieceType::Rook, PieceType::Knight, PieceType::Bishop, PieceType::Queen] {
                    moves.push(Move {
                        from: src_square,
                        to: dst,
                        promotion: Some(promo),
                    });
                }
            } else {
                moves.push(Move {
                    from: src_square,
                    to: dst,
                    promotion: None,
                });
            }

            // double forward from starting rank
            let (_, src_rank) = src_square.to_file_rank();
            if src_rank == start_rank {
                if let Some(dst2) = src_square.get_relative_square(0, forward_rank_delta * 2) {
                    // ensure both squares are empty
                    if board.get(dst2).is_none() {
                        moves.push(Move {
                            from: src_square,
                            to: dst2,
                            promotion: None,
                        });
                    }
                }
            }
        }
    }

    // captures
    for &df in &[-1, 1] {
        if let Some(dst) = src_square.get_relative_square(df, forward_rank_delta) {
            if let Some(target_piece) = board.get(dst) {
                if target_piece.piece_color != piece.piece_color {
                    // promotion on capture
                    if dst.to_index() as i8 == promotion_rank * 8 + (dst.to_file_rank().0) {
                        for promo in [PieceType::Rook, PieceType::Knight, PieceType::Bishop, PieceType::Queen] {
                            moves.push(Move {
                                from: src_square,
                                to: dst,
                                promotion: Some(promo),
                            });
                        }
                    } else {
                        moves.push(Move {
                            from: src_square,
                            to: dst,
                            promotion: None,
                        });
                    }
                }
            }
        }
    }

    moves
}
