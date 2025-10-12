use crate::core::{
    piece::PieceType::{self, *},
    square::{ALL_SQUARES, Square},
};

use super::{color::Color, piece::Piece};

type BoardState = [Option<Piece>; 64];

#[derive(Copy, Clone)]
pub struct PlayerCastlingRights {
    pub queenside: bool,
    pub kingside: bool,
}

#[derive(Debug, Copy, Clone)]
pub struct Move {
    pub from: Square,
    pub to: Square,
    pub promotion: Option<PieceType>,
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

    pub const fn get(&self, square: Square) -> &Option<Piece> {
        &self.state[square.to_index() as usize]
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

    pub fn get_psuedo_legal_moves(&self) -> Vec<Move> {
        let mut moves = Vec::new();

        for src_square in ALL_SQUARES {
            if let Some(piece) = self.get(src_square) {
                if piece.piece_color == self.turn {
                    let piece_moves = piece.get_psuedo_legal_moves(src_square, self);
                    moves.extend(piece_moves);
                }
            }
        }

        moves
    }

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
