#![allow(dead_code)]
use crate::chessboard::{ChessBoard, ChessPiece, ChessPieceType, Spot};


pub struct Move<'a> {
    board: &'a ChessBoard,
    src: Spot,
    dest: Spot
}

impl Move<'_> {

    /*
     * get the piece which is moving
     */
    fn src_piece(&self) -> ChessPiece {
        self.board.get(self.src).unwrap()
    }

    /*
     * get the piece which is captured, if any
     */
    fn dest_capture(&self) -> Option<ChessPiece> {
        self.board.get(self.dest)
    }
}

/*
 * color based
 */
macro_rules! pawn_direction {
    ($color:expr) => {
        if $color == crate::chessboard::Color::White {
            -1
        } else {
            1
        }
    };
}

impl Spot {
    fn mv(&self, drow: i32, dcol: i32) -> Spot {
        Spot { row: self.row + drow, column: self.column + dcol }
    }
}

impl ChessBoard {
    fn get_available_moves(&self, s: Spot) -> Vec<Move<'_>>{
        use ChessPieceType::*;

        let ChessPiece { piece_type, color } = self.get(s).unwrap();
        let mut available: Vec<Spot> = Vec::new();

        match piece_type {
            Pawn => {
                // straight forward
                let fwd = s.mv(1 * pawn_direction!(color), 0);
                if self.get(fwd).is_none() {
                    available.push(fwd)
                }

                // diagonal captures
                for j in [-1, 1] {
                    let diag_capture = s.mv(pawn_direction!(color), j);
                    if let Some(p) = self.get(diag_capture) {
                        if p.color == color.invert() {
                            available.push(diag_capture)
                        }
                    }
                }
            }
            _ => todo!()
        }

        available.iter().map(|sp| Move{ src: s, dest: *sp, board: self }).collect()
    }
}

