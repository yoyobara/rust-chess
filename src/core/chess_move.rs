use crate::core::{piece::PieceType, square::Square};

#[derive(Debug, Copy, Clone)]
pub struct Move {
    pub from: Square,
    pub to: Square,
    pub promotion: Option<PieceType>,
}
