use crate::{ConditionalDistribution, ConditionalPDF, TransitionKernel};
use ndarray::prelude::*;
use ndarray::IntoDimension;
use ndarray_rand::normal::MultivariateStandardNormal;
use rand;
use rand::Rng;
use rand::distributions::Distribution;

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

impl ConditionalDistribution<Array1<f64>> for MultivariateRandomWalk {
    fn conditional_sample<R: Rng + ?Sized>(&self, rng: &mut R, x: Array1<f64>) -> Array1<f64> {
        // property of the normal distribution for easy sampling
        x + self.0.sample(rng)
    }
}

impl ConditionalPDF<Array1<f64>> for MultivariateRandomWalk {
    fn conditional_pdf(&self, x: Array1<f64>, y: Array1<f64>) -> f64 {
        let half_d: f64 = 0.5*self.0.shape()[0] as f64;
        let z = x - y;
        (-0.5*z.dot(&z)).exp()/(2.*std::f64::consts::PI).powf(half_d)
    }
}

impl TransitionKernel<Array1<f64>> for MultivariateRandomWalk {
    fn is_symmetrical(&self) -> bool { true }
}
