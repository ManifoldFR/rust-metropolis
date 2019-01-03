# `metropolis`

An implementation of MCMC (Markov Chain Monte Carlo) algorithms in Rust

## Installation

Just clone the depo or add the Git URL to your `Cargo.toml` and you're good to go!

## FFI test

```bash
cbindgen -o bindings.h -l C
gcc test.c -I . -L target/debug -lmetropolis
```