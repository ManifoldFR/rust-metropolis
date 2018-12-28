//! Implementation of the Metropolis-Hastings MCMC (Monte Carlo Markov Chain) algorithm.
use std::fmt;
use std::f64::consts::PI;
use rand;
use rand::distributions::Distribution;
use rand::Rng;

pub struct MHSampler<F, G>
    where F: Fn(f64) -> f64, G: ConditionalDistribution+ConditionalPDF {
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



impl<F, G> MHSampler<F, G>
    where F: Fn(f64) -> f64, G: ConditionalDistribution+ConditionalPDF {
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
        let mut alpha: f64; // acceptance
        let mut y = x0;
        let ref kernel = self.kernel;
        let p = &self.p;

        for t in 1..n {
            candidate = kernel.csample(rng, y);
            alpha = p(candidate)/p(y);
            alpha *= kernel.cpdf(y, candidate)/kernel.cpdf(candidate, y);
            alpha = alpha.min(1.);
            let u: f64 = rng.gen();
            if u <= alpha {
                y = candidate;
            }
            res.push(y);
        }

        res
    }
}

#[cfg(test)]
mod tests {
    use super::MHSampler;
    use crate::ConditionalDistribution;
    use rand;
    use std::f64::consts::PI;
    use rand::distributions::{Distribution, Normal};
    use rand::Rng;
    use crate::ConditionalPDF;

    struct MyKernel; // normal distribution

    impl ConditionalPDF for MyKernel {
        fn cpdf(&self, x: f64, y: f64) -> f64 {
            let norm_factor = (2.*PI).sqrt();
            (-0.5 * (x - y)).exp()/norm_factor
        }
    }

    impl ConditionalDistribution for MyKernel {
        fn csample<R:Rng+?Sized>(&self, rng: &mut R, y: f64) -> f64 {
            let mut n = Normal::new(y, 1.0);
            n.sample(rng)
        }
    }

    #[test]
    fn it_works() {
        let q = MyKernel {};
        let p = |x: f64| {
            (-x.abs()).exp()
        }; // uniform probability
        println!("{}", p(0.5));

        let mhe = MHSampler::new(p, q);
        let ref mut rng = rand::thread_rng();
        let samples = mhe.sample(rng, 10, 0.5);
        println!("{:?}", samples);

    }

}
