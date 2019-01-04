//! Wrapping for the C FFI
#![allow(non_snake_case)]
use crate::MHSampler;
use crate::random_walk::RandomWalk;

/// Generates a distribution sample and copies its data to the output buffer.
#[no_mangle]
pub extern fn sampleMHrandomWalk(n_samples: u32, x0: f64, out_buf: *mut f64) {
    let q = RandomWalk::new();
    let p = |x: f64| {
        (-x.abs()).exp()
    }; // Laplace distribution
    let mhe = MHSampler::new(p, &q);
    let ref mut rng = rand::thread_rng();
    let samples = mhe.sample(rng, n_samples as usize, x0);
    unsafe {
        ::std::slice::from_raw_parts_mut(out_buf, n_samples as usize)
            .copy_from_slice(&samples);
    }
    println!("Rust says: number of samples is {}", samples.len());
    println!("{:?}", samples);
}
