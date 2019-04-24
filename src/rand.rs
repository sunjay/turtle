//! Utilities for generating random values
//!
//! The entire [`rand`](https://github.com/rust-lang-nursery/rand) crate which allows you to
//! generate random values is re-exported for your convenience. That means that you can use any
//! of its parts by importing from this module.
//!
//! **Almost all of the documented items listed at the bottom of this page are from the `rand`
//! crate directly.** The only additional function provided specifically by `turtle` is the
//! [`random_range()`] function.
//!
//! ```rust,no_run
//! use turtle::rand::{random, random_range};
//! # fn main() {}
//! ```
//!
//! Of everything provided, the [`random()`] function is probably the most common function you will
//! use. In fact, it's exported from the turtle crate directly so you don't even have to use the
//! `rand` module in order to access it.
//!
//! Instead of this:
//!
//! ```rust,no_run
//! use turtle::Turtle;
//! use turtle::rand::random;
//! # fn main() {}
//! ```
//!
//! You can do this:
//!
//! ```rust,no_run
//! use turtle::{Turtle, random};
//! # fn main() {}
//! ```
//!
//! This means that for the most part, unless you are doing something very advanced, you won't need
//! to import this module. The [`random()`] function should be enough for
//! most cases. See the next section for more information on that.
//!
//! # Generating Random Values
//!
//! The [`random()`] function allows you to generate random values for many different types.
//! The following are some examples of types that can be used with [`random()`]:
//!
//! * [`Distance`] - `f64` values greater than or equal to `0.0` and less than `1.0`
//! * [`Angle`] - `f64` values greater than or equal to `0.0` and less than `1.0`
//! * [`Speed`] - any speed value in the valid range, not including instant
//! * [`Color`] - colors with random red, green, blue and alpha values (use
//!   [`opaque()`](../color/struct.Color.html#method.opaque) to get a solid random color)
//! * [`Point`] - a random point with two `f64` values greater than or equal to `0.0` and less than `1.0`
//! * and more!
//!
//! Using [`random()`] often requires you to specify a type that you want to generate. For example,
//! if you run the following, the compiler will tell you that it needs you to give it more
//! information about the type that you want to generate:
//!
//! ```rust,compile_fail,E0283
//! # use turtle::{Turtle, random};
//! let mut turtle = Turtle::new();
//! turtle.set_speed(random());
//! ```
//!
//! This will produce an error that looks something like the following:
//!
//! ```text
//! error[E0283]: type annotations required: cannot resolve `_: std::convert::Into<turtle::Speed>`
//!  --> src/rand.rs:5:8
//!   |
//! 5 | turtle.set_speed(random());
//!   |        ^^^^^^^^^
//! ```
//!
//! To resolve this, you can either annotate the type by generating the random value in a separate
//! variable, or use Rust's "turbofish" syntax.
//!
//! ```rust
//! # use turtle::{Turtle, Speed, random};
//! let mut turtle = Turtle::new();
//! // 1. Separate out into a variable, then annotate the desired type
//! let speed: Speed = random();
//! turtle.set_speed(speed);
//! // 2. Turbofish syntax ::<T>
//! turtle.set_speed(random::<Speed>());
//! ```
//!
//! # Generating Random Values in a Range
//!
//! The [`random_range()`] function allows you to generate values in a given range. You provide
//! a lower bound and an upper bound. The number generated will be greater than or equal to the
//! lower bound and strictly less than the upper bound.
//!
//! ```rust
//! # use turtle::random_range;
//! // Generates an f64 value between 394.0 and 499.99999...
//! let value: f64 = random_range(394.0, 500.0);
//! assert!(value >= 394.0 && value < 500.0);
//! // Generates a u64 value between 32 and 64
//! let value = random_range::<u64>(32, 65);
//! assert!(value >= 32 && value <= 64);
//! // You do not need to specify the type if the compiler has enough information:
//! fn foo(a: u64) {}
//! foo(random_range(381, 920));
//! ```
//!
//! Most types that can be used with [`random()`] can also be used with [`random_range()`]. This
//! includes all of the types listed above.
//!
//! When [`random_range()`] is used to generate a [`Point`], it creates a random point within the
//! rectangle formed by the two points given as arguments to [`random_range()`]. This is
//! illustrated in the example below:
//!
//! ```rust
//! # use turtle::{Point, random_range};
//! // Generates a Point value with:
//! //   x-coordinate between 46.0 and 99932.0
//! //   y-coordinate between 309.0 and 1803.0
//! let value: Point = random_range([99932.0, 309.0].into(), [46.0, 1803.0].into());
//! assert!(value.x >= 46.0 && value.x < 99932.0);
//! assert!(value.y >= 309.0 && value.y < 1803.0);
//! ```
//!
//! # How can one function generate so many different return types?
//!
//! Knowing how [`random()`] works is **not required** in order to be able to use it. That being said,
//! it is an excellent example of combing the concepts of "generics" and "traits". If you are not
//! familiar with those concepts yet, take a look at the
//! [Rust book](https://doc.rust-lang.org/book/second-edition/). It has an excellent
//! [section on both generics and traits](https://doc.rust-lang.org/book/second-edition/ch10-00-generics.html).
//!
//! The type signature of the [`random()`] function is similar to the following:
//!
//! ```rust,compile_fail,E0308
//! # use turtle::rand::distributions::{Distribution, Standard};
//! fn random<T>() -> T where Standard: Distribution<T> { /* ... */ }
//! ```
//!
//! This tells us the following:
//!
//! * The `random()` function takes **no arguments** and returns a value of type `T`
//! * The generic type `T` is required to fulfill the condition `Standard: Distribution<T>`
//!
//! That's pretty incredible! With *zero* arguments, [`random()`] can generate all kinds of values.
//! This works because while it doesn't have any normal arguments passed in, it does require a
//! single *type* argument to be provided. This type is required to be part of a specific *trait*
//! implementation and the [`random()`] function is able to use that implementation to generate a
//! random value. This is done at compile time so no additional work needs to be performed at
//! runtime in order to determine the type to generate or how to generate it.
//!
//! This also explains why we sometimes need to use the turbofish syntax (`::<T>`) in order to
//! specify the type `T`. When a function requires a type argument and doesn't take that argument
//! as one of its parameters, the compiler can often end up in a situation where it doesn't have
//! the necessary information to determine which type to return.
//!
//! The turbofish syntax and type annotations provide two different ways to clarify what we want.
//!
//! That being said, there are a lot of situations where the compiler *can* figure out what type
//! we need. In some of the examples above, we specified which type we would like the function to
//! return by annotating the variable that we assigned the random value to.
//!
//! ```rust
//! # use turtle::{Speed, random};
//! let speed: Speed = random();
//! ```
//!
//! If we were passing the value to a function that is known to take [`Speed`] as its type, the
//! compiler can use that information to determine the return type of [`random()`]. The following
//! example compiles without any additional annotations:
//!
//! ```rust
//! # use turtle::{Speed, random};
//! fn foo(speed: Speed) {}
//! // No type annotations required!
//! foo(random());
//! ```
//!
//! # The Orphan Rule
//!
//! Not all of the implementations of the traits from the `rand` crate for the types above are
//! implemented in this crate. There is a rule known as the [orphan rule] which prevents anyone
//! from implementing a trait on a type that they do not define. That is why we implemented the
//! `rand` traits for for [`Speed`], [`Color`], and [`Point`], but not for type aliases like
//! [`Distance`]. [`Distance`] is a type alias for `f64`. `f64` is provided by the standard
//! library, so we cannot implement any traits for it. The implementations of the `rand` crate's
//! traits for those types come from the `rand` crate itself.
//!
//! [`random()`]: ../fn.random.html
//! [`random_range()`]: fn.random_range.html
//! [`Distance`]: ../type.Distance.html
//! [`Angle`]: ../type.Angle.html
//! [`Speed`]: ../speed/struct.Speed.html
//! [`Color`]: ../color/struct.Color.html
//! [`Point`]: ../struct.Point.html
//! [orphan rule]: https://doc.rust-lang.org/book/ch10-02-traits.html#implementing-a-trait-on-a-type

pub use ::rand::*;

use self::distributions::uniform::SampleUniform;

/// Implement this type to allow it to be used with the random_range function.
pub trait RandomRange {
    fn random_range<R: Rng>(rng: &mut R, low: Self, high: Self) -> Self;
}

impl<T> RandomRange for T
where
    T: PartialOrd + SampleUniform,
{
    fn random_range<R: Rng>(rng: &mut R, low: T, high: T) -> T {
        rng.gen_range(low, high)
    }
}

/// Generates a random value in the given range.
///
/// The value `x` that is returned will be such that low &le; x &lt; high.
///
/// See [Generating Random Values in a Range](index.html#generating-random-values-in-a-range)
/// for more information.
///
/// # Panics
/// Panics if low &ge; high
///
/// # Example:
/// ```rust
/// # use turtle::random_range;
/// // Generates an f64 value between 100 and 199
/// let value: f64 = random_range(100.0, 200.0);
/// assert!(value >= 100.0 && value < 200.0);
/// // Generates a u64 value between 1000 and 3000000
/// let value = random_range::<u64>(1000, 3000001);
/// assert!(value >= 1000 && value < 3000001);
/// // You do not need to specify the type if the compiler has enough information:
/// fn foo(a: u64) {}
/// foo(random_range(432, 1938));
/// ```
pub fn random_range<T: RandomRange>(low: T, high: T) -> T {
    let mut rng = thread_rng();
    RandomRange::random_range(&mut rng, low, high)
}
