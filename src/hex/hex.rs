#![allow(dead_code)]
#![allow(unused_variables)]

use crate::hex::direction::HexDirection;
use crate::hex::fractional::FractionalHex;
use crate::hex::layout::{Layout, Orientation, Point};
use crate::hex::offset::Offset;
use bevy::prelude::Vec3;
use bevy::render::render_graph::NodeId;
use bevy_inspector_egui::Inspectable;
use petgraph::prelude::NodeIndex;
use rustc_hash::FxHashMap;
use std::cmp;
use std::collections::hash_map::DefaultHasher;
use std::f32::consts::PI;
use std::fmt::{Debug, Formatter};
use std::hash::{Hash, Hasher};
use std::ops::{Add, Div, Mul, Neg, Sub};

#[derive(Eq, PartialEq, Copy, Clone, Hash, Inspectable)]
pub struct Hex {
    #[inspectable(min=-10, max=10)]
    pub q: i32,
    #[inspectable(min=-10, max=10)]
    pub r: i32,
    #[inspectable(min=-10, max=10)]
    pub s: i32,
}

impl Default for Hex {
    fn default() -> Self {
        Hex::new(0, 0, 0)
    }
}

impl std::convert::From<Hex> for NodeIndex<u32> {
    fn from(hex: Hex) -> Self {
        NodeIndex::<u32>::new(u32::from(hex) as usize)
    }
}

impl std::convert::From<Hex> for u32 {
    fn from(hex: Hex) -> Self {
        let q = hex.q;
        let r = hex.r;
        let s = hex.s;

        println!(
            "From Hex: q:{:?}, r:{:?}, s:{:?} --> {:?}",
            q,
            r,
            s,
            (q * 31 + r * (31 ^ 2) + s * (31 ^ 3)) as u32
        );
        (q * 31 + r * (31 ^ 2) + s * (31 ^ 3)) as u32
    }
}

impl Hex {
    pub(crate) fn neighbors(&self) -> Vec<Hex> {
        let mut neighbors: Vec<Hex> = Vec::from([Hex::default(); 6]);

        for i in 0..5 {
            neighbors[i] = self.neighbor(HexDirection::new(i as i32));
        }

        neighbors
    }
}

impl Hex {
    pub const fn new(q: i32, r: i32, s: i32) -> Hex {
        Hex { q, r, s }
    }

    pub fn screen_pos(&self, layout: &Layout, z: f32) -> Vec3 {
        let m: &Orientation = &layout.orientation;
        let f: FractionalHex = self.to_fractional_hex();

        let x = (m.f0 * f.q + m.f1 * f.r) * layout.size.x;
        let y = (m.f2 * f.q + m.f3 * f.r) * layout.size.y;

        Vec3::new(x + layout.origin.x, y + layout.origin.y, z)
    }

    pub fn length(self) -> i32 {
        ((self.q.abs() + self.r.abs() + self.s.abs()) / 2) as i32
    }

    pub fn to_fractional_hex(&self) -> FractionalHex {
        let q: f32 = self.q as f32;
        let r: f32 = self.r as f32;
        let s: f32 = self.s as f32;

        FractionalHex { q, r, s }
    }

    pub fn distance_to(self, other: Hex) -> i32 {
        let distance: Hex = self - other;
        distance.length()
    }

    pub fn neighbor(self, direction: HexDirection) -> Hex {
        let direction = direction.to_hex();
        self + direction
    }

    pub fn rotate_left(self) -> Hex {
        Hex {
            q: -self.s,
            r: -self.q,
            s: -self.r,
        }
    }
    pub fn rotate_right(self) -> Hex {
        Hex {
            q: -self.r,
            r: -self.s,
            s: -self.q,
        }
    }

    pub fn nudge(self) -> FractionalHex {
        let s = self;
        FractionalHex {
            q: s.q as f32 + 1e-6,
            r: s.r as f32 + 1e-6,
            s: s.s as f32 - 2e-6,
        }
    }

    pub fn in_range(&self, n: &i32) -> Vec<Hex> {
        let mut result: Vec<Hex> = Vec::new();
        let q_min = -n;
        for q in q_min..=*n {
            let n = n;
            let q = q;

            let minus_q_minus_n = -q - n;

            let r_min = cmp::min(-n, minus_q_minus_n);
            let r_max = cmp::max(-n, minus_q_minus_n);

            for r in r_min..=r_max {
                let s = -q - r;
                result.push(Hex::new(q, r, s));
            }
        }
        result
    }

    pub fn to_offset(self) -> Offset {
        let q = &self.q;
        let r = &self.r;

        Offset {
            col: self.q,
            row: r + (q + (q % 2)) / 2,
        }
    }

    pub fn corner_offset(&self, layout: &Layout, corner: i32) -> Point {
        let size = layout.size;
        let angle = 2.0 * PI * (layout.orientation.start_angle + (corner as f32)) / 6.0;
        Point::new(size.x * angle.cos(), size.y * angle.sin())
    }

    pub fn corners(&self, layout: &Layout) -> [Point; 6] {
        let mut corners = [Point::new(0.0, 0.0); 6];
        let center = self.screen_pos(layout, 0.0);

        for i in 0..6 {
            let offset: Point = self.corner_offset(layout, i);
            corners[i as usize] = Point::new(center.x + offset.x, center.y + offset.y);
        }
        corners
    }
}

// impl PartialEq for Hex {
//     fn eq(&self, rhs: &Self) -> bool {
//         self.q == rhs.q && self.r == rhs.r && self.s == rhs.s
//     }
// }

impl Debug for Hex {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Hex")
            .field("q", &self.q)
            .field("r", &self.r)
            .field("s", &self.s)
            .finish()
    }
}

impl Add for Hex {
    type Output = Hex;

    fn add(self, rhs: Self) -> Self::Output {
        Hex {
            q: self.q + rhs.q,
            r: self.r + rhs.r,
            s: self.s + rhs.s,
        }
    }
}

impl Add<FractionalHex> for Hex {
    type Output = FractionalHex;

    fn add(self, rhs: FractionalHex) -> Self::Output {
        FractionalHex {
            q: (self.q as f32) + rhs.q,
            r: (self.r as f32) + rhs.r,
            s: (self.s as f32) + rhs.s,
        }
    }
}

impl Sub for Hex {
    type Output = Hex;

    fn sub(self, rhs: Self) -> Self::Output {
        Hex {
            q: self.q - rhs.q,
            r: self.r - rhs.r,
            s: self.s - rhs.s,
        }
    }
}

impl Sub<FractionalHex> for Hex {
    type Output = FractionalHex;

    fn sub(self, rhs: FractionalHex) -> Self::Output {
        FractionalHex {
            q: (self.q as f32) - rhs.q,
            r: (self.r as f32) - rhs.r,
            s: (self.s as f32) - rhs.s,
        }
    }
}

impl Mul<i32> for Hex {
    type Output = Hex;

    fn mul(self, rhs: i32) -> Self::Output {
        let rhs = &rhs;
        let q = self.q * rhs;
        let r = self.r * rhs;
        let s = self.s * rhs;

        Hex { q, r, s }
    }
}

impl Div<i32> for Hex {
    type Output = Hex;

    fn div(self, rhs: i32) -> Self::Output {
        let rhs = &rhs;
        Hex {
            q: self.q / rhs,
            r: self.r / rhs,
            s: self.s / rhs,
        }
    }
}

impl Neg for Hex {
    type Output = Hex;

    fn neg(self) -> Self::Output {
        Hex::new(-self.q, -self.r, -self.s)
    }
}

pub fn hex_from_screen(layout: Layout, p: Vec3) -> FractionalHex {
    let m: &Orientation = &layout.orientation;
    let pt: Point = Point::new(
        (p.x - layout.origin.x) / layout.size.x,
        (p.y - layout.origin.y) / layout.size.y,
    );

    let q = m.b0 * pt.x + m.b1 * pt.y;
    let r = m.b2 * pt.x + m.b3 * pt.y;
    FractionalHex { q, r, s: -q - r }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_sets_values_correctly() {
        let x = Hex::new(1, 2, -3);
        assert_eq!(x.q, 1);
    }

    #[test]
    fn it_calculates_length_correctly() {
        let x = Hex::new(1, -1, 0);

        assert_eq!(x.length(), 1);

        let y = Hex::new(1, -2, -1);

        assert_eq!(y.length(), 2);

        let z = Hex::new(0, 3, -3);

        assert_eq!(z.length(), 3);
    }

    #[test]
    fn it_multiplies_correctly() {
        for i in 1..10 {
            let x = Hex::new(1, 2, -3);

            let z = x * i;

            let i = &i;
            assert_eq!(z.q, x.q * i);
            assert_eq!(z.r, x.r * i);
            assert_eq!(z.s, x.s * i);
        }
    }

    #[test]
    fn it_divides_correctly() {
        for i in 1..10 {
            let i_ref = &i;

            let x = Hex::new(120, 240, -360);

            let z = x / i;

            assert_eq!(z.q, x.q / i_ref);
            assert_eq!(z.r, x.r / i_ref);
            assert_eq!(z.s, x.s / i_ref);
        }
    }
}
