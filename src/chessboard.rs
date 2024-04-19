#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Color {
    White,
    Black,
}

impl Color {
    // black -> white, white -> black
    pub fn invert(self) -> Self {
        if self == Self::Black {
            Self::White
        } else {
            Self::Black
        }
    }
}

#[derive(Clone, Copy)]
pub enum ChessPieceType {
    King,
    Queen,
    Rook,
    Bishop,
    Knight,
    Pawn,
}

#[derive(Clone, Copy)]
pub struct ChessPiece {
    pub piece_type: ChessPieceType,
    pub color: Color,
}

/*
 * easily create a piece from type and color
 */
#[macro_export] 
macro_rules! piece {
    ($type:expr, $color:expr) => {
        (ChessPiece{piece_type: $type, color: $color})
    };
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
        use ChessPieceType::*;
        use Color::*;

        const PIECE_ORDER: [ChessPieceType; 8] = [
            Rook,
            Knight,
            Bishop,
            Queen,
            King,
            Bishop,
            Knight,
            Rook,
        ];

        let mut board = ChessBoard::new_empty();

        for (first_row_index, second_row_index, color) in [(7i32, 6, White), (0i32, 1, Black)] {

            for (i, &piece_type) in PIECE_ORDER.iter().enumerate() {
                let i = i as i32;

                board.set(spot!(first_row_index, i), piece!(piece_type, color));
                board.set(spot!(second_row_index, i), piece!(Pawn, color));
            }
        }

        board
    }
}
