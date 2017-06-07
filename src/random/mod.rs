extern crate noise;

use self::noise::*;


pub struct RandomGenerator {
    fbm: Perlin,
    seed: usize,
}

impl RandomGenerator {
    pub fn new() -> Self {
        RandomGenerator {
            fbm: Perlin::new(),
            seed: 12,
        }
    }
    pub fn next(&self, dt: f64) -> f64 {
        self.fbm.set_seed(self.seed);
        self.fbm.get([42.0, 37.0, 2.0, dt as f64]) + 0.5
    }
}
