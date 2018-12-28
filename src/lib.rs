//! Implementation of the Metropolis-Hastings MCMC (Monte Carlo Markov Chain) algorithm.
use std::fmt;
use rand;
use rand::distributions::Distribution;
use rand::Rng;

pub struct MHEstimator<F, G> where F: Distribution<f64>, G: ConditionalDistribution {
    p: F,
    q: G
}

pub trait ConditionalDistribution {
    /// Draw a sample conditionally to the previous state `y`.
    fn csample<R:Rng+?Sized>(self, rng: &mut R, y: f64) -> f64;
}

impl<F, G> MHEstimator<F, G> where F: Distribution<f64>, G: ConditionalDistribution {
    /// q: reference conditional density function kernel
    pub fn new(p: F, q: G) -> MHEstimator<F, G> {
        MHEstimator {
            p,
            q
        }
    }

    /// Sample `n` samples of the distribution you want, starting from
    /// value `x0`.
    pub fn sample<R: Rng+?Sized>(self, rng: &mut R, n: usize, x0: f64) -> Vec<f64> {
        let mut res = Vec::with_capacity(n);
        res
    }
}



#[cfg(test)]
mod tests {
    use super::MHEstimator;
    use crate::ConditionalDistribution;
    use rand;
    use rand::distributions::Normal;
    use rand::Rng;

    struct MyKernel; // normal distribution

    impl ConditionalDistribution for MyKernel {
        fn csample<R:Rng+?Sized>(self, rng: &mut R, y: f64) -> f64 {
            let res: f64 = Normal::new(y, 1.0).sample(rng);
            res
        }
    }

    #[test]
    fn it_works() {
        let n = Normal::new(0., 1.).unwrap();
        let q = MyKernel {};
        let p = |x: f64| {
            if 0. <= x && x < 1. {
                1.0
            } else {
                0.
            }
        }; // uniform probability
        let mhe = MHEstimator::new(p, q);
        let ref mut rng = rand::thread_rng();
        let samples = mhe.sample(rng, 10, 0.5);

        println!("{:?}", samples);

    }

}
