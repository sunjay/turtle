//! Utilities for generating random values
//!
//! The entire [`rand`](https://github.com/rust-lang-nursery/rand) crate which allows you to
//! generate random values is re-exported for your convenience. That means that you can use any
//! of its parts by importing from this module.
//! See the documentation for [`rand_crate`](../../rand/index.html).
//!
//! ```rust,no_run
//! extern crate turtle;
//! use turtle::rand::Rand;
//! # fn main() {}
//! ```
//!
//! The [`random()`](../fn.random.html) function is the most common function you will use. In fact,
//! it's exported from the turtle crate directly so you don't even have to use the `rand` module
//! in order to access it.
//!
//! Instead of this:
//!
//! ```rust,no_run
//! extern crate turtle;
//! use turtle::Turtle;
//! use turtle::rand::random;
//! # fn main() {}
//! ```
//!
//! You can do this:
//!
//! ```rust,no_run
//! extern crate turtle;
//! use turtle::{Turtle, random};
//! # fn main() {}
//! ```
//!
//! This means that for the most part, unless you are doing something very advanced, you won't need
//! to import this module. The [`turtle::random()`](../fn.random.html) function should be enough for
//! most cases.

pub use rand_crate::*;
