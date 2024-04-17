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

#[derive(Clone)]
pub struct ChessBoard {
    state: [[Option<ChessPiece>; 8]; 8],
}

// TODO maybe make non Copy?
#[derive(Clone, Copy)]
pub struct Spot {
    pub row: i32,
    pub column: i32
}

/*
 * easily create a spot from row and column
 */
#[macro_export] 
macro_rules! spot {
    ($r:expr, $c:expr) => {
        (Spot{row: $r, column: $c})
    };
}

impl ChessBoard {
    /*
     * get the piece in a certain square
     */
    pub fn get(&self, s: Spot) -> Option<ChessPiece> {
        self.state[s.row as usize][s.column as usize]
    }

    /*
     * set the piece in a certain square
     * NOTE this does not make sure a piece is not overriden!!
     */
    pub fn set(&mut self, s: Spot, piece: ChessPiece) {
        self.state[s.row as usize][s.column as usize] = Some(piece);
    }

    /*
     * remove the piece in a certain square
     */
    pub fn set_none(&mut self, s: Spot) {
        self.state[s.row as usize][s.column as usize] = None;
    }

    /*
     * create a new empty ChessBoard
     */
    fn new_empty() -> ChessBoard {
        ChessBoard { state: [[None; 8]; 8] }
    }

    /*
     * create a new ChessBoard with traditional pieces
     */
    pub fn new_traditional() -> ChessBoard {
        use ChessPiece::*;
        use Color::*;

        let mut board = ChessBoard::new_empty();

        for (first_row_index, second_row_index, color) in [(7i32, 6, White), (0i32, 1, Black)] {
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
                let i = i as i32;

                board.set(spot!(first_row_index, i), piece);
                board.set(spot!(second_row_index, i), Pawn(color));
            }
        }

        board
    }
}
