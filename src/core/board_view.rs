use crate::core::{color::Color, piece::Piece, square::Square};

pub trait BoardView {
    fn get(&self, square: Square) -> Option<Piece>;
    fn has_kingside_castling_right(&self, color: Color) -> bool;
    fn has_queenside_castling_right(&self, color: Color) -> bool;
}
