//! Normal distribution random walk kernel.
use rand::distributions::{Distribution, Normal};
use rand::Rng;
use std::f64::consts::PI;
use super::{ConditionalDistribution, ConditionalPDF};

/// Random walk Markov transition kernel.
pub struct RandomWalk(Normal);

impl RandomWalk {
    /// Initialize the random walk kernel with the standard normal distribution.
    pub fn new() -> RandomWalk {
        RandomWalk(Normal::new(0., 1.0))
    }
}

impl ConditionalPDF for RandomWalk {
    fn cpdf(&self, x: f64, y: f64) -> f64 {
        (-0.5 * (x - y).powi(2)).exp()/(2.*PI).sqrt()
    }
}

impl ConditionalDistribution for RandomWalk {
    fn csample<R:Rng+?Sized>(&self, rng: &mut R, y: f64) -> f64 {
        // property of the normal distribution provides easy method to sample
        y + self.0.sample(rng)
    }
}

#[no_mangle]
pub extern fn sample_mh_randomwalk(n_samples: u32) -> *const Vec<f64> {
    let q = RandomWalk::new();
    let p = |x: f64| {
        (-x.abs()).exp()
    }; // Laplace distribution
    let x0= 0.5;
    let mhe = super::MHSampler::new(p, q);
    let ref mut rng = rand::thread_rng();
    let samples = mhe.sample(rng, n_samples as usize, x0);
    &samples
}
