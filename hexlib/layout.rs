#[derive(PartialOrd, PartialEq, Debug, Copy, Clone)]
pub struct Orientation {
    pub f0: f32,
    pub f1: f32,
    pub f2: f32,
    pub f3: f32,
    pub b0: f32,
    pub b1: f32,
    pub b2: f32,
    pub b3: f32,
    pub start_angle: f32,
}

impl Orientation {
    const fn new(
        f0: f32,
        f1: f32,
        f2: f32,
        f3: f32,
        b0: f32,
        b1: f32,
        b2: f32,
        b3: f32,
        start_angle: f32,
    ) -> Orientation {
        Orientation {
            f0,
            f1,
            f2,
            f3,
            b0,
            b1,
            b2,
            b3,
            start_angle,
        }
    }
}

pub fn pointy() -> Orientation {
    Orientation::new(
        (3.0 as f32).sqrt(),
        (3.0 as f32).sqrt() / 2.0,
        0.0,
        3.0 / 2.0,
        (3.0 as f32).sqrt() / 3.0,
        -1.0 / 3.0,
        0.0,
        2.0 / 3.0,
        0.5,
    )
}

pub fn flat() -> Orientation {
    Orientation::new(
        3.0 / 2.0,
        0.0,
        (3.0 as f32).sqrt() / 2.0,
        (3.0 as f32).sqrt(),
        2.0 / 3.0,
        0.0,
        -1.0 / 3.0,
        (3.0 as f32).sqrt() / 3.0,
        0.0,
    )
}

#[derive(Debug, Copy, Clone)]
pub struct Layout {
    pub orientation: Orientation,
    pub size: Point,
    pub origin: Point,
}

impl Layout {
    pub const fn new(orientation: Orientation, size: Point, origin: Point) -> Layout {
        Layout {
            orientation,
            size,
            origin,
        }
    }
}

#[derive(PartialOrd, PartialEq, Debug, Copy, Clone)]
pub struct Point {
    pub x: f32,
    pub y: f32,
}

impl Point {
    pub const fn new(x: f32, y: f32) -> Point {
        Point { x, y }
    }

    pub const fn zero() -> Point {
        Point::new(0.0, 0.0)
    }
}
