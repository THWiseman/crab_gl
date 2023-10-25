use std::collections::hash_map::RandomState;
use std::hash::{BuildHasher, Hasher};

pub fn random() -> usize {
    RandomState::new().build_hasher().finish() as usize
}

pub fn random_float(min: f32, max: f32) -> f32 {
    let mut hasher = RandomState::new().build_hasher();
    hasher.write_i32(0);
    let hash = hasher.finish() as f32;
    min + (max - min) * hash
}