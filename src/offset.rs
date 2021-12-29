use crate::hex::Hex;
use std::ops::Add;

#[derive(Clone, Debug, Copy)]
pub struct Offset {
    pub col: i32,
    pub row: i32,
}

impl Offset {
    pub fn to_hex(self) -> Hex {
        let row = &self.row;
        let col = &self.col;
        let s = row - (col + (col % 2)) / 2;

        let q = self.col;
        let r = self.row;

        Hex { q, r, s }
    }
}

impl Add for Offset {
    type Output = Offset;

    fn add(self, rhs: Self) -> Self::Output {
        Offset {
            col: self.col + rhs.col,
            row: self.row + rhs.row,
        }
    }
}
