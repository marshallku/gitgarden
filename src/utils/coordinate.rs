use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha8Rng;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

const MAX_RETRY: u32 = 10;

pub struct Rectangle {
    pub x1: f64,
    pub y1: f64,
    pub x2: f64,
    pub y2: f64,
}

pub fn generate_coordinate<T: Hash>(
    key: T,
    x_range: (f64, f64),
    y_range: (f64, f64),
    dead_zones: Option<&[Rectangle]>,
) -> Option<(f64, f64)> {
    let mut hasher = DefaultHasher::new();
    key.hash(&mut hasher);
    let seed = hasher.finish();

    let mut rng = ChaCha8Rng::seed_from_u64(seed);

    for _ in 0..MAX_RETRY {
        let x = rng.gen_range(x_range.0..x_range.1);
        let y = rng.gen_range(y_range.0..y_range.1);

        if let Some(zones) = dead_zones {
            if !is_in_rectangles(x, y, zones) {
                return Some((x, y));
            }
        } else {
            return Some((x, y));
        }
    }

    log::error!("Failed to generate a coordinate.");

    None
}

pub fn is_in_rectangles(x: f64, y: f64, rectangles: &[Rectangle]) -> bool {
    rectangles
        .iter()
        .any(|zone| x >= zone.x1 && x <= zone.x2 && y >= zone.y1 && y <= zone.y2)
}
