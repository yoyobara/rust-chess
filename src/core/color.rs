use std::ops::Not;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum Color {
	White = 0,
	Black = 1,
}

impl Not for Color {
	type Output = Self;

	fn not(self) -> <Self as Not>::Output { 
		match self {
			Self::Black => Self::White,
			Self::White => Self::Black
		}
	}
}