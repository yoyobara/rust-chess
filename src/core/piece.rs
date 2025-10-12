use crate::core::{
    board::{Board, Move},
    square::Square,
};

use super::color::Color;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum PieceType {
    Pawn = 0,
    Rook = 1,
    Knight = 2,
    Bishop = 3,
    King = 4,
    Queen = 5,
}

impl PieceType {
    pub const fn to_ascii(self) -> char {
        use PieceType::*;

        match self {
            Pawn => 'p',
            Rook => 'r',
            Knight => 'n',
            Bishop => 'b',
            King => 'k',
            Queen => 'q',
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Piece {
    pub piece_type: PieceType,
    pub piece_color: Color,
}

impl Piece {
    pub const fn new(piece_type: PieceType, piece_color: Color) -> Self {
        Piece {
            piece_type,
            piece_color,
        }
    }

    pub const fn to_ascii(self) -> char {
        let letter = self.piece_type.to_ascii();

        match self.piece_color {
            Color::White => letter.to_ascii_uppercase(),
            Color::Black => letter,
        }
    }

    pub fn get_psuedo_legal_moves(&self, from: Square, board: &Board) -> Vec<Move> {
        match self.piece_type {
            PieceType::Pawn => self.get_pawn_moves(from, board),
            _ => vec![],
        }
    }

    fn get_pawn_moves(&self, from: Square, board: &Board) -> Vec<Move> {
        let mut moves = Vec::new();
        let direction: i8 = match self.piece_color {
            Color::White => 1,
            Color::Black => -1,
        };

        let one_step_forward = from.get_relative_square(0, direction);
        if let Some(square) = one_step_forward {
            if board.get(square).is_none() {
                moves.push(Move {
                    from,
                    to: square,
                    promotion: None,
                });

                // Check for two steps forward from the initial position
                let initial_row = match self.piece_color {
                    Color::White => 1,
                    Color::Black => 6,
                };
                if from.to_file_rank().1 == initial_row {
                    if let Some(two_steps_forward) = from.get_relative_square(0, 2 * direction) {
                        if board.get(two_steps_forward).is_none() {
                            moves.push(Move {
                                from,
                                to: two_steps_forward,
                                promotion: None,
                            });
                        }
                    }
                }
            }
        }

        // Captures
        for &file_offset in &[-1, 1] {
            if let Some(capture_square) = from.get_relative_square(file_offset, direction) {
                if let Some(target_piece) = board.get(capture_square) {
                    if target_piece.piece_color != self.piece_color {
                        moves.push(Move {
                            from,
                            to: capture_square,
                            promotion: None,
                        });
                    }
                }
            }
        }

        moves
    }
}
