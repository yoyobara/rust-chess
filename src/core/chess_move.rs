use crate::core::{piece::PieceType, square::Square};

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum MoveType {
    Quiet,
    Capture,
    QueensideCastling,
    KingsideCastling,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Move {
    pub from: Square,
    pub to: Square,
    pub promotion: Option<PieceType>,
    pub move_type: MoveType,
}

impl Move {
    pub fn new(
        from: Square,
        to: Square,
        promotion: Option<PieceType>,
        move_type: MoveType,
    ) -> Self {
        Self {
            from,
            to,
            promotion,
            move_type,
        }
    }
}
