use crate::{
    board::Board,
    castling::{CastlingType, get_castling_data},
    core::{
        chess_move::{Move, MoveType},
        color::Color,
        piece::{Piece, PieceType},
        square::Square,
    },
};

// checks only for rook existence and empty squares between
fn can_pseudo_castle(
    board: &Board,
    king_square: Square,
    color: Color,
    castling_type: CastlingType,
) -> bool {
    let castling_data = get_castling_data(castling_type, color);

    board.get(castling_data.rook_src_square) == Some(Piece::new(PieceType::Rook, color))
        && castling_data
            .clear_squares
            .iter()
            .all(|&clear_square| board.get(clear_square).is_none())
}

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

    for castling_type in [CastlingType::Kingside, CastlingType::Queenside] {
        if can_pseudo_castle(board, src_square, piece.piece_color, castling_type) {
            moves.push(Move::new(
                src_square,
                get_castling_data(castling_type, piece.piece_color).king_dest_square,
                None,
                match castling_type {
                    CastlingType::Kingside => MoveType::KingsideCastling,
                    CastlingType::Queenside => MoveType::QueensideCastling,
                },
            ));
        }
    }

    moves
}
