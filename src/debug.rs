use std::fmt::{Debug, Write};
use crate::{chessboard::{ChessBoard, ChessPiece, Color, Spot}, spot};

impl Debug for ChessPiece {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use Color::*;

        f.write_char(match self {
            // whites
            Self::King(White) => '\u{2654}',
            Self::Queen(White) => '\u{2655}',
            Self::Rook(White) => '\u{2656}',
            Self::Bishop(White) => '\u{2657}',
            Self::Knight(White) => '\u{2658}',
            Self::Pawn(White) => '\u{2659}',

            // blacks
            Self::King(Black) => '\u{265a}',
            Self::Queen(Black) => '\u{265b}',
            Self::Rook(Black) => '\u{265c}',
            Self::Bishop(Black) => '\u{265d}',
            Self::Knight(Black) => '\u{265e}',
            Self::Pawn(Black) => '\u{265f}',
        })
    }
}

impl Debug for ChessBoard {

    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {

        for i in 0..8 {
            for j in 0..8 {
                if let Some(p) = self.get(spot!(i, j)) {
                    write!(f, "{:?} ", p)?;
                }
                else {
                    f.write_str("- ")?;
                }
            }
            f.write_char('\n')?;
        }
        Ok(())
    }
}

