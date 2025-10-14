use crate::core::{
    board_view::BoardView,
    chess_move::Move,
    color::Color,
    piece::{Piece, PieceType},
    square::Square,
};

pub fn get_pawn_pseudo_legal_moves(
    board: &impl BoardView,
    src_square: Square,
    piece: Piece,
) -> Vec<Move> {
    use PieceType::*;

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
                // add promotions to Rook, Knight, Bishop, Queen
                moves.push(Move::new(src_square, dst, Some(Rook)));
                moves.push(Move::new(src_square, dst, Some(Knight)));
                moves.push(Move::new(src_square, dst, Some(Bishop)));
                moves.push(Move::new(src_square, dst, Some(Queen)));
            } else {
                moves.push(Move::new(src_square, dst, None));
                // double forward from start rank
                if rank == start_rank {
                    let dst2 = src_square.get_relative_square(0, forward_dir * 2).unwrap();
                    if board.get(dst2).is_none() {
                        moves.push(Move::new(src_square, dst2, None));
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
                        moves.push(Move::new(src_square, dst, Some(Rook)));
                        moves.push(Move::new(src_square, dst, Some(Knight)));
                        moves.push(Move::new(src_square, dst, Some(Bishop)));
                        moves.push(Move::new(src_square, dst, Some(Queen)));
                    } else {
                        moves.push(Move::new(src_square, dst, None));
                    }
                }
            }
        }
    }

    moves
}

pub fn get_bishop_pseudo_legal_moves(
    board: &impl BoardView,
    src_square: Square,
    piece: Piece,
) -> Vec<Move> {
    let mut moves = Vec::new();

    for (i, j) in [(-1, -1), (-1, 1), (1, -1), (1, 1)] {
        let mut n = 1;
        while let Some(dst) = src_square.get_relative_square(i * n, j * n) {
            if let Some(target_piece) = board.get(dst) {
                if target_piece.piece_color != piece.piece_color {
                    moves.push(Move::new(src_square, dst, None));
                }
                break;
            } else {
                moves.push(Move::new(src_square, dst, None));
            }
            n += 1;
        }
    }

    moves
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
    let mut moves = Vec::new();

    for (i, j) in [(1, 0), (-1, 0), (0, 1), (0, -1)] {
        let mut n = 1;

        while let Some(dst) = src_square.get_relative_square(i, j) {
            if let Some(target_piece) = board.get(dst) {
                if target_piece.piece_color != piece.piece_color {
                    moves.push(Move::new(src_square, dst, None));
                }
                break;
            } else {
                moves.push(Move::new(src_square, dst, None));
            }

            n += 1;
        }
    }

    moves
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
