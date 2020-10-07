# DCI-Closed

DCI-Closed, a frequent closed itemset mining algorithm, implemented in Rust.

[![Cargo](https://img.shields.io/crates/v/dci.svg)](https://crates.io/crates/dci)
[![Documentation](https://docs.rs/dci/badge.svg)](https://docs.rs/dci)

## Features

- Two flavors: sequential and parallel.
- Dataset generic: you can use your own dataset type.
- Optional [bitmatrix](https://docs.rs/bitmatrix) dataset support.
- No unsafe code.

## Background

The implementation is based on the [original paper](http://hpc.isti.cnr.it/~claudio/papers/2004_FIMI_dci_closed.pdf),
by Lucchese, C. et al. The parallel flavor is a trivial [Rayon](https://docs.rs/rayon/)
spin of the original algorithm.

## Licence

`dci` is licenced under the [MIT Licence](http://opensource.org/licenses/MIT).
