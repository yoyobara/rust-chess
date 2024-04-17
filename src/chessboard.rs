#[derive(Clone, Copy)]
pub enum Color {
    White,
    Black,
}

#[derive(Clone, Copy)]
pub enum ChessPiece {
    King(Color),
    Queen(Color),
    Rook(Color),
    Bishop(Color),
    Knight(Color),
    Pawn(Color),
}

pub struct ChessBoard {
    state: [Option<ChessPiece>; 64],
}

impl ChessBoard {
    /*
     * get the piece in a certain square
     */
    pub fn get(&self, row: usize, column: usize) -> Option<ChessPiece> {
        self.state[row * 8 + column]
    }

    /*
     * set the piece in a certain square
     * NOTE this does not make sure a piece is not overriden!!
     */
    pub fn set(&mut self, row: usize, column: usize, piece: ChessPiece) {
        self.state[row * 8 + column] = Some(piece);
    }

    /*
     * remove the piece in a certain square
     */
    pub fn set_none(&mut self, row: usize, column: usize) {
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
