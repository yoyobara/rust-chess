#![allow(dead_code)] // TODO

#[derive(Clone, Copy)]
enum Color {
    White,
    Black,
}

#[derive(Clone, Copy)]
enum ChessPiece {
    Pawn(Color),
    Rook(Color),
    Bishop(Color),
    Knight(Color),
    King(Color),
    Queen(Color),
}

struct ChessBoard {
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
     * create a new empty ChessBoard
     */
    fn new_empty() -> ChessBoard {
        ChessBoard { state: [None; 64] }
    }

    /*
     * create a new ChessBoard with traditional pieces
     */
    fn new_traditional() -> ChessBoard {
        use ChessPiece::*;
        use Color::*;

        let mut board = ChessBoard::new_empty();

        for (first_row_index, color) in [(0, White), (7, Black)] {
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
            }
        }

        board
    }
}

/*
 * a struct which represents and manages a game of chess.
 */
struct ChessEngine {}
