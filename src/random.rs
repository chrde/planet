extern crate noise;

use self::noise::*;


pub struct RandomGenerator {
    fbm: Perlin,
}

impl RandomGenerator {
    pub fn new(seed: usize) -> Self {
        let r = RandomGenerator { fbm: Perlin::new() };
        r.fbm.set_seed(seed);
        r
    }
    pub fn next(&self, point: &[f64]) -> f64 {
        let mut vs = [0.0; 3];
        vs.copy_from_slice(&point);
        self.fbm.get(vs)
    }
}

