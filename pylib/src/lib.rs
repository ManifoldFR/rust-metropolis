#![feature(custom_attribute)]

use pyo3::prelude::*;
use metropolis;
use metropolis::random_walk::RandomWalk;
use rand;

#[pymodule]
fn random_walk(py: Python, m: &PyModule) -> PyResult<()> {

    #[pyfn(m, "sample_random_walk")]
    fn sample_rw(n: i64, x0: f64) -> PyResult<Vec<f64>> {
        let q = RandomWalk::new();
        let cauchy_x0 = 1.5;
        let p = |x: f64| 1. / (1. + (x - cauchy_x0).powi(2));

        let mhs = metropolis::MHSampler::new(&p, &q);
        let ref mut rng = rand::thread_rng();
        Ok(mhs.sample(rng, n as usize, x0))
    }

    Ok(())
}
