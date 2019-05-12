//! Random walk kernel.
use super::{ConditionalDistribution, ConditionalPDF, TransitionKernel};
use statrs::distribution::{Continuous, Normal};

use rand::distributions::Distribution;
use rand::Rng;

#[cfg(feature = "array")]
/// Multivariate random walk kernel.
pub mod multidimensional;

/// Random walk Markov transition kernel on the real line.
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

impl ConditionalDistribution<f64, f64> for RandomWalk {
    fn conditional_sample<R: Rng + ?Sized>(&self, rng: &mut R, x: f64) -> f64 {
        // property of the normal distribution for easy sampling
        x + self.0.sample(rng)
    }
}

impl TransitionKernel<f64, f64> for RandomWalk {
    fn is_symmetrical(&self) -> bool {
        true
    }
}
