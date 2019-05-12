use crate::{ConditionalDistribution, ConditionalPDF, TransitionKernel};
use ndarray::prelude::*;
use ndarray::IntoDimension;
use ndarray_rand::normal::MultivariateStandardNormal;
use rand;
use rand::Rng;
use rand::distributions::Distribution;

/// Multivariate random walk.
pub struct MultivariateRandomWalk(MultivariateStandardNormal<Ix1>);

impl MultivariateRandomWalk
{
    pub fn new<Sh>(shape: Sh) -> Self
    where
        Sh: IntoDimension<Dim = Ix1>,
    {
        let n = MultivariateStandardNormal::new(shape);
        MultivariateRandomWalk(n)
    }
}

impl ConditionalDistribution<Array1<f64>> for MultivariateRandomWalk
{
    fn conditional_sample<R: Rng + ?Sized>(&self, rng: &mut R, x: Array1<f64>) -> Array1<f64> {
        // property of the normal distribution for easy sampling
        x + self.0.sample(rng)
    }
}

impl ConditionalPDF<Array1<f64>> for MultivariateRandomWalk
{
    fn conditional_pdf(&self, x: Array1<f64>, y: Array1<f64>) -> f64 {
        let half_d: f64 = 0.5*self.0.shape()[0] as f64;
        let z = x - y;
        (-0.5*z.dot(&z)).exp()/(2.*std::f64::consts::PI).powf(half_d)
    }
}

impl TransitionKernel<Array1<f64>> for MultivariateRandomWalk
{
    fn is_symmetrical(&self) -> bool { true }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::{distributions::Distribution,thread_rng};
    use crate::{MHSampler,};

    fn test() {
        let dim = 2usize;
    let q = MultivariateRandomWalk::new(dim);
    let p = |x: Array1<f64>| {
        let r: f64 = x.dot(&x).sqrt();
        1. / (1. + r*r)
    };
    let n_samples = 1000;
    let x0 = arr1(&[1.0, -1.0]);

    let ref mut rng = thread_rng();

    let mhe = MHSampler::new(&p, &q);
    let samples = mhe.sample(rng, n_samples, x0);

    }
}
