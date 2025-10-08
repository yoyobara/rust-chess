use super::color::Color;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
enum PieceType {
	Pawn,
	Rook,
	Knight,
	Bishop,
	King,
	Queen
}

impl PieceType {
	const fn to_ascii(self) -> char {
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
struct Piece(PieceType, Color);

impl Piece {
	const fn to_ascii(self) -> char {
		let letter = self.0.to_ascii(); 

		match self.1 {
		    Color::White => letter.to_ascii_uppercase(),
		    Color::Black => letter,
		}
	}
}