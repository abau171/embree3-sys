[![Build Status](https://app.travis-ci.com/abau171/embree3-sys.svg?branch=main)](https://app.travis-ci.com/abau171/embree3-sys)

# `embree3-sys`
Rust bindings for Intel's [Embree](https://www.embree.org/) high-performance ray tracing library.

Bindings are automatically generated at build time using [`bindgen`](https://github.com/rust-lang/rust-bindgen). Unlike the [`embree`](https://crates.io/crates/embree) and [`embree-rs`](https://crates.io/crates/embree-rs/0.3.6) crates, `embree3-sys`'s generated bindings will pick up your current Embree installation's configuration (so, for example, `RTC_MAX_INSTANCE_LEVEL_COUNT` will be set correctly if you have enabled multi-level instancing).

Currently, an existing Embree installation is required.
See [Embree's GitHub page](https://github.com/embree/embree) for installation instructions.
