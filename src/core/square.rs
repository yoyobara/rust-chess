use std::ops::Add;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Square(pub u8, pub u8);

impl Square {
    pub fn name(&self) -> String {
        format!("{}{}", (b'a' + self.0) as char, self.1 + 1)
    }
}

impl Add<(u8, u8)> for Square {
    type Output = Self;

    fn add(self, rhs: (u8, u8)) -> Self::Output {
        Self(self.0 + rhs.0, self.1 + rhs.1)
    }
}