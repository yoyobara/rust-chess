mod chessboard;
mod debug;
mod chess;

use chessboard::ChessBoard;

fn main() {
    let c: ChessBoard = ChessBoard::new_traditional();
    println!("{:?}", c);
}
