use crate::{
    board::{Board, CastlingType},
    core::{
        chess_move::{Move, MoveType},
        color::Color,
        piece::{Piece, PieceType},
        square::Square,
    },
};

const KINGSIDE_CLEAR_SQUARES_RELATIVE: [(i8, i8); 2] = [(1, 0), (2, 0)];
const KINGSIDE_DEST_SQUARE_RELATIVE: (i8, i8) = (2, 0);

const QUEENSIDE_CLEAR_SQUARES_RELATIVE: [(i8, i8); 3] = [(-1, 0), (-2, 0), (-3, 0)];
const QUEENSIDE_DEST_SQUARE_RELATIVE: (i8, i8) = (-2, 0);

fn is_square_castle_clear(board: &Board, square: Square, castling_color: Color) -> bool {
    board.get(square).is_none() && !board.is_under_threat(square, !castling_color)
}

fn can_castle(
    board: &Board,
    king_square: Square,
    color: Color,
    castling_type: CastlingType,
) -> bool {
    if !board.allowed_to_castle(color, castling_type) {
        return false;
    }

    let rook_square = match (castling_type, color) {
        (CastlingType::Kingside, Color::White) => Square::H1,
        (CastlingType::Queenside, Color::White) => Square::A1,
        (CastlingType::Kingside, Color::Black) => Square::H8,
        (CastlingType::Queenside, Color::Black) => Square::A8,
    };

    if board.get(rook_square) != Some(Piece::new(PieceType::Rook, color)) {
        return false;
    }

    let clear_squares_relative: &[(i8, i8)] = match castling_type {
        CastlingType::Kingside => &KINGSIDE_CLEAR_SQUARES_RELATIVE,
        CastlingType::Queenside => &QUEENSIDE_CLEAR_SQUARES_RELATIVE,
    };

    for &(df, dr) in clear_squares_relative {
        if !is_square_castle_clear(
            board,
            king_square.get_relative_square(df, dr).unwrap(),
            color,
        ) {
            return false;
        }
    }

    true
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
        if can_castle(board, src_square, piece.piece_color, castling_type) {
            let (df, dr) = match castling_type {
                CastlingType::Kingside => KINGSIDE_DEST_SQUARE_RELATIVE,
                CastlingType::Queenside => QUEENSIDE_DEST_SQUARE_RELATIVE,
            };

            let dest = src_square.get_relative_square(df, dr).unwrap();

            moves.push(Move::new(
                src_square,
                dest,
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
