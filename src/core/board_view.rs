use crate::core::{piece::Piece, square::Square};

pub trait BoardView {
    fn get(&self, square: Square) -> Option<Piece>;
}
