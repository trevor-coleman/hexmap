use std::ops::Add;
use crate::hex::Hex;

pub struct HexDirection(usize);

impl HexDirection {
    pub const fn new(direction: i32) -> HexDirection {
        let mod_direction: usize = (direction % 6) as usize;
        HexDirection(mod_direction)
    }

    pub fn to_hex(&self) -> Hex {
        HEX_DIRECTIONS[self.0]
    }
}

impl Add for HexDirection {
    type Output = HexDirection;

    fn add(self, rhs: Self) -> Self::Output {
        HexDirection(self.0 + rhs.0)
    }
}

const HEX_DIRECTIONS: [Hex; 6] = [
    Hex::new(0, -1, 1),
    Hex::new(1, -1, 0),
    Hex::new(1, 0, -1),
    Hex::new(0, 1, -1),
    Hex::new(-1, 1, 0),
    Hex::new(-1, 0, 1),
];
