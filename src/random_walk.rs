//! Normal distribution random walk kernel.
use rand::distributions::{Distribution, Normal};
use rand::Rng;
use std::f64::consts::PI;
use super::{ConditionalDistribution, ConditionalPDF};

/// Random walk Markov transition kernel.
pub struct RandomWalk(Normal);

impl RandomWalk {
    /// Initialize the random walk kernel with the standard normal distribution.
    pub fn new() -> Self {
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
