//! Implementation of the Metropolis-Hastings MCMC (Monte Carlo Markov Chain) algorithm.
use rand;
use rand::Rng;

pub mod ffi;
pub mod random_walk;

/// The `ConditionalDistribution` trait provides an interface for sampling
/// elements conditionally to priors.
pub trait ConditionalDistribution {
    /// Draw a sample conditionally to the previous state `y`.
    fn conditional_sample<R: Rng + ?Sized>(&self, rng: &mut R, y: f64) -> f64;
}

/// The `ConditionalPDF` trait provides an interface for conditional
/// probability densities.
pub trait ConditionalPDF {
    /// Returns the conditional probability density function calculated at `x`.
    fn conditional_pdf(&self, x: f64, y: f64) -> f64;

    /// Calculates the log-probability density at point `x`.
    fn ln_conditional_pdf(&self, x: f64, y: f64) -> f64 {
        self.conditional_pdf(x, y).ln()
    }
}

/// Metropolis-Hastings sampler.
pub struct MHSampler<'a, F, G>
where
    F: Fn(f64) -> f64,
    G: ConditionalDistribution + ConditionalPDF,
{
    p: F,
    kernel: &'a G,
}

impl<'a, F, G> MHSampler<'a, F, G>
where
    F: Fn(f64) -> f64,
    G: ConditionalDistribution + ConditionalPDF,
{
    /// q: reference conditional density function kernel
    pub fn new(p: F, kernel: &'a G) -> Self {
        MHSampler { p, kernel }
    }

    /// Sample `n` samples of the distribution you want, starting from
    /// value `x0`.
    pub fn sample<R: Rng + ?Sized>(&self, rng: &mut R, n: usize, x0: f64) -> Vec<f64> {
        let mut res = Vec::with_capacity(n); // capacity O(n)
        res.push(x0);
        let mut candidate: f64;
        let mut acceptance: f64; // acceptance
        let mut y = x0;
        let ref kernel = self.kernel;
        let p = &self.p;

        for _t in 1..n {
            candidate = kernel.conditional_sample(rng, y);
            acceptance = p(candidate) * kernel.conditional_pdf(y, candidate)
                / (p(y) * kernel.conditional_pdf(candidate, y));
            acceptance = acceptance.min(1.);
            let u: f64 = rng.gen();
            if u <= acceptance {
                y = candidate;
            }
            res.push(y);
        }

        res
    }
}
