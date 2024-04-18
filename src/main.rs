mod chessboard;
mod debug;
mod r#move;
mod chess;


use crate::chess::ChessEngine;

fn main() {
    let mut c = ChessEngine::new();
    c.configure_players("yotam".to_owned(), "stav".to_owned());

    println!("{:?}", c);
}
