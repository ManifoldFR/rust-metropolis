//! Wrapping for the C FFI
#![allow(non_snake_case)]
use crate::random_walk::RandomWalk;
use crate::ConditionalDistribution;
use crate::MHSampler;

/// Generates a distribution sample and copies its data to the output buffer.
#[no_mangle]
pub extern "C" fn sampleMHrandomWalk(n_samples: u32, x0: f64, out_buf: *mut f64) {
    let q = RandomWalk::new();
    let p = |x: f64| (-x.abs()).exp(); // Laplace distribution
    let mhe = MHSampler::new(p, &q);
    let ref mut rng = rand::thread_rng();
    let samples = mhe.sample(rng, n_samples as usize, x0);
    unsafe {
        ::std::slice::from_raw_parts_mut(out_buf, n_samples as usize).copy_from_slice(&samples);
    }
}

/// Generates a random walk Markov chain.
#[no_mangle]
pub extern "C" fn randomWalk(n_samples: u32, x0: f64, sigma: f64, out_buf: *mut f64) {
    assert!(sigma > 0.);
    use statrs::distribution::Normal;
    let n = Normal::new(0., sigma).unwrap();
    let q = RandomWalk::from_normal(n);
    let ref mut rng = rand::thread_rng();
    let mut y: f64;
    let mut res = vec![x0];
    for i in 1..n_samples as usize {
        y = res[i - 1];
        res.push(q.conditional_sample(rng, y));
    }
    unsafe {
        ::std::slice::from_raw_parts_mut(out_buf, n_samples as usize).copy_from_slice(&res);
    }
}
