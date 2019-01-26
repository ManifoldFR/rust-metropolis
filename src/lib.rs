#![feature(specialization)]
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

/// Markov chain transition kernel.
pub trait TransitionKernel<T>: ConditionalDistribution<T> + ConditionalPDF<T> {
    fn is_symmetrical(&self) -> bool;
}

default impl<T, K> TransitionKernel<T> for K
where K: ConditionalDistribution<T> + ConditionalPDF<T> {
    default fn is_symmetrical(&self) -> bool {
        false
    }
}

/// Metropolis-Hastings sampler.
pub struct MHSampler<'a, T, G>
where
    T: Copy,
    G: TransitionKernel<T>
{
    /// Target probability distribution.
    target: &'a Fn(T) -> f64,
    /// Conditional probability distribution kernel for the Markov chain.
    kernel: &'a G,
}

impl<'a, T, G> MHSampler<'a, T, G>
where
    T: Copy,
    G: TransitionKernel<T>
{
    /// Returns a sampler targeting the given distribution using the given transition kernel.
    ///
    /// * `p` - Target probability distribution
    /// * `kernel` - Markov Chain transition kernel
    pub fn new(p: &'a impl Fn(T) -> f64, kernel: &'a G) -> Self {
        MHSampler { target: p, kernel }
    }

    /// Compute the acceptance ratio for the proposal state y
    /// with previous state x.
    fn acceptance(&self, x: T, y: T) -> f64 {
        let p = self.target;
        let mut r = p(y) / p(x);
        if !(self.kernel.is_symmetrical()) {
            let ref kernel = self.kernel;
            r *= kernel.conditional_pdf(x, y) / kernel.conditional_pdf(y, x);
        }
        r.min(1.0)
    }

    /// Sample `n` samples of the distribution you want, starting from
    /// value `x0`.
    pub fn sample<R: Rng + ?Sized>(&self, rng: &mut R, n: usize, x0: T) -> Vec<T> {
        let mut res: Vec<T> = Vec::with_capacity(n); // capacity O(n)
        res.push(x0);
        let mut candidate: T;
        let mut acceptance: f64; // acceptance
        let mut x = x0.clone();
        let ref kernel = self.kernel;

        for _t in 1..n {
            candidate = kernel.conditional_sample(rng, x);
            acceptance = self.acceptance(x, candidate);
            let u: f64 = rng.gen();
            if u <= acceptance {
                x = candidate;
            }
            res.push(x);
        }

        res
    }
}
