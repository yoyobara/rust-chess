#![allow(dead_code)]
use crate::chessboard::{ChessBoard, ChessPiece, ChessPieceType, Color, Spot};


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

impl ChessBoard {
    fn get_available_moves(&self, s: Spot) -> Vec<Move<'_>>{
        use ChessPieceType::*;

        let ChessPiece { piece_type, color: piece_color } = self.get(s).unwrap();
        let mut available: Vec<Spot> = Vec::new();

        match piece_type {
            Pawn => {
                // straight forward
                let fwd = s.mv(1 * pawn_direction!(piece_color), 0);
                if self.get(fwd).is_none() {
                    available.push(fwd)
                }

                // diagonal captures
                for j in [-1, 1] {
                    let diag_capture = s.mv(pawn_direction!(piece_color), j);
                    if let Some(ChessPiece {color, ..}) = self.get(diag_capture) {
                        if color == piece_color.invert() {
                            available.push(diag_capture)
                        }
                    }
                }
            }

            Rook => {
                // horizontal
                for dir in [-1, 1] {
                    let i = 1;
                    loop {
                        let spot_to_check = s.mv(0, i * dir);

                        if !spot_to_check.bounded() {
                            break;
                        }

                        // add only if empty spot or occupied with foe
                        match self.get(spot_to_check) {
                            Some(ChessPiece{color, ..}) => {
                                if color == piece_color.invert() {
                                    available.push(spot_to_check);
                                }
                            }
                            None => {
                                available.push(spot_to_check);
                            }
                        }
                    }
                }
            }

            _ => todo!()
        }

        available.iter().map(|sp| Move{ src: s, dest: *sp, board: self }).collect()
    }
}

