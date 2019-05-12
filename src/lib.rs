#![feature(specialization)]
//! Implementation of MCMC (Monte Carlo Markov Chain) sampling algorithms.
use rand;
use rand::Rng;

pub mod ffi;
pub mod random_walk;
pub mod defs;

use defs::*;


/// Metropolis-Hastings sampler.
/// Utilises a `TransitionKernel` object and targets a specified probability
/// distribution.
/// `T`: data type passed around
pub struct MHSampler<'a, T, G>
where
    T: Clone,
    G: TransitionKernel<T, T>,
{
    /// Target probability distribution.
    target: &'a Fn(T) -> f64,
    /// Conditional transition probability kernel for the Markov chain.
    kernel: &'a G
}


impl<'a, T, G> MHSampler<'a, T, G>
where
    T: Clone,
    G: TransitionKernel<T, T>,
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
    fn acceptance(&self, x: T, y: T) -> f64
    {
        let p = self.target;
        let mut r = p(y.clone()) / p(x.clone());
        if !(self.kernel.is_symmetrical()) {
            let ref kernel = self.kernel;
            r *= kernel.conditional_pdf(x.clone(), y.clone())
                / kernel.conditional_pdf(y.clone(), x.clone());
        }
        r.min(1.0)
    }

    /// Sample `n` samples of the distribution you want, starting from
    /// value `x0`.
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R, n: usize, x0: T) -> Vec<T>
    {
        let mut res: Vec<T> = Vec::with_capacity(n); // capacity O(n)
        let mut candidate: T;
        let mut acceptance: f64; // acceptance
        let mut x = x0.clone();
        res.push(x.clone());
        let ref kernel = self.kernel;

        for _t in 1..n {
            candidate = kernel.conditional_sample(rng, x.clone());
            acceptance = self.acceptance(x.clone(), candidate.clone());
            let u: f64 = rng.gen();
            if u <= acceptance {
                x = candidate.clone();
            }
            res.push(x.clone());
        }

        res
    }
}
