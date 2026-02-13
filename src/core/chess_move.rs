use crate::core::{piece::PieceType, square::Square};

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Move {
    pub from: Square,
    pub to: Square,
    pub captured: Option<PieceType>,
    pub promotion: Option<PieceType>,
}

impl Move {
    pub fn new(
        from: Square,
        to: Square,
        captured: Option<PieceType>,
        promotion: Option<PieceType>,
    ) -> Self {
        Self {
            from,
            to,
            promotion,
            captured,
        }
    }
}
