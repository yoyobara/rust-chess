use crate::core::piece::PieceType::{self, *};

use super::{color::Color, piece::Piece};

type BoardState = [[Option<Piece>; 8]; 8];

#[derive(Copy, Clone)]
pub struct PlayerCastlingRights {
    pub queenside: bool,
    pub kingside: bool,
}

pub struct Board {
    state: BoardState,
    turn: Color,
    castling_rights: [PlayerCastlingRights; 2],
}

impl Board {
    pub fn new() -> Self {
        Self {
            state: Self::get_initial_state(),
            turn: Color::White,
            castling_rights: [PlayerCastlingRights {
                queenside: true,
                kingside: true,
            }; 2],
        }
    }

    fn get_initial_state() -> BoardState {
        const FIRST_ROW: [PieceType; 8] = [Rook, Knight, Bishop, Queen, King, Bishop, Knight, Rook];
        const SECOND_ROW: [PieceType; 8] = [Pawn; 8];

        let mut new_board = [[None; 8]; 8];

        new_board[0] = FIRST_ROW.map(|t| Some(Piece::new(t, Color::White)));
        new_board[1] = SECOND_ROW.map(|t| Some(Piece::new(t, Color::White)));
        new_board[7] = FIRST_ROW.map(|t| Some(Piece::new(t, Color::Black)));
        new_board[6] = SECOND_ROW.map(|t| Some(Piece::new(t, Color::Black)));

        new_board
    }

    pub const fn get_castling_rights(&self, color: Color) -> PlayerCastlingRights {
        self.castling_rights[color as usize]
    }

    pub fn pretty_print(&self) {
        for row in self.state {
            println!("+---+---+---+---+---+---+---+---+");
            for piece in row {
                let symbol = piece.map(|p| p.to_ascii()).unwrap_or(' ');

                print!("| {} ", symbol);
            }
            println!("|");
        }
        println!("+---+---+---+---+---+---+---+---+");
    }
}
