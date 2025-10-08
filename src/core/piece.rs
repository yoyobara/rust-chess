use super::color::Color;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum PieceType {
	Pawn,
	Rook,
	Knight,
	Bishop,
	King,
	Queen
}

impl PieceType {
	pub const fn to_ascii(self) -> char {
		match self {
		    PieceType::Pawn => 'p',
		    PieceType::Rook => 'r',
		    PieceType::Knight => 'n',
		    PieceType::Bishop => 'b',
		    PieceType::King => 'k',
		    PieceType::Queen => 'q',
		}
	}
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Piece{
	pub piece_type: PieceType,
	pub piece_color: Color
}

impl Piece {
	pub const fn to_ascii(self) -> char {
		let letter = self.piece_type.to_ascii(); 

		match self.piece_color {
		    Color::White => letter.to_ascii_uppercase(),
		    Color::Black => letter,
		}
	}
}