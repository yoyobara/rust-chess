use crate::core::{
    chess_move::Move,
    color::Color,
    piece::{Piece, PieceType},
    square::{ALL_SQUARES, Square},
};

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

    pub const fn get(&self, square: Square) -> Option<Piece> {
        self.state[square.to_index() as usize]
    }

    fn get_initial_state() -> BoardState {
        use PieceType::*;

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

    pub fn get_psuedo_legal_moves(&self) -> Vec<Move> {
        use PieceType::*;

        let my_turn_pieces = ALL_SQUARES.iter().filter_map(|&sq| {
            let piece = self.get(sq)?;
            if piece.piece_color == self.turn {
                Some((sq, piece))
            } else {
                None
            }
        });

        let mut moves = Vec::new();

        for (src_square, piece) in my_turn_pieces {
            moves.extend(match piece.piece_type {
                Pawn => self.get_pawn_psuedo_legal_moves(src_square),
                Bishop => self.get_bishop_psuedo_legal_moves(src_square),
                Knight => self.get_knight_psuedo_legal_moves(src_square),
                Rook => self.get_rook_psuedo_legal_moves(src_square),
                Queen => self.get_queen_psuedo_legal_moves(src_square),
                King => self.get_king_psuedo_legal_moves(src_square),
            });
        }

        moves
    }

    fn get_pawn_psuedo_legal_moves(&self, src_square: Square) -> Vec<Move> {
        unimplemented!()
    }

    fn get_bishop_psuedo_legal_moves(&self, src_square: Square) -> Vec<Move> {
        unimplemented!()
    }

    fn get_knight_psuedo_legal_moves(&self, src_square: Square) -> Vec<Move> {
        unimplemented!()
    }

    fn get_rook_psuedo_legal_moves(&self, src_square: Square) -> Vec<Move> {
        unimplemented!()
    }

    fn get_queen_psuedo_legal_moves(&self, src_square: Square) -> Vec<Move> {
        unimplemented!()
    }

    fn get_king_psuedo_legal_moves(&self, src_square: Square) -> Vec<Move> {
        unimplemented!()
    }
}
