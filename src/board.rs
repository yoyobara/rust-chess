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
    castling_rights: [PlayerCastlingRights; 2],
}

impl Board {
    pub fn new() -> Self {
        Self {
            state: Self::get_initial_state(),
            castling_rights: [PlayerCastlingRights {
                queenside: true,
                kingside: true,
            }; 2],
        }
    }

    pub fn get(&self, square: Square) -> Option<Piece> {
        self.state[square.to_index() as usize]
    }

    pub fn get_mut(&mut self, square: Square) -> &mut Option<Piece> {
        &mut self.state[square.to_index() as usize]
    }

    pub fn set(&mut self, square: Square, piece: Option<Piece>) {
        *self.get_mut(square) = piece;
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

    pub fn find_piece(&self, piece: Option<Piece>) -> impl Iterator<Item = Square> {
        ALL_SQUARES
            .iter()
            .copied()
            .filter(move |&sq| self.get(sq) == piece)
    }

    pub fn find_king(&self, color: Color) -> Square {
        let mut king_squares = self.find_piece(Some(Piece::new(PieceType::King, color)));
        let only_king_square = king_squares.next().expect("king not found");

        assert!(king_squares.next().is_none());

        only_king_square
    }

    pub fn apply_move(&mut self, mv: Move) {
        let mut moved_piece = self
            .get_mut(mv.from)
            .take()
            .expect("can't move empty square");

        let captured_piece = self.get(mv.to);
        assert_eq!(mv.captured, captured_piece.map(|p| p.piece_type));

        if let Some(promotion_type) = mv.promotion {
            moved_piece.piece_type = promotion_type;
        }

        *self.get_mut(mv.to) = Some(moved_piece);
    }

    pub fn revert_move(&mut self, mv: Move) {
        let mut moved_piece = self
            .get_mut(mv.to)
            .take()
            .expect("move's destination is empty");

        if let Some(promoted_piece) = mv.promotion {
            moved_piece.piece_type = PieceType::Pawn;
        }

        *self.get_mut(mv.to) = mv
            .captured
            .map(|cap_type| Piece::new(cap_type, !moved_piece.piece_color));
        *self.get_mut(mv.from) = Some(moved_piece);
    }

    pub fn get_pseudo_legal_moves(&self, square: Square) -> Option<Vec<Move>> {
        use PieceType::*;

        let piece = self.get(square)?;

        Some(match piece.piece_type {
            Pawn => get_pawn_pseudo_legal_moves(self, square, piece),
            Bishop => get_bishop_pseudo_legal_moves(self, square, piece),
            Knight => get_knight_pseudo_legal_moves(self, square, piece),
            Rook => get_rook_pseudo_legal_moves(self, square, piece),
            Queen => get_queen_pseudo_legal_moves(self, square, piece),
            King => get_king_pseudo_legal_moves(self, square, piece),
        })
    }

    pub fn get_all_pseudo_legal_moves(&self, color: Color) -> Vec<Move> {
        self.squares_of_piece_color(color)
            .flat_map(|sq| self.get_pseudo_legal_moves(sq).unwrap())
            .collect()
    }

    pub fn get_legal_moves(&self, square: Square) -> Option<Vec<Move>> {
        let color = self.get(square)?.piece_color;
        let pseudo_legal = self.get_pseudo_legal_moves(square)?;

        Some(
            pseudo_legal
                .iter()
                .filter(|&&mv| {
                    let mut board_clone = self.clone();

                    board_clone.apply_move(mv);
                    board_clone
                        .get_all_pseudo_legal_moves(!color)
                        .iter()
                        .all(|&mv| mv.captured != Some(PieceType::King))
                })
                .copied()
                .collect(),
        )
    }

    pub fn get_all_legal_moves(&self, color: Color) -> Vec<Move> {
        self.squares_of_piece_color(color)
            .flat_map(|sq| self.get_legal_moves(sq).unwrap())
            .collect()
    }

    fn squares_of_piece_color(&self, color: Color) -> impl Iterator<Item = Square> {
        ALL_SQUARES
            .iter()
            .filter(move |&&sq| self.get(sq).map_or(false, |p| p.piece_color == color))
            .copied()
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
