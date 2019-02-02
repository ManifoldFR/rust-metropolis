#![feature(custom_attribute)]

use pyo3::prelude::*;
use metropolis;
use metropolis::random_walk::RandomWalk;
use rand;


#[pyclass]
/// Metropolis-Hastings sampler.
struct MHSampler {
    target: PyObject,
    kernel: PyObject
}

#[pymethod]
impl MHSampler {

    #[new]
    fn __new__(obj: &PyRawObject,
               target: PyObject, kernel: PyObject) -> PyResult<()> {
        obj.init(|| {
            MHSampler {
                target, kernel
            }
        })
    }
}

#[pymodule]
fn metropolis(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<MHSampler>();
    
    Ok(())
}


#[pymodule]
fn random_walk(_py: Python, m: &PyModule) -> PyResult<()> {

    #[pyfn(m, "sample_random_walk")]
    /// sample_rw(n, init_state, p)
    /// --
    /// Draw samples from a target distribution using the Metropolis-Hastings algorithm.
    ///
    /// Args:
    ///     n: number of samples to draw
    ///     init_state: initial state of the Markov chain
    ///     p: target distribution
    fn sample_rw(py: Python, n: i64, init_state: f64, p: PyObject) -> PyResult<Vec<f64>> {
        let q = RandomWalk::new();
        let p = |x: f64| {
            let obj: PyObject = p.call1(py, (x,)).unwrap();
            obj.extract::<f64>(py).unwrap()
        };

        let mhs = metropolis::MHSampler::new(&p, &q);
        let ref mut rng = rand::thread_rng();
        Ok(mhs.sample(rng, n as usize, init_state))
    }

    Ok(())
}
