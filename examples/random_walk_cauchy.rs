use metropolis::random_walk::RandomWalk;
use metropolis::*;

fn main() {
    let q = RandomWalk::new();
    let p = |x: f64| 1. / (1. + (x + 1.5) * (x + 1.5)); // Laplace distribution
    let n_samples = 10000;
    let x0 = 0.5;
    let mhe = MHSampler::new(&p, &q);
    let ref mut rng = rand::thread_rng();
    let samples = mhe.sample(rng, n_samples, x0);
    let sam_str = serde_json::to_string_pretty(&samples).unwrap();
    use std::fs;

    fs::write("examples/output.json", sam_str).unwrap()
}
