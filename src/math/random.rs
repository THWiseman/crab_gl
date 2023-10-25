use rand::{Rng, thread_rng};
use rand::distributions::Uniform;

pub fn random_float(min: f32, max: f32) -> f32 {
    let mut rng = thread_rng();
    let dist = Uniform::new(min, max);
    return rng.sample(dist);
}