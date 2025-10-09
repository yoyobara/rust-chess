use super::color::Color;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum PieceType {
	Pawn = 0,
	Rook = 1,
	Knight = 2,
	Bishop = 3,
	King = 4,
	Queen = 5
}

impl PieceType {
	pub const fn to_ascii(self) -> char {
		use PieceType::*;

		match self {
		    Pawn => 'p',
		    Rook => 'r',
		    Knight => 'n',
		    Bishop => 'b',
		    King => 'k',
		    Queen => 'q',
		}
	}
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Piece{
	pub piece_type: PieceType,
	pub piece_color: Color
}

impl Piece {
	pub const fn new(piece_type: PieceType, piece_color: Color) -> Self {
		Piece {piece_type, piece_color}
	}

	pub const fn to_ascii(self) -> char {
		let letter = self.piece_type.to_ascii(); 

		match self.piece_color {
		    Color::White => letter.to_ascii_uppercase(),
		    Color::Black => letter,
		}
	}
}