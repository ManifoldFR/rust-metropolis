//! Normal distribution random walk kernel, on the real line.
use super::{ConditionalDistribution, ConditionalPDF};
use statrs::distribution::{Continuous, Normal};

use rand::distributions::Distribution;
use rand::Rng;

/// Random walk Markov transition kernel.
pub struct RandomWalk(Normal);

impl RandomWalk {
    /// Initialize the random walk kernel with the standard normal distribution.
    pub fn new() -> Self {
        RandomWalk(Normal::new(0., 1.0).unwrap())
    }

    pub fn from_normal(n: Normal) -> Self {
        RandomWalk(n)
    }
}

impl ConditionalPDF<f64> for RandomWalk {
    fn conditional_pdf(&self, x: f64, y: f64) -> f64 {
        self.0.pdf(x - y)
    }
}

impl ConditionalDistribution<f64> for RandomWalk {
    fn conditional_sample<R: Rng + ?Sized>(&self, rng: &mut R, y: f64) -> f64 {
        // property of the normal distribution for easy sampling
        y + self.0.sample(rng)
    }
}
