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
//! a lower bound and an upper bound. The value generated will be greater than or equal to the
//! lower bound and strictly less than the upper bound.
//!
//! ```rust
//! # use turtle::random_range;
//! // Generates an f64 value between 394.0 and 499.99999...
//! let value: f64 = random_range(394.0, 500.0);
//! assert!(value >= 394.0 && value <= 500.0);
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
//! assert!(value.x >= 46.0 && value.x <= 99932.0);
//! assert!(value.y >= 309.0 && value.y <= 1803.0);
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

use std::num::Wrapping;

/// This trait represents any type that can have random values generated for it.
///
/// There is a list later on this page that shows many of the types that implement this trait.
///
/// # Example
///
/// To implement this trait for your own types, call [`random()`] or [`random_range()`] for each field.
/// For enums, generate a random number and pick a variant of your enum based on that. You can then
/// use [`random()`] or [`random_range()`] to pick values for that variant's fields (if necessary).
///
/// [`random()`]: fn.random.html
/// [`random_range()`]: fn.random_range.html
///
/// ```rust,no_run
/// use turtle::{
///     random,
///     random_range,
///     rand::{Random, RandomRange},
/// };
///
/// #[derive(Debug, Clone)]
/// struct Product {
///     price: f64,
///     quantity: u32,
/// }
///
/// impl Random for Product {
///     fn random() -> Self {
///         Self {
///             // Prices sure are fluctuating!
///             price: Random::random(),
///             // This will generate a value between 1 and 15 (inclusive)
///             quantity: RandomRange::random_range(0, 15),
///         }
///     }
/// }
///
/// #[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// enum Door {
///     Open,
///     Closed,
///     Locked,
/// }
///
/// impl Random for Door {
///     fn random() -> Self {
///         use Door::*;
///         // Pick a variant of `Door` based on a number.
///         // Notice the type in the numeric literal so Rust knows what to generate
///         match RandomRange::random_range(0u8, 3) {
///             0 => Open,
///             1 => Closed,
///             2 => Locked,
///             // Even though we know that the value will be between 0 and 2, the compiler does
///             // not, so we instruct it to panic if this case ever occurs.
///             _ => unreachable!(),
///         }
///     }
/// }
///
/// fn main() {
///     // These types can now be used with the `random()` function!
///     let product: Product = random();
///     let door: Door = random();
///
///     // ...
/// }
/// ```
pub trait Random {
    /// Generate a single random value.
    ///
    /// # Example
    ///
    /// ```rust
    /// use turtle::{Color, Speed, random};
    ///
    /// // Need to annotate the type so Rust knows what you want it to generate
    /// let x: f32 = random();
    /// let y: f32 = random();
    ///
    /// // Works with turtle specific types too!
    /// let color: Color = random();
    /// let speed: Speed = random();
    /// ```
    fn random() -> Self;
}

/// This trait represents any type that can have random values generated for it within a certain
/// range.
///
/// There is a list later on this page that shows many of the types that implement this trait.
///
/// It will usually only make sense to implement this trait for types that represent a single
/// value (e.g. [`Speed`], [`Color`], `f64`, `u32`, etc.). For example, if you had the type:
///
/// ```rust,no_run
/// #[derive(Debug, Clone)]
/// struct Product {
///     price: f64,
///     quantity: u32,
/// }
/// ```
///
/// What would `random_range(1.5, 5.2)` mean for this type? It's hard to say because while you
/// could generate a random value for `price`, it doesn't make sense to generate a `quantity`
/// given that range.
///
/// A notable exception to this is the `Point` type. You can interpret a call to
/// `random_range(Point {x: 1.0, y: 2.0}, Point {x: 5.0, y: 8.0})` as wanting to
/// generate a random `Point` in the rectangle formed by the two points provided
/// as arguments.
///
/// [`Speed`]: ../speed/struct.Speed.html
/// [`Color`]: ../color/struct.Color.html
///
/// # Example
///
/// This example demonstrates how to implement this trait for a type with a limited range of valid
/// values.
///
/// ```rust,no_run
/// use turtle::rand::{Random, RandomRange};
///
/// // Some constants to represent the valid range of difficulty levels
/// const MIN_DIFFICULTY: u32 = 1;
/// const MAX_DIFFICULTY: u32 = 10;
///
/// /// Represents the difficulty level of the game
/// #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
/// struct Difficulty(u32);
///
/// impl Random for Difficulty {
///     /// Generates a random difficulty within the valid range of difficulty levels
///     fn random() -> Self {
///         Difficulty(RandomRange::random_range(MIN_DIFFICULTY, MAX_DIFFICULTY))
///     }
/// }
///
/// // Choosing `u32` for the `Bound` type because that is more convenient to type than if
/// // we had chosen `Difficulty`.
/// //
/// // Example: `random_range(1, 2)` vs. `random_range(Difficulty(1), Difficulty(2))`
/// impl RandomRange<u32> for Difficulty {
///     /// Generates a random difficulty level within the given range.
///     ///
///     /// # Panics
///     ///
///     /// Panics if either bound could result in a value outside the valid range
///     /// of difficulties or if `low > high`.
///     fn random_range(low: u32, high: u32) -> Self {
///         if low < MIN_DIFFICULTY || high > MAX_DIFFICULTY {
///             panic!("The boundaries must be within the valid range of difficulties");
///         }
///
///         RandomRange::random_range(low, high)
///     }
/// }
///
/// fn main() {
///     use turtle::{random, random_range};
///
///     // We can now generate random values for Difficulty!
///     let difficulty: Difficulty = random();
///     let difficulty: Difficulty = random_range(5, 10);
/// }
/// ```
pub trait RandomRange<Bound = Self> {
    /// Generate a single random value in the given range. The value `x` that is returned will be
    /// such that low &le; x &le; high.
    ///
    /// The `Bound` type is used to represent the boundaries of the range of values to generate.
    /// For most types this will just be `Self`.
    ///
    /// # Panics
    ///
    /// Panics if `low > high`.
    fn random_range(low: Bound, high: Bound) -> Self;
}

macro_rules! impl_random {
    ($($typ:ty),*) => (
        $(
            impl Random for $typ {
                fn random() -> Self {
                    use rand::Rng;
                    rand::thread_rng().gen()
                }
            }

            impl RandomRange for $typ {
                fn random_range(low: Self, high: Self) -> Self {
                    use rand::{Rng, distributions::Uniform};
                    let uniform = Uniform::new_inclusive(low, high);
                    rand::thread_rng().sample(&uniform)
                }
            }
        )*
    );
}

impl_random!(f32, f64, u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize);

// `char` does not support sampling in a range, so no RandomRange impl
impl Random for char {
    fn random() -> Self {
        use rand::Rng;
        rand::thread_rng().gen()
    }
}

// `bool` does not support sampling in a range, so no RandomRange impl
impl Random for bool {
    fn random() -> Self {
        use rand::Rng;
        rand::thread_rng().gen()
    }
}

macro_rules! impl_random_tuple {
    // Use variables to indicate the arity of the tuple
    ($($tyvar:ident),* ) => {
        // Trailing comma for the 1 tuple
        impl< $( $tyvar: Random ),* > Random for ( $( $tyvar ),* , ) {
            #[inline]
            fn random() -> Self {
                (
                    // use the $tyvar's to get the appropriate number of repeats (they're not
                    // actually needed because type inference can figure it out)
                    $(
                        random::<$tyvar>()
                    ),*
                    // Trailing comma for the 1 tuple
                    ,
                )
            }
        }
    }
}

impl Random for () {
    fn random() -> Self {
        ()
    }
}

impl_random_tuple!(A);
impl_random_tuple!(A, B);
impl_random_tuple!(A, B, C);
impl_random_tuple!(A, B, C, D);
impl_random_tuple!(A, B, C, D, E);
impl_random_tuple!(A, B, C, D, E, F);
impl_random_tuple!(A, B, C, D, E, F, G);
impl_random_tuple!(A, B, C, D, E, F, G, H);
impl_random_tuple!(A, B, C, D, E, F, G, H, I);
impl_random_tuple!(A, B, C, D, E, F, G, H, I, J);
impl_random_tuple!(A, B, C, D, E, F, G, H, I, J, K);
impl_random_tuple!(A, B, C, D, E, F, G, H, I, J, K, L);

macro_rules! impl_random_array {
    // Recursive, given at least one type parameter
    {$n:expr, $t:ident, $($ts:ident,)*} => {
        impl_random_array!(($n - 1), $($ts,)*);

        impl<T: Random> Random for [T; $n] {
            #[inline]
            fn random() -> Self {
                [random::<$t>(), $(random::<$ts>()),*]
            }
        }
    };
    // Empty case (no type parameters left)
    {$n:expr,} => {
        impl<T: Random> Random for [T; $n] {
            fn random() -> Self {
                []
            }
        }
    };
}

impl_random_array!(32, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T,);

impl<T: Random> Random for Option<T> {
    fn random() -> Self {
        if Random::random() {
            Some(Random::random())
        } else {
            None
        }
    }
}

impl<T: Random> Random for Wrapping<T> {
    fn random() -> Self {
        Wrapping(Random::random())
    }
}

/// Generates a single random value of the type T.
///
/// # Example
///
/// This example sets the pen color to a randomly generated color:
///
/// ```rust,no_run
/// use turtle::{Turtle, Color, random};
///
/// let mut turtle = Turtle::new();
/// let color: Color = random();
/// // Calling `opaque()` because even the `alpha` value of the color is randomized.
/// turtle.set_pen_color(color.opaque());
/// ```
pub fn random<T: Random>() -> T {
    <T as Random>::random()
}

/// Generates a random value in the given range.
///
/// The value `x` that is returned will be such that low &le; x &le; high.
///
/// See [Generating Random Values in a Range](index.html#generating-random-values-in-a-range)
/// for more information.
///
/// # Panics
/// Panics if low > high
///
/// # Example:
/// ```rust
/// use turtle::random_range;
///
/// // Generates an f64 value between 100 and 199.99999...
/// let value: f64 = random_range(100.0, 200.0);
/// assert!(value >= 100.0 && value <= 200.0);
///
/// // Generates a u64 value between 1000 and 3000000
/// let value = random_range::<u64, _>(1000, 3000001);
/// assert!(value >= 1000 && value <= 3000001);
///
/// // You do not need to specify the type if the compiler has enough information:
/// fn foo(a: u64) {}
/// foo(random_range(432, 1938));
///
/// // This will always return the same value because `low == `high`
/// assert_eq!(random_range::<i32, _>(10i32, 10), 10);
/// ```
pub fn random_range<T: RandomRange<B>, B>(low: B, high: B) -> T {
    RandomRange::random_range(low, high)
}

/// This trait represents useful random operations for slices.
///
/// You will not typically use this trait directly or even import it.
/// The [`shuffle()`] and [`choose()`] functions provide all the functionality of
/// this trait without needing to have it in scope.
///
/// [`shuffle()`]: fn.shuffle.html
/// [`choose()`]: fn.choose.html
pub trait RandomSlice {
    /// The type of item stored in this slice
    type Item;

    /// Shuffle the slice's elements in place. None of the elements will be modified during
    /// this process, only moved.
    fn shuffle(&mut self);

    /// Chooses a random element from this slice and returns a reference to it.
    ///
    /// If the slice is empty, returns None.
    fn choose(&self) -> Option<&Self::Item>;
}

impl<T> RandomSlice for [T] {
    type Item = T;

    fn shuffle(&mut self) {
        use rand::seq::SliceRandom;
        let mut rng = rand::thread_rng();
        <Self as SliceRandom>::shuffle(self, &mut rng);
    }

    fn choose(&self) -> Option<&Self::Item> {
        use rand::seq::SliceRandom;
        let mut rng = rand::thread_rng();
        <Self as SliceRandom>::choose(self, &mut rng)
    }
}

impl<T> RandomSlice for Vec<T> {
    type Item = T;

    fn shuffle(&mut self) {
        (&mut *self as &mut [T]).shuffle();
    }

    fn choose(&self) -> Option<&Self::Item> {
        (&*self as &[T]).choose()
    }
}

macro_rules! impl_random_slice {
    // Recursive, given at least one type parameter
    {$n:expr, $t:ident, $($ts:ident,)*} => {
        impl_random_slice!(($n - 1), $($ts,)*);

        impl<T: Random> RandomSlice for [T; $n] {
            type Item = T;

            fn shuffle(&mut self) {
                (&mut *self as &mut [T]).shuffle();
            }

            fn choose(&self) -> Option<&Self::Item> {
                (&*self as &[T]).choose()
            }
        }
    };
    // Empty case (no type parameters left)
    {$n:expr,} => {
        impl<T: Random> RandomSlice for [T; $n] {
            type Item = T;

            fn shuffle(&mut self) {
                (&mut *self as &mut [T]).shuffle();
            }

            fn choose(&self) -> Option<&Self::Item> {
                (&*self as &[T]).choose()
            }
        }
    };
}

impl_random_slice!(32, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T,);

/// Shuffle the elements of the given slice in place.
///
/// None of the elements are modified during this process, only moved.
///
/// # Example
///
/// ```rust,no_run
/// use turtle::{color, shuffle};
///
/// let mut pen_colors = [color::RED, color::BLUE, color::GREEN, color::YELLOW];
/// // A different order of colors every time!
/// shuffle(&mut pen_colors);
///
/// // Even works with Vec
/// let mut pen_colors = vec![color::RED, color::BLUE, color::GREEN, color::YELLOW];
/// shuffle(&mut pen_colors);
/// ```
pub fn shuffle<S: RandomSlice>(slice: &mut S) {
    slice.shuffle();
}

/// Chooses a random element from the slice and returns a reference to it.
///
/// If the slice is empty, returns None.
///
/// # Example
///
/// ```rust,no_run
/// use turtle::{Turtle, color, choose};
///
/// let mut turtle = Turtle::new();
///
/// let mut pen_colors = [color::RED, color::BLUE, color::GREEN, color::YELLOW];
/// // Choose a random pen color
/// let chosen_color = choose(&mut pen_colors).cloned().unwrap();
/// turtle.set_pen_color(chosen_color);
///
/// // Even works with Vec
/// let mut pen_colors = vec![color::RED, color::BLUE, color::GREEN, color::YELLOW];
/// let chosen_color = choose(&mut pen_colors).cloned().unwrap();
/// turtle.set_pen_color(chosen_color);
/// ```
pub fn choose<S: RandomSlice>(slice: &S) -> Option<&<S as RandomSlice>::Item> {
    slice.choose()
}
