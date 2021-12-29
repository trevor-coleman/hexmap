use crate::hex::Hex;
use std::ops::{Add, Div, Mul, Neg, Sub};

#[derive(Debug, Copy, Clone)]
pub struct FractionalHex {
    pub q: f32,
    pub r: f32,
    pub s: f32,
}

impl FractionalHex {
    pub const fn new(q: f32, r: f32, s: f32) -> FractionalHex {
        FractionalHex { q, r, s }
    }
    pub fn add(self, rhs: FractionalHex) -> FractionalHex {
        FractionalHex {
            q: self.q + rhs.q,
            r: self.r + rhs.r,
            s: self.s + rhs.s,
        }
    }
    pub fn round(&self) -> Hex {
        let FractionalHex { q, r, s } = self;

        let q_diff = diff_from_round(q);
        let r_diff = diff_from_round(r);
        let s_diff = diff_from_round(s);

        if q_diff > r_diff && q_diff > s_diff {
            let hi_q = (-r.round() - s.round()) as i32;
            let r = r.round() as i32;
            let s = s.round() as i32;

            return Hex { q: hi_q, r, s };
        } else if r_diff > s_diff {
            let hi_r = (-q.round() - s.round()) as i32;
            let q = q.round() as i32;
            let s = s.round() as i32;

            return Hex { q, r: hi_r, s };
        }

        let hi_s = (-q.round() - r.round()) as i32;
        let q = q.round() as i32;
        let r = r.round() as i32;

        Hex { q, r, s: hi_s }
    }
}

fn diff_from_round(i: &f32) -> f32 {
    (i - i.round()).abs()
}

impl PartialEq for FractionalHex {
    fn eq(&self, rhs: &Self) -> bool {
        self.q == rhs.q && self.r == rhs.r && self.s == rhs.s
    }
}

impl Add for FractionalHex {
    type Output = FractionalHex;

    fn add(self, rhs: Self) -> Self::Output {
        FractionalHex {
            q: self.q + rhs.q,
            r: self.r + rhs.r,
            s: self.s + rhs.s,
        }
    }
}

impl Add<Hex> for FractionalHex {
    type Output = FractionalHex;

    fn add(self, rhs: Hex) -> Self::Output {
        FractionalHex {
            q: (rhs.q as f32) + self.q,
            r: (rhs.r as f32) + self.r,
            s: (rhs.s as f32) + self.s,
        }
    }
}

impl Sub for FractionalHex {
    type Output = FractionalHex;

    fn sub(self, rhs: Self) -> Self::Output {
        FractionalHex {
            q: self.q - rhs.q,
            r: self.r - rhs.r,
            s: self.s - rhs.s,
        }
    }
}

impl Sub<Hex> for FractionalHex {
    type Output = FractionalHex;

    fn sub(self, rhs: Hex) -> Self::Output {
        FractionalHex {
            q: self.q - (rhs.q as f32),
            r: self.r - (rhs.r as f32),
            s: self.s - (rhs.s as f32),
        }
    }
}

impl Mul<f32> for FractionalHex {
    type Output = FractionalHex;

    fn mul(self, rhs: f32) -> Self::Output {
        let rhs = &rhs;

        FractionalHex {
            q: self.q * rhs,
            r: self.r * rhs,
            s: self.s * rhs,
        }
    }
}

impl Div<f32> for FractionalHex {
    type Output = FractionalHex;

    fn div(self, rhs: f32) -> Self::Output {
        let rhs = &rhs;

        let q = self.q / rhs;
        let r = self.r / rhs;
        let s = self.s / rhs;

        FractionalHex { q, r, s }
    }
}

impl Neg for FractionalHex {
    type Output = FractionalHex;

    fn neg(self) -> Self::Output {
        FractionalHex::new(-self.q, -self.r, -self.s)
    }
}
