use crate::core::piece::PieceType::{self, *};

use super::{color::Color, piece::Piece};

type BoardState = [Option<Piece>; 64];

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

        let mut new_board: BoardState = [None; 64];

        for i in 0..8 {
            new_board[i] = Some(Piece::new(FIRST_ROW[i], Color::White));
            new_board[8 + i] = Some(Piece::new(Pawn, Color::White));
            new_board[48 + i] = Some(Piece::new(Pawn, Color::Black));
            new_board[56 + i] = Some(Piece::new(FIRST_ROW[i], Color::Black));
        }

        new_board
    }

    // pub fn get_psuedo_legal_moves(&self) {
    //     let current_turn_squares = self.state.iter().map(|x| x);
    // }

    pub const fn get_castling_rights(&self, color: Color) -> PlayerCastlingRights {
        self.castling_rights[color as usize]
    }

    pub fn pretty_print(&self) {
        for i in (0..8).rev() {
            println!("+---+---+---+---+---+---+---+---+");
            for j in 0..8 {
                let symbol = self.state[i * 8 + j].map(|p| p.to_ascii()).unwrap_or(' ');

                print!("| {} ", symbol);
            }
            println!("|");
        }
        println!("+---+---+---+---+---+---+---+---+");
    }
}
