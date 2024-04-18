#![allow(dead_code)]
use crate::{chess::ChessEngine, chessboard::{ChessBoard, ChessPiece, Spot}};


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

impl ChessBoard {
    fn get_available_moves(&self, s: Spot) {
    }
}
