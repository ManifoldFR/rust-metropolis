use metropolis::random_walk::multidimensional::MultivariateRandomWalk;
use metropolis::MHSampler;
use ndarray::prelude::*;
use rand::thread_rng;

fn main() {
    let dim = 2usize;
    let q = MultivariateRandomWalk::new(dim);
    let p = |x: ArrayView1<f64>| {
        let r: f64 = x.dot(&x).sqrt();
        1. / (1. + r*r)
    };
    let n_samples = 1000;
    let x0 = arr1(&[1.0, -1.0]);

    let ref mut rng = thread_rng();

    let mhe = MHSampler::new(&p, &q);
    let samples = mhe.sample(rng, n_samples, x0.view());
    let sam_str = serde_json::to_string_pretty(&samples).unwrap();
    use std::fs;

    fs::write("examples/output.json", sam_str).unwrap()
}
