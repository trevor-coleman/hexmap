use crate::hexlib::hex::Hex;
use crate::hexlib::offset::Offset;
use crate::hexlib::tile::Tile;
use std::cmp;
use std::collections::{HashMap, HashSet};

enum MapDefinition {
    Parallelogram { width: i32, height: i32 },
}

type HexSet = HashSet<Hex>;


pub struct HexMap(pub HashMap<Hex, Tile>);

pub fn hex_map_from_hex_set(hex_set: HexSet) -> HexMap {
    let mut hex_map: HashMap<Hex, Tile> = HashMap::new();
    for hex in hex_set {
        hex_map.insert(hex, Tile::new(hex));
    }

    HexMap(hex_map)
}

struct Map {
    pub tiles: HexSet,
}

fn make_parallelogram(width: i32, height: i32) -> HexSet {
    let mut hex_set: HexSet = HexSet::new();

    for q in 0..width {
        for r in 0..height {
            hex_set.insert(Hex::new(q, r, -q - r));
        }
    }

    hex_set
}

fn make_triangle(size: i32) -> HexSet {
    let mut hex_set: HexSet = HexSet::new();

    let q_min = 0;
    let q_max = size;

    for q in q_min..=q_max {
        let size = &size;
        let r_min = size - q;
        for r in r_min..=*size {
            hex_set.insert(Hex::new(q, r, -q - r));
        }
    }

    hex_set
}

pub fn make_hexagon(radius: i32) -> HexSet {
    let mut hex_set: HexSet = HexSet::new();

    let q_min = -radius;
    let q_max = radius;

    for q in q_min..=q_max {
        let r_min = cmp::max(-radius, -q - radius);
        let r_max = cmp::min(radius, -q + radius);
        for r in r_min..=r_max {
            hex_set.insert(Hex::new(q, r, -q - r));
        }
    }

    hex_set
}

fn make_rectangle(width: i32, height: i32) -> HexSet {
    let mut hex_set: HexSet = HexSet::new();

    for col in 0..=width {
        for row in 0..=height {
            let offset = Offset { col, row };
            let hex = offset.to_hex();
            hex_set.insert(hex);
        }
    }

    hex_set
}
