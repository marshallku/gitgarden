use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha8Rng;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

pub fn generate_coordinate<T: Hash>(
    key: T,
    x_range: (f64, f64),
    y_range: (f64, f64),
) -> (f64, f64) {
    let mut hasher = DefaultHasher::new();
    key.hash(&mut hasher);
    let seed = hasher.finish();

    let mut rng = ChaCha8Rng::seed_from_u64(seed);
    let x = rng.gen_range(x_range.0..x_range.1);
    let y = rng.gen_range(y_range.0..y_range.1);

    (x, y)
}
