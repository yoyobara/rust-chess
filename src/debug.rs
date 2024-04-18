use std::fmt::{Debug, Write};
use crate::{chessboard::{ChessBoard, ChessPiece, Color, Spot, ChessPieceType}, spot};

impl Debug for ChessPiece {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use Color::*;
        use ChessPieceType::*;

        let mut unicode: u32 = match self.piece_type {
            // whites
            King => 0x2654,
            Queen => 0x2655,
            Rook => 0x2656,
            Bishop => 0x2657,
            Knight => 0x2658,
            Pawn => 0x2659,
        };

        // if black increase by 6 (look at unicde table, black pieces come exactly after white)
        if self.color == Black {
            unicode += 6;
        }

        f.write_char(char::from_u32(unicode).unwrap())
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

