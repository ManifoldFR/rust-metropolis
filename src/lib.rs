//! Implementation of the Metropolis-Hastings MCMC (Monte Carlo Markov Chain) algorithm.
use rand;
use rand::Rng;

pub mod random_walk;

pub struct MHSampler<F, G>
where
    F: Fn(f64) -> f64,
    G: ConditionalDistribution+ConditionalPDF
{
    p: F,
    kernel: G
}

pub trait ConditionalDistribution {
    /// Draw a sample conditionally to the previous state `y`.
    fn csample<R:Rng+?Sized>(&self, rng: &mut R, y: f64) -> f64;
}

pub trait ConditionalPDF {
    fn cpdf(&self, x: f64, y: f64) -> f64;
}



impl<F, G> MHSampler<F, G> where
    F: Fn(f64) -> f64,
    G: ConditionalDistribution+ConditionalPDF
{
    /// q: reference conditional density function kernel
    pub fn new(p: F, kernel: G) -> MHSampler<F, G> {
        MHSampler {
            p,
            kernel
        }
    }

    /// Sample `n` samples of the distribution you want, starting from
    /// value `x0`.
    pub fn sample<R: Rng+?Sized>(&self, rng: &mut R, n: usize, x0: f64) -> Vec<f64> {
        let mut res = Vec::with_capacity(n);
        res.push(x0);
        let mut candidate: f64;
        let mut acceptance: f64; // acceptance
        let mut y = x0;
        let ref kernel = self.kernel;
        let p = &self.p;

        for t in 1..n {
            candidate = kernel.csample(rng, y);
            acceptance = p(candidate)/p(y);
            acceptance *= kernel.cpdf(y, candidate)/kernel.cpdf(candidate, y);
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
