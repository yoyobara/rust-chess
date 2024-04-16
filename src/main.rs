mod chess;
use chess::ChessBoard;

fn main() {
    let c: ChessBoard = ChessBoard::new_traditional();
    println!("{:?}", c);
}
