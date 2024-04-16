#![allow(dead_code)] // TODO

use std::fmt::{Debug, Write};

#[derive(Clone, Copy)]
enum Color {
    White,
    Black,
}

#[derive(Clone, Copy)]
enum ChessPiece {
    King(Color),
    Queen(Color),
    Rook(Color),
    Bishop(Color),
    Knight(Color),
    Pawn(Color),
}

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

pub struct ChessBoard {
    state: [Option<ChessPiece>; 64],
}

impl ChessBoard {
    /*
     * get the piece in a certain square
     */
    fn get(&self, row: usize, column: usize) -> Option<ChessPiece> {
        self.state[row * 8 + column]
    }

    /*
     * set the piece in a certain square
     * NOTE this does not make sure a piece is not overriden!!
     */
    fn set(&mut self, row: usize, column: usize, piece: ChessPiece) {
        self.state[row * 8 + column] = Some(piece);
    }

    /*
     * remove the piece in a certain square
     */
    fn set_none(&mut self, row: usize, column: usize) {
        self.state[row * 8 + column] = None;
    }

    /*
     * create a new empty ChessBoard
     */
    fn new_empty() -> ChessBoard {
        ChessBoard { state: [None; 64] }
    }

    /*
     * create a new ChessBoard with traditional pieces
     */
    pub fn new_traditional() -> ChessBoard {
        use ChessPiece::*;
        use Color::*;

        let mut board = ChessBoard::new_empty();

        for (first_row_index, second_row_index, color) in [(7, 6, White), (0, 1, Black)] {
            let piece_order = [
                Rook(color),
                Knight(color),
                Bishop(color),
                Queen(color),
                King(color),
                Bishop(color),
                Knight(color),
                Rook(color),
            ];

            for (i, &piece) in piece_order.iter().enumerate() {
                board.set(first_row_index, i, piece);
                board.set(second_row_index, i, Pawn(color))
            }
        }

        board
    }
}

impl Debug for ChessBoard {

    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {

        for i in 0..8 {
            for j in 0..8 {
                if let Some(p) = self.get(i, j) {
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

/*
 * a struct which represents and manages a game of chess.
 */
struct ChessEngine {}
