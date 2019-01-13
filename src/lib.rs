//! Implementation of the Metropolis-Hastings MCMC (Monte Carlo Markov Chain) algorithm.
use rand;
use rand::Rng;

pub mod ffi;
pub mod random_walk;

/// The `ConditionalDistribution` trait provides an interface for sampling
/// elements conditionally to priors.
pub trait ConditionalDistribution<T> {
    /// Draw a sample conditionally to the previous state `y`.
    fn conditional_sample<R: Rng + ?Sized>(&self, rng: &mut R, y: T) -> T;
}

/// The `ConditionalPDF` trait provides an interface for conditional
/// probability densities.
pub trait ConditionalPDF<T> {
    /// Returns the conditional probability density function calculated at `x`.
    fn conditional_pdf(&self, x: T, y: T) -> f64;

    /// Calculates the log-probability density at point `x`.
    fn ln_conditional_pdf(&self, x: T, y: T) -> f64 {
        self.conditional_pdf(x, y).ln()
    }
}

/// Metropolis-Hastings sampler.
pub struct MHSampler<'a, F, G, T>
where
    F: Fn(T) -> f64,
    G: ConditionalDistribution<T> + ConditionalPDF<T>,
    T: Clone
{
    p: F,
    kernel: &'a G,
}

impl<'a, F, G, T> MHSampler<'a, F, G, T>
where
    F: Fn(T) -> f64,
    G: ConditionalDistribution<T> + ConditionalPDF<T>,
    T: Clone
{
    /// p: target probability distribution
    /// q: reference conditional density function kernel
    pub fn new(p: F, kernel: &'a G) -> Self {
        MHSampler { p, kernel }
    }

    /// Sample `n` samples of the distribution you want, starting from
    /// value `x0`.
    pub fn sample<R: Rng + ?Sized>(&self, rng: &mut R, n: usize, x0: T) -> Vec<T> {
        let mut res: Vec<T> = Vec::with_capacity(n); // capacity O(n)
        res.push(x0);
        let mut candidate: T;
        let mut acceptance: f64; // acceptance
        let mut y = x0.clone();
        let ref kernel = *self.kernel;
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
