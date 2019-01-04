# `metropolis`

An implementation of MCMC (Markov Chain Monte Carlo) algorithms in Rust

## Installation

Just clone the depo or add the Git URL to your `Cargo.toml` and you're good to go!

## FFI test

The [build script](./build.rs) already generates a C++ bindings header.

```bash
g++ cffi/test.cpp -I . -L target/debug -lmetropolis -o test --std=c++11
```
