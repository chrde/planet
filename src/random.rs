extern crate noise;

use self::noise::*;


pub struct RandomGenerator {
    fbm: Perlin,
    seed: usize,
}

impl RandomGenerator {
    pub fn new() -> Self {
        // let fbm: Fbm<f64> = Fbm::new();
        // for y in 0..1024 {
        //     for x in 0..1024{
        //         let (a, b) = (((x as f64 - (1024.0 / 2.0)) / 400.0), ((y as f64 - (1024.0 / 2.0)) / 400.0));
        //         let z = fbm.get([a, b, 0.0, 0.0]);
        //         println!("{}", z);
        //     }
        // }
        RandomGenerator { fbm: Perlin::new(), seed: 12 }
    }
    pub fn next(&self, dt: f64) -> f64 {
        self.fbm.set_seed(self.seed);
        self.fbm.get([42.0, 37.0, 2.0, dt as f64]) + 0.5
    }
}

