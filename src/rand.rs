//! Utilities for generating random values
//!
//! This module provides a much, much simpler version of the very robust [`rand`] crate. The
//! purpose of this is to give people teaching/learning Rust an easier target for doing interesting
//! things with random numbers. You don't need to know very much Rust to use this module and once
//! you get comfortable with this, you can always move to learning the [`rand`] crate as well. If
//! the items in this module are not advanced enough for your needs, you probably want to consider
//! using the full [`rand`] crate instead.
//!
//! # Random Number Generation
//!
//! Computers can't generate "true" random numbers yet, however they can approximate sequences of
//! numbers that *look* random. This is called [pseudo-random number generation]. This module
//! provides a set of functions (and traits) for generating pseudo-random numbers.
//!
//! The current set of utilities provided includes:
//!
//! * [`random()`] - for generating a single random value of a given type
//! * [`random_range()`] - for generating a single random value of a given type in a certain range
//! * [`shuffle()`] - for mixing up a slice of values (`Vec`, slices, etc.)
//! * [`choose()`] - for choosing a single value from a slice of values (`Vec`, slices, etc.)
//!
//! See the documentation for each of those functions for more on what you can use them for.
//!
//! # Generating Random Values
//!
//! The [`random()`] function supports all of the common primitive types you would expect:
//!
//! * Booleans: `bool`
//! * Floating-point: `f32`, `f64`
//! * Integers: `u8`, `u16`, `u32`, `u64`, `u128`, `usize`, `i8`, `i16`, `i32`, `i64`, `i128`, `isize`
//! * Tuples, arrays, and [many more][`Random`]
//!
//! It also supports types specific to the `turtle` crate:
//!
//! * [`Distance`] - `f64` values greater than or equal to `0.0` and less than or equal to `1.0`
//! * [`Angle`] - `f64` values greater than or equal to `0.0` and less than or equal to `1.0`
//! * [`Speed`] - any speed value in the valid range, not including instant
//! * [`Color`] - colors with random red, green, blue, and alpha values (use
//!   [`opaque()`] to get a solid random color)
//! * [`Point`] - a random point with two `f64` values greater than or equal to `0.0` and less than
//!   or equal to `1.0`
//!
//! # Random Custom Types
//!
//! To make types within your application capable of being used with [`random()`] or
//! [`random_range()`], implement the [`Random`] or [`RandomRange`] traits respectively. See the
//! documentation of those traits for more information.
//!
//! You typically won't need to implement [`RandomSlice`] for yourself since it is already
//! implemented for slices. That being said, if your type can be represented as a slice, you can
//! implement [`RandomSlice`] so it can be used with the [`shuffle()`] and [`choose()`] functions.
//!
//! ```rust
//! use turtle::rand::RandomSlice;
//!
//! // This is a "newtype" wrapper around a Vec<T> which can be represented as a slice.
//! #[derive(Debug, Clone)]
//! struct MyVec<T>(Vec<T>);
//!
//! impl<T> RandomSlice for MyVec<T> {
//!     type Item = T;
//!
//!     fn shuffle(&mut self) {
//!         (&mut *self.0 as &mut [T]).shuffle();
//!     }
//!
//!     fn choose(&self) -> Option<&Self::Item> {
//!         (&*self.0 as &[T]).choose()
//!     }
//! }
//! ```
//!
//! [`rand`]: https://docs.rs/rand
//! [pseudo-random number generation]: https://en.wikipedia.org/wiki/Pseudorandom_number_generator
//! [`random()`]: fn.random.html
//! [`random_range()`]: fn.random_range.html
//! [`shuffle()`]: fn.shuffle.html
//! [`choose()`]: fn.choose.html
//! [`Random`]: trait.Random.html
//! [`RandomRange`]: trait.RandomRange.html
//! [`RandomSlice`]: trait.RandomSlice.html
//! [`Distance`]: ../type.Distance.html
//! [`Angle`]: ../type.Angle.html
//! [`Speed`]: ../speed/struct.Speed.html
//! [`Color`]: ../color/struct.Color.html
//! [`Point`]: ../struct.Point.html
//! [`opaque()`]: ../color/struct.Color.html#method.opaque

use std::num::Wrapping;

/// This trait represents any type that can have random values generated for it.
///
/// **Tip:** There is a list later on this page that shows many of the types that implement this
/// trait.
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
/// use turtle::rand::{
///     random,
///     Random,
///     RandomRange,
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
    /// use turtle::{Color, Speed, rand::random};
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
/// **Tip:** There is a list later on this page that shows many of the types that implement this
/// trait.
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
///     use turtle::rand::{random, random_range};
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

/// Generates a single random value of the type `T`.
///
/// # Specifying the type to generate
///
/// Since `T` can be any of a number of different types, you may need to provide a "type parameter"
/// explicitly so that Rust knows what to generate. You can do this either by annotating the type
/// of the variable you are assigning to or by using "turbofish" syntax to specify the type on
/// the `random()` function itself.
///
/// ```rust,no_run
/// use turtle::{Turtle, Speed, rand::random};
/// let mut turtle = Turtle::new();
///
/// // 1. Separate out into a variable, then annotate the desired type
/// let speed: Speed = random();
/// turtle.set_speed(speed);
///
/// // 2. Turbofish syntax ::<T>
/// turtle.set_speed(random::<Speed>());
/// ```
///
/// # Example
///
/// Setting the pen color to a randomly generated color:
///
/// ```rust,no_run
/// use turtle::{Turtle, Color, rand::random};
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
/// Most types that can be used with [`random()`] can also be used with this function. For a list
/// of types that can be passed into this function, see the documentation for the [`RandomRange`]
/// trait.
///
/// [`random()`]: fn.random.html
/// [`RandomRange`]: trait.RandomRange.html
///
/// # Panics
/// Panics if low > high
///
/// # Example:
///
/// ```rust
/// use turtle::rand::random_range;
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
    ($n:expr, $t:ident, $($ts:ident,)*) => {
        impl_random_slice!(($n - 1), $($ts,)*);

        impl<T> RandomSlice for [T; $n] {
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
    ($n:expr,) => {
        impl<T> RandomSlice for [T; $n] {
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
/// use turtle::{rand::shuffle, colors::{RED, BLUE, GREEN, YELLOW}};
///
/// let mut pen_colors = [RED, BLUE, GREEN, YELLOW];
/// // A different order of colors every time!
/// shuffle(&mut pen_colors);
///
/// // Even works with Vec
/// let mut pen_colors = vec![RED, BLUE, GREEN, YELLOW];
/// shuffle(&mut pen_colors);
/// ```
pub fn shuffle<S: RandomSlice + ?Sized>(slice: &mut S) {
    slice.shuffle();
}

/// Chooses a random element from the slice and returns a reference to it.
///
/// If the slice is empty, returns None.
///
/// # Example
///
/// ```rust,no_run
/// use turtle::{Turtle, rand::choose, colors::{RED, BLUE, GREEN, YELLOW}};
///
/// let mut turtle = Turtle::new();
///
/// let pen_colors = [RED, BLUE, GREEN, YELLOW];
/// // Choose a random pen color
/// let chosen_color = choose(&pen_colors).copied().unwrap();
/// turtle.set_pen_color(chosen_color);
///
/// // Even works with Vec
/// let pen_colors = vec![RED, BLUE, GREEN, YELLOW];
/// let chosen_color = choose(&pen_colors).copied().unwrap();
/// turtle.set_pen_color(chosen_color);
/// ```
pub fn choose<S: RandomSlice + ?Sized>(slice: &S) -> Option<&<S as RandomSlice>::Item> {
    slice.choose()
}
