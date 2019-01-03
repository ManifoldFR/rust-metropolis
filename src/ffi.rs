//! Wrapping for the C FFI
use crate::MHSampler;
use crate::random_walk::RandomWalk;

#[no_mangle]
pub extern fn sample_mh_randomwalk(n_samples: u32) -> *mut f64 {
    let q = RandomWalk::new();
    let p = |x: f64| {
        (-x.abs()).exp()
    }; // Laplace distribution
    let x0= 0.5;
    let mhe = MHSampler::new(p, q);
    let ref mut rng = rand::thread_rng();
    let mut samples = mhe.sample(rng, n_samples as usize, x0);
    println!("Rust says: number of samples is {}", samples.len());
    println!("{:?}", samples);
    samples.as_mut_ptr()
}
