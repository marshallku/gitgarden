use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha8Rng;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

const MAX_RETRY: u32 = 10;

#[derive(Debug, Clone)]
pub struct Rectangle {
    pub x1: f64,
    pub y1: f64,
    pub x2: f64,
    pub y2: f64,
}

pub fn must_generate_coordinate<T: Hash>(
    key: T,
    x_range: (f64, f64),
    y_range: (f64, f64),
    dead_zone: Option<&Rectangle>,
) -> (f64, f64) {
    if dead_zone.is_none() {
        return generate_coordinate(&key, x_range, y_range, dead_zone).unwrap();
    }

    match generate_coordinate(&key, x_range, y_range, dead_zone) {
        Some(coord) => coord,
        None => generate_coordinate(&key, x_range, y_range, None).unwrap(),
    }
}

pub fn generate_coordinate<T: Hash>(
    key: T,
    x_range: (f64, f64),
    y_range: (f64, f64),
    dead_zone: Option<&Rectangle>,
) -> Option<(f64, f64)> {
    let mut hasher = DefaultHasher::new();
    key.hash(&mut hasher);
    let seed = hasher.finish();

    let mut rng = ChaCha8Rng::seed_from_u64(seed);

    for _ in 0..MAX_RETRY {
        let x = rng.gen_range(x_range.0..x_range.1);
        let y = rng.gen_range(y_range.0..y_range.1);

        if let Some(zones) = dead_zone {
            if !is_in_rectangle(x, y, zones) {
                return Some((x, y));
            }
        } else {
            return Some((x, y));
        }
    }

    log::error!("Failed to generate a coordinate.");

    None
}

pub fn is_in_rectangle(x: f64, y: f64, rectangles: &Rectangle) -> bool {
    x >= rectangles.x1 && x <= rectangles.x2 && y >= rectangles.y1 && y <= rectangles.y2
}
