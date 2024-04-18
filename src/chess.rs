#![allow(dead_code)]
use crate::chessboard::{ChessBoard, ChessPiece, Color, Spot};

/*
 * a struct which represents and manages a game of chess.
 */
#[derive(Debug)]
pub struct ChessEngine {
    white_player_name: String,
    black_player_name: String,
    chessboard: ChessBoard,
    current_turn: Color,
}

impl ChessEngine {

    /*
     * creates a new ChessEngine
     * TODO consider builder pattern
     */
    pub fn new() -> ChessEngine {
        ChessEngine {
            white_player_name: "white_player".to_owned(),
            black_player_name: "black_player".to_owned(),
            chessboard: ChessBoard::new_traditional(),
            current_turn: Color::White
        }
    }

    fn swap_turn(&mut self) {
        self.current_turn = match self.current_turn {
            Color::White => Color::Black,
            Color::Black => Color::White
        }
    }

    /* configure the names of the players */
    pub fn configure_players(&mut self, white_name: String, black_name: String) {
        self.white_player_name = white_name;
        self.black_player_name = black_name;
    }


}
