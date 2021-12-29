use crate::fractional::FractionalHex;
use crate::hex::Hex;

pub fn line(a: &Hex, b: &Hex) -> Vec<Hex> {
    let mut results: Vec<Hex> = Vec::new();
    let iterations = a.distance_to(*b);
    let step: f32 = 1.0 / (iterations as f32).max(1.0);

    for i in 0..=iterations {
        let i = i as f32;
        results.push(hex_lerp(a.nudge(), b.nudge(), &step * i).round());
    }

    results
}

fn lerp(a: f32, b: f32, t: &f32) -> f32 {
    let a = &a;
    a + (b - a) * t
}

fn hex_lerp(a: FractionalHex, b: FractionalHex, t: f32) -> FractionalHex {
    let q = lerp(a.q, b.q, &t);
    let r = lerp(a.r, b.r, &t);
    let s = lerp(a.s, b.s, &t);
    FractionalHex { q, r, s }
}
