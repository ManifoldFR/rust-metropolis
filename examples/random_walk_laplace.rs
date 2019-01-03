use metropolis::*;
use metropolis::random_walk::RandomWalk;


fn main() {
    let q = RandomWalk::new();
    let p = |x: f64| {
        (-x.abs()).exp()
    }; // Laplace distribution
    let n_samples = 10000;
    let x0= 0.5;
    let mhe = MHSampler::new(p, q);
    let ref mut rng = rand::thread_rng();
    let samples = mhe.sample(rng, n_samples, x0);
    let sam_str = serde_json::to_string_pretty(&samples).unwrap();
    use std::fs;

    fs::write("examples/output.json", sam_str).unwrap()
}
