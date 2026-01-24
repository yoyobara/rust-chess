use std::ops::Not;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(
    feature = "serde-support",
    derive(serde::Serialize, serde::Deserialize)
)]
pub enum Color {
    White = 0,
    Black = 1,
}

impl Not for Color {
    type Output = Self;

    fn not(self) -> <Self as Not>::Output {
        match self {
            Self::Black => Self::White,
            Self::White => Self::Black,
        }
    }
}
