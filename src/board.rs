use crate::{
    core::{
        board_view::BoardView,
        chess_move::Move,
        color::Color,
        piece::{Piece, PieceType},
        square::{ALL_SQUARES, Square},
    },
    movegen::{
        get_bishop_pseudo_legal_moves, get_king_pseudo_legal_moves, get_knight_pseudo_legal_moves,
        get_pawn_pseudo_legal_moves, get_queen_pseudo_legal_moves, get_rook_pseudo_legal_moves,
    },
};

type BoardState = [Option<Piece>; 64];

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct PlayerCastlingRights {
    pub queenside: bool,
    pub kingside: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
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

    pub fn get(&self, square: Square) -> Option<Piece> {
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
        println!("{}", self);
    }

    pub fn get_pseudo_legal_moves(&self) -> Vec<Move> {
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
                Pawn => get_pawn_pseudo_legal_moves(self, src_square, piece),
                Bishop => get_bishop_pseudo_legal_moves(self, src_square, piece),
                Knight => get_knight_pseudo_legal_moves(self, src_square, piece),
                Rook => get_rook_pseudo_legal_moves(self, src_square, piece),
                Queen => get_queen_pseudo_legal_moves(self, src_square, piece),
                King => get_king_pseudo_legal_moves(self, src_square, piece),
            });
        }

        moves
    }
}

impl std::fmt::Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for i in (0..8).rev() {
            writeln!(f, "+---+---+---+---+---+---+---+---+")?;
            for j in 0..8 {
                let symbol = self.state[i * 8 + j].map(|p| p.to_ascii()).unwrap_or(' ');

                write!(f, "| {} ", symbol)?;
            }
            writeln!(f, "|")?;
        }
        writeln!(f, "+---+---+---+---+---+---+---+---+")
    }
}

impl BoardView for Board {
    fn get(&self, square: Square) -> Option<Piece> {
        self.get(square)
    }
}
