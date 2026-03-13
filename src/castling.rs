use crate::core::{
    chess_move::MoveType,
    color::Color,
    square::Square::{self, *},
};

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(
    feature = "serde-support",
    derive(serde::Serialize, serde::Deserialize)
)]
pub enum CastlingType {
    Kingside = 0,
    Queenside = 1,
}

pub struct CastlingData {
    pub clear_squares: &'static [Square],
    pub king_src_square: Square,
    pub king_dest_square: Square,
    pub rook_src_square: Square,
    pub rook_dest_square: Square,
}

pub const KINGSIDE_WHITE_CASTLING_DATA: CastlingData = CastlingData {
    clear_squares: &[F1, G1],
    king_src_square: E1,
    king_dest_square: G1,
    rook_src_square: H1,
    rook_dest_square: F1,
};

pub const QUEENSIDE_WHITE_CASTLING_DATA: CastlingData = CastlingData {
    clear_squares: &[D1, C1, B1],
    king_src_square: E1,
    king_dest_square: C1,
    rook_src_square: A1,
    rook_dest_square: D1,
};

pub const KINGSIDE_BLACK_CASTLING_DATA: CastlingData = CastlingData {
    clear_squares: &[F8, G8],
    king_src_square: E8,
    king_dest_square: G8,
    rook_src_square: H8,
    rook_dest_square: F8,
};

pub const QUEENSIDE_BLACK_CASTLING_DATA: CastlingData = CastlingData {
    clear_squares: &[D8, C8, B8],
    king_src_square: E8,
    king_dest_square: C8,
    rook_src_square: A8,
    rook_dest_square: D8,
};

pub fn get_castling_data(castling_type: CastlingType, color: Color) -> CastlingData {
    match (castling_type, color) {
        (CastlingType::Kingside, Color::White) => KINGSIDE_WHITE_CASTLING_DATA,
        (CastlingType::Queenside, Color::White) => QUEENSIDE_WHITE_CASTLING_DATA,
        (CastlingType::Kingside, Color::Black) => KINGSIDE_BLACK_CASTLING_DATA,
        (CastlingType::Queenside, Color::Black) => QUEENSIDE_BLACK_CASTLING_DATA,
    }
}

impl From<MoveType> for Option<CastlingType> {
    fn from(move_type: MoveType) -> Self {
        match move_type {
            MoveType::KingsideCastling => Some(CastlingType::Kingside),
            MoveType::QueensideCastling => Some(CastlingType::Queenside),
            _ => None,
        }
    }
}
