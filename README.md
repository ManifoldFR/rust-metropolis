# `metropolis`

An implementation of MCMC (Markov Chain Monte Carlo) algorithms in Rust

## Installation

Just clone the depo or add the Git URL to your `Cargo.toml` and you're good to go!

## C FFI

The [build script](./build.rs) generates a C++ header that can be used to integrate this library within C++ code.

## Python API

The [pylib](./pylib) directory contains Python bindings
generated with [PyO3](https://github.com/PyO3/PyO3).
