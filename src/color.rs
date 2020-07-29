pub mod color_consts;

use std::fmt::Debug;
use std::iter::repeat;
use std::f64::EPSILON;

use serde::{Serialize, Deserialize};

use crate::rand::{Random, RandomRange};

/// The maximum allowed value for RGB
const RGB_MAX_VAL: f64 = 255.0;

/// The minimum allowed value for RGB and HSL values
const COLOR_MIN_VALUE: f64 = 0.0;

/// The maximum allowed value for saturation, alpha, or lightness
const SAL_MAX_VAL: f64 = 1.0;

/// The maximum allowed value for hue
const HUE_MAX_VAL: f64 = 360.0;

macro_rules! assert_value_in_range {
    ($name:expr, $value:expr, $min:expr, $max:expr) => {
        assert!(
            $value >= $min && $value <= $max,
            "{} is not a valid value for {}, values must be between {:.1} and {:.1}.",
            $value,
            $name,
            $min,
            $max
        );
    };
}

macro_rules! assert_color_valid {
    ($color:expr) => {
        assert!(
            $color.is_valid(),
            "{:?} is not a valid Color. Please see color module documentation.",
            $color
        );
    };
}

/// Compare f64 values for equality. Floating point numbers are not precise enough to be compared
/// with `==` reliably. This helper function ensures that the two numbers are within EPSILON
/// of each other.
fn f64_eq(left: f64, right: f64) -> bool {
    (left - right).abs() < EPSILON
}

/// A type for representing a color.
///
/// # Color types and constants
///
/// When setting a color, you can use a variety of different color names.
/// This module contains many of the most common colors that you might want to use. There is an
/// even more comprehensive list in the [`extra_colors`](extra_colors/index.html) module. Any of the color names
/// listed in this module or in the `extra_colors` module can be used as a color. You only need to
/// reference the `extra_colors` module if you want to use a specific color constant from that
/// module.
///
/// You can refer to a color by using its color name as a string literal. For example:
///
/// ```rust
/// # use turtle::Color;
/// # let mut turtle = turtle::Turtle::new();
/// // This will set the turtle's pen color to BLACK
/// turtle.set_pen_color("black");
/// // This is the same as the previous line
/// turtle.set_pen_color(Color::BLACK);
/// // You can use any of the supported color names (including the ones from `extra_colors`)
/// turtle.set_pen_color("deep lilac");
/// ```
///
/// You can also use hexadecimal color strings to get any color you want
/// (even ones that aren't listed here).
///
/// ```rust
/// # let mut turtle = turtle::Turtle::new();
/// turtle.set_pen_color("#3366ff");
/// turtle.set_pen_color("#36f");
/// ```
///
//TODO: Hiding this for now because the constants don't actually contain all
// the available colors
///// For your convenience, there are two static variables [`COLORS`](static.COLORS.html) and
///// [`COLOR_NAMES`](static.COLOR_NAMES.html) which contain the values of all the color constants
///// and each of their names as strings. These static variables only contain the colors from this
///// module. The [`extra_colors`](extra_colors/index.html) module has its own static `COLOR` and
///// `COLOR_NAMES` variables.
///
/// # Random Colors
///
/// You can also generate random colors. Here's an example:
///
/// ```rust
/// # let mut turtle = turtle::Turtle::new();
/// use turtle::{rand::random, Color};
/// turtle.set_pen_color(random::<Color>().opaque());
/// ```
///
/// The syntax used in `random::<Color>()` is referred to as
/// ["turbofish" syntax](https://doc.rust-lang.org/book/first-edition/generics.html#resolving-ambiguities).
/// See that documentation for more information.
///
/// Notice that you need to explicitly call the [`opaque()`](struct.Color.html#method.opaque)
/// method on the color in order to make sure that the color has an alpha value of 1.0. By default,
/// when you generate a random color, it's alpha value will be random as well.
///
/// See the [examples directory on GitHub](https://github.com/sunjay/turtle/tree/master/examples)
/// for more information.
///
/// # Creating a Color from Values
///
/// Usually, you won't need to initialize a color this way since the above methods are quite
/// convenient. However, in some situations it may be necessary to create a color with specific
/// red, green, and blue values. The following example illustrates how to do that.
///
/// ```rust
/// use turtle::Color;
/// let my_color = Color {red: 255.0, green: 55.0, blue: 11.0, alpha: 1.0};
/// ```
///
/// Note that when creating a color this way, we **do not** check if the values of each property are
/// within their valid ranges.
///
///
/// Another ergonomic syntax can also be used when passing a color to a method that supports any
/// type that implements `Into<Color>`.
///
/// ```rust
/// # use turtle::*;
/// let mut drawing = Drawing::new();
/// let mut turtle = drawing.add_turtle();
///
/// // A solid color with alpha = 1.0
/// // Syntax is [red, green, blue] and doesn't require explicitly writing the field names
/// turtle.set_pen_color([133.0, 23.0, 96.0]);
/// turtle.set_fill_color([133.0, 23.0, 96.0]);
/// drawing.set_background_color([133.0, 23.0, 96.0]);
/// // This is a little easier to type than the equivalent:
/// drawing.set_background_color(Color {red: 133.0, green: 23.0, blue: 96.0, alpha: 1.0});
///
/// // Add an additional element to the array to specify the alpha
/// // Syntax is [red, green, blue, alpha]
/// turtle.set_pen_color([133.0, 23.0, 96.0, 0.5]);
/// turtle.set_fill_color([133.0, 23.0, 96.0, 0.5]);
/// drawing.set_background_color([133.0, 23.0, 96.0, 0.5]);
/// // This is a little easier to type than the equivalent:
/// drawing.set_background_color(Color {red: 133.0, green: 23.0, blue: 96.0, alpha: 0.5});
/// ```
///
/// When creating a color this way, we **will** check whether or not the color is valid and provide
/// an error message to let you know what happened.
///
/// ```rust,should_panic
/// # let mut turtle = turtle::Turtle::new();
/// // Color values must only go up to 255.0
/// turtle.set_pen_color([133.0, 256.0, 96.0]); // This will panic with an error message
/// ```
/// There are also constructor methods available for `Color` that allow you to create a new
/// color using provided values. These are:
///
/// * [`rgb(red, green, blue)`]: Create from the given red, green, and blue values with an alpha value of 1.0
/// * [`rgba(red, green, blue, alpha)`]: Similar to `rgb` but also accepts an alpha value
/// * [`hsl(hue, saturation, lightness)`]: Create from the given hue, saturation, and lightness values with an alpha of 1.0
/// * [`hsla(hue, saturation, lightness, alpha)`]: Similar to `hsl` but also accepts an alpha value
///
/// These methods provide a concise syntax for creating a new `Color`. If the values passed in are invalid,
/// the program will exit with an error that lets you know what happened. See the documentation for each
/// method (linked above) to see which values are correct for each parameter.
///
/// ```rust
/// use turtle::Color;
///
/// // These are equivalent
/// let white_manual = Color { red: 255.0, green: 255.0, blue: 255.0, alpha: 1.0 };
/// let white_rgb = Color::rgb(255.0, 255.0, 255.0);
/// let white_rgba = Color::rgba(255.0, 255.0, 255.0, 1.0);
/// let white_hsl = Color::hsl(0.0, 0.0, 1.0);
/// let white_hsla = Color::hsla(0.0, 0.0, 1.0, 1.0);
///
/// assert_eq!(white_manual, white_rgb);
/// assert_eq!(white_rgb, white_rgba);
/// assert_eq!(white_rgba, white_hsl);
/// assert_eq!(white_hsl, white_hsla);
/// ```
///
/// So, you can incorporate these constructors into your turtle code along with
/// other methods of color creation if you like:
///
/// ```rust
/// # use turtle::*;
/// let mut drawing = Drawing::new();
/// let mut turtle = drawing.add_turtle();
///
/// // Set the pen color to blue
/// turtle.set_pen_color(Color::rgb(0.0, 130.0, 200.0));
///
/// // And the same color can be set for the fill color via the array syntax.
/// turtle.set_fill_color([0.0, 130.0, 200.0]);
///
/// // Then, we can set the background to black
/// drawing.set_background_color("black");
/// ```
/// [`rgb(red, green, blue)`]: ./struct.Color.html#method.rgb
/// [`rgba(red, green, blue, alpha)`]: ./struct.Color.html#method.rgba
/// [`hsl(hue, saturation, lightness)`]: ./struct.Color.html#method.hsl
/// [`hsla(hue, saturation, lightness, alpha)`]: ./struct.Color.html#method.hsla
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Color {
    /// Value between 0.0 and 255.0
    pub red: f64,
    /// Value between 0.0 and 255.0
    pub green: f64,
    /// Value between 0.0 and 255.0
    pub blue: f64,
    /// Value between 0.0 and 1.0
    pub alpha: f64,
}

impl Color {
    /// Create a new `Color` from the given [`RGB`] values and alpha set to 1.0.
    /// This provides a more concise way to create `Color` values, instead
    /// of using the manual `Color {...}` style.
    ///
    /// The given values must adhere to those laid out in the documentation
    /// for [`Color`]. Thus:
    ///
    /// * 0.0 &le; `red` &le; 255.0
    /// * 0.0 &le; `green` &le; 255.0
    /// * 0.0 &le; `blue` &le; 255.0
    ///
    /// ```rust
    /// use turtle::Color;
    /// let expected = Color { red: 35.0, green: 200.0, blue: 180.0, alpha: 1.0 };
    /// let actual = Color::rgb(35.0, 200.0, 180.0);
    /// assert_eq!(expected, actual);
    /// ```
    ///
    /// Values that are outside the accepted RGB range will result in a panic
    ///
    /// ```should_panic
    /// use turtle::Color;
    /// // This will not work as 256.0 is greater than the maximum allowed value for blue
    /// let color = Color::rgb(255.0, 255.0, 256.0);
    /// ```
    /// [`Color`]: ./index.html
    /// [`RGB`]: https://developer.mozilla.org/en-US/docs/Glossary/RGB
    pub fn rgb(red: f64, green: f64, blue: f64) -> Self {
        Color::rgba(red, green, blue, 1.0)
    }

    /// Create a new `Color` from the given [`RGB`] values and the provided alpha setting.
    /// This provides a more concise way to create `Color` values instead of using
    /// the manual `Color {...}` style.
    /// The given values must adhere to those laid out in the documentation for [`Color`].
    /// Thus:
    ///
    /// * 0.0 &le; `red` &le; 255.0
    /// * 0.0 &le; `green` &le; 255.0
    /// * 0.0 &le; `blue` &le; 255.0
    /// * 0.0 &le; `alpha` &le; 1.0
    ///
    /// ```rust
    /// use turtle::Color;
    /// let expected = Color { red: 35.0, green: 200.0, blue: 180.0, alpha: 0.5 };
    /// let actual = Color::rgba(35.0, 200.0, 180.0, 0.5);
    /// assert_eq!(expected, actual);
    /// ```
    ///
    /// Values that are outside the accepted RGB or alpha range will result in a panic
    ///
    /// ```should_panic
    /// use turtle::Color;
    /// // This will not work as 1.1 is greater than the maximum allowed value for alpha
    /// let color = Color::rgba(255.0, 255.0, 255.0, 1.1);
    /// ```
    /// [`Color`]: ./index.html
    /// [`RGB`]: https://developer.mozilla.org/en-US/docs/Glossary/RGB
    pub fn rgba(red: f64, green: f64, blue: f64, alpha: f64) -> Self {
        assert_value_in_range!("red", red, COLOR_MIN_VALUE, RGB_MAX_VAL);
        assert_value_in_range!("green", green, COLOR_MIN_VALUE, RGB_MAX_VAL);
        assert_value_in_range!("blue", blue, COLOR_MIN_VALUE, RGB_MAX_VAL);
        assert_value_in_range!("alpha", alpha, COLOR_MIN_VALUE, SAL_MAX_VAL);
        Color { red, green, blue, alpha }
    }

    /// Create a new `Color` from the given [`HSL`] values with alpha set to 1.0.
    ///
    /// The expected value ranges are:
    ///
    /// * 0.0 &le; `hue` &le; 360.0
    /// * 0.0 &le; `saturation` &le; 1.0
    /// * 0.0 &le; `lightness` &le; 1.0
    ///
    /// ```rust
    /// use turtle::Color;
    /// let black: Color = "black".into();
    /// let black_hsl = Color::hsl(0.0, 0.0, 0.0);
    /// assert_eq!(black, black_hsl);
    ///
    /// let white: Color = "white".into();
    /// let white_hsl = Color::hsl(0.0, 1.0, 1.0);
    /// assert_eq!(white, white_hsl);
    ///
    /// let blue: Color = "blue".into();
    /// let blue_hsl = Color::hsl(201.0, 1.0, 0.392);
    /// assert_eq!(blue, blue_hsl);
    /// ```
    /// [`HSL`]: https://en.wikipedia.org/wiki/HSL_and_HSV
    pub fn hsl(hue: f64, saturation: f64, lightness: f64) -> Self {
        Color::hsla(hue, saturation, lightness, 1.0)
    }

    /// Create a new `Color` from the given [`HSL`] values and the given alpha value.
    ///
    /// The expected value ranges are:
    ///
    /// * 0.0 &le; `hue` &le; 360.0
    /// * 0.0 &le; `saturation` &le; 1.0
    /// * 0.0 &le; `lightness` &le; 1.0
    /// * 0.0 &le; `alpha` &le; 1.0
    ///
    /// ```rust
    /// use turtle::Color;
    ///
    /// // You can chain using Color::from()
    /// let black = Color::from("black").with_alpha(0.5);
    /// let black_hsla = Color::hsla(0.0, 0.0, 0.0, 0.5);
    /// assert_eq!(black, black_hsla);
    ///
    /// // But even better, you can use the color enum value and chain the
    /// // calls.
    /// let white = Color::WHITE.with_alpha(0.75);
    /// let white_hsla = Color::hsla(0.0, 1.0, 1.0, 0.75);
    /// assert_eq!(white, white_hsla);
    ///
    /// let blue: Color = Color::BLUE.with_alpha(0.8);
    /// let blue_hsla = Color::hsla(201.0, 1.0, 0.392, 0.8);
    /// assert_eq!(blue, blue_hsla);
    /// ```
    /// [`HSL`]: https://en.wikipedia.org/wiki/HSL_and_HSV
    pub fn hsla(hue: f64, saturation: f64, lightness: f64, alpha: f64) -> Self {
        assert_value_in_range!("hue", hue, COLOR_MIN_VALUE, HUE_MAX_VAL);
        assert_value_in_range!("saturation", saturation, COLOR_MIN_VALUE, SAL_MAX_VAL);
        assert_value_in_range!("lightness", lightness, COLOR_MIN_VALUE, SAL_MAX_VAL);
        assert_value_in_range!("alpha", alpha, COLOR_MIN_VALUE, SAL_MAX_VAL);

        // Most of this code comes courtesy of work done by 'killercup' on GitHub (MIT Licensed) and
        // the answer here: https://stackoverflow.com/a/9493060
        // Link: https://github.com/killercup/hsl-rs/blob/d3b0ff50091f45bc5cf28a765be1aa52a70461d3/src/lib.rs#L111
        if saturation == 0.0 {
            let achromatic = (lightness * 255.).round();
            return Color::rgba(achromatic, achromatic, achromatic, alpha);
        }

        // Given a hue value, convert it to the percentage of a given RGB color
        // it should be.
        fn hue_to_rgb(p: f64, q: f64, t: f64) -> f64 {
            let t = match t {
                t if t < 0. => t + 1.,
                t if t > 1. => t - 1.,
                _ => t,
            };

            match t {
                t if t < 1. / 6. => p + (q - p) * 6. * t,
                t if t < 1. / 2. => q,
                t if t < 2. / 3. => p + (q - p) * (2. / 3. - t) * 6.,
                _ => p,
            }
        };

        let q = if lightness < 0.5 {
            lightness * (1. + saturation)
        } else {
            lightness + saturation - (lightness * saturation)
        };

        let p = lightness.mul_add(2., -q);
        let h = hue / 360.;

        // Change the percentage value returned from hue_to_rgb to an actual
        // rgb value between 0 and 255
        let red: f64 = (hue_to_rgb(p, q, h + 1. / 3.) * 255.).round();
        let green: f64 = (hue_to_rgb(p, q, h) * 255.).round();
        let blue: f64 = (hue_to_rgb(p, q, h - 1. / 3.) * 255.).round();

        Color::rgba(red, green, blue, alpha)
    }

    /// Returns true if the values for each field are valid.
    ///
    /// The documentation above lists the valid range for each field.
    pub fn is_valid(&self) -> bool {
        self.red >= 0.0
            && self.red <= 255.0
            && self.red.is_finite()
            && self.green >= 0.0
            && self.green <= 255.0
            && self.green.is_finite()
            && self.blue >= 0.0
            && self.blue <= 255.0
            && self.blue.is_finite()
            && self.alpha >= 0.0
            && self.alpha <= 1.0
            && self.alpha.is_finite()
    }

    /// Return a new color with all of the same values except with opacity (alpha) set to 1.0
    ///
    /// ```rust
    /// use turtle::Color;
    /// let color = Color {red: 43.0, green: 79.0, blue: 23.0, alpha: 0.5};
    /// assert_eq!(color.alpha, 0.5);
    /// let color2 = color.opaque();
    /// assert_eq!(color.alpha, 0.5);
    /// assert_eq!(color2.alpha, 1.0);
    /// ```
    pub fn opaque(self) -> Color {
        assert_color_valid!(self);
        self.with_alpha(1.0)
    }

    /// Return a new color with all of the same values except with opacity (alpha) set to 0.0
    ///
    /// ```rust
    /// use turtle::Color;
    /// let color = Color {red: 43.0, green: 79.0, blue: 23.0, alpha: 0.5};
    /// assert_eq!(color.alpha, 0.5);
    /// let color2 = color.transparent();
    /// assert_eq!(color.alpha, 0.5);
    /// assert_eq!(color2.alpha, 0.0);
    /// ```
    pub fn transparent(self) -> Color {
        assert_color_valid!(self);
        self.with_alpha(0.0)
    }

    /// Return a new color with alpha set to the given value
    ///
    /// ```rust
    /// use turtle::Color;
    /// let color = Color {red: 43.0, green: 79.0, blue: 23.0, alpha: 0.5};
    /// assert_eq!(color.alpha, 0.5);
    /// let color2 = color.with_alpha(0.2);
    /// assert_eq!(color.alpha, 0.5);
    /// assert_eq!(color2.alpha, 0.2);
    /// ```
    pub fn with_alpha(mut self, alpha: f64) -> Color {
        self.alpha = alpha;
        self
    }

    /// Mix this color with the other given color, with the given weighting.
    ///
    /// ```rust
    /// use turtle::Color;
    ///
    /// // Start with red
    /// let red: Color = "red".into();
    ///
    /// // Create a mix that is 60% blue and 40% red
    /// let mixed = red.mix("blue", 0.40);
    /// let expected: Color = [92.0, 88.0, 150.0, 1.0].into();
    ///
    /// assert_eq!(mixed, expected);
    /// ```
    ///
    /// The `weight` parameter is a value between 0.0 and 1.0. It determines the percentage to mix
    /// `self` with `other`. So let's say that in the example above, we had used
    /// `red.mix("blue", 0.5)`. 0.5 means 50%, so we would mix 50% red and 50% blue. The colors
    /// would be mixed evenly. If we had instead done `red.mix("blue", 0.25)`, the weight would be
    /// 0.25 which is 25%. That means that we would get a mix of 25% red and 75% blue.
    ///
    /// The alpha channel of both colors are also combined in the same way.
    ///
    /// # Panics
    ///
    /// Passing a value for `weight` that is not between 0.0 and 1.0 will result in a `panic`
    ///
    /// ```should_panic
    /// use turtle::Color;
    ///
    /// let orange: Color = "orange".into();
    ///
    /// // This will panic as 1.01 is not a valid value for weight, which must be between 0.0 and 1.0.
    /// let mixed = orange.mix(Color::BROWN.with_alpha(0.8), 1.01);
    /// ```
    ///
    /// # Example
    ///
    /// Let's look at a more complete example to really show what happens when we're mixing colors together.
    ///
    /// ```no_run
    /// use turtle::{Color, Drawing};
    ///
    /// fn main() {
    ///     let mut drawing = Drawing::new();
    ///     let mut turtle = drawing.add_turtle();
    ///
    ///     drawing.set_title("Mixing colors!");
    ///     turtle.set_pen_size(5.0);
    ///
    ///     let red: Color = "red".into();
    ///
    ///     // This will draw a red line
    ///     turtle.set_pen_color(red);
    ///     turtle.forward(100.0);
    ///     turtle.right(90.0);
    ///
    ///     // This will draw a purple line because we have equally mixed red with blue
    ///     turtle.set_pen_color(red.mix("blue", 0.5));
    ///     turtle.forward(100.0);
    ///     turtle.right(90.0);
    ///
    ///     // This will draw a line that is 25% red and 75% blue (a red-ish dark blue color)
    ///     turtle.set_pen_color(red.mix("blue", 0.25));
    ///     turtle.forward(100.0);
    ///     turtle.right(90.0);
    ///
    ///     // This will draw a line that is 75% red and 25% blue (medium red violet)
    ///     turtle.set_pen_color(red.mix("blue", 0.75));
    ///     turtle.forward(100.0);
    ///     turtle.right(90.0);
    /// }
    /// ```
    ///
    /// Running the above program will result in the following image:
    /// ![turtle color mixing](https://github.com/sunjay/turtle/raw/9240f8890d1032a0033ec5c5338a10ffa942dc21/docs/assets/images/docs/color_mixing.png)
    pub fn mix<C: Into<Color> + Copy + Debug>(self, other: C, weight: f64) -> Self {
        assert_value_in_range!("weight", weight, 0.0, 1.0);

        // This algorithm (and the explanation) cribbed from Sass
        // (http://sass-lang.com/documentation/Sass/Script/Functions.html#mix-instance_method)

        // It factors in the user-provided weight (w) and the difference between the alpha values of the colors (a) to decide
        // how to perform the weighted average of the two RGB colors.
        // It works by first normalizing both parameters to be within [-1, 1],
        // where 1 indicates "only use color1", -1 indicates "only use color2", and
        // all values in between indicated a proportionately weighted average.
        //
        // Once we have the normalized variables w and a, we apply the formula
        // (w + a)/(1 + w*a) to get the combined weight (in [-1, 1]) of color1.
        // This formula has two especially nice properties:
        //
        //   * When either w or a are -1 or 1, the combined weight is also that number
        //     (cases where w * a == -1 are undefined, and handled as a special case).
        //
        //   * When a is 0, the combined weight is w, and vice versa.
        //
        // Finally, the weight of color1 is renormalized to be within [0, 1]
        // and the weight of color2 is given by 1 minus the weight of color1.
        let with_color = other.into();
        assert_color_valid!(self);
        assert_color_valid!(with_color);

        let p = weight;
        let w = p.mul_add(2., -1.);
        let a = self.alpha - with_color.alpha;

        let w1 = if f64_eq(w * a, -1.0) {
            (w + 1.) / 2.
        } else {
            ((w + a) / a.mul_add(w, 1.) + 1.) / 2.
        };

        let w2 = 1. - w1;

        let r_mod = self.red.mul_add(w1, with_color.red * w2).round();
        let g_mod = self.green.mul_add(w1, with_color.green * w2).round();
        let b_mod = self.blue.mul_add(w1, with_color.blue * w2).round();
        let a_mod = self.alpha.mul_add(p, with_color.alpha * (1. - p));
        Color::rgba(r_mod, g_mod, b_mod, a_mod)
    }

    /// Retrieve the hue for this `Color`. The returned value is in degrees
    /// between 0° and 360° that represents its position on the color wheel.
    ///
    /// ```rust
    /// use turtle::Color;
    ///
    /// let c: Color = "blue".into();
    /// assert_eq!(201.0, c.hue());
    /// ```
    pub fn hue(self) -> f64 {
        self.to_hsl().0
    }

    /// Retrieve the saturation value for this `Color`. The returned value is
    /// between 0.0 and 1.0 (inclusive) that indicates the saturation percentage.
    ///
    /// ```rust
    /// use turtle::Color;
    ///
    /// let c: Color = "blue".into();
    /// assert_eq!(1.0, c.saturation());
    /// ```
    pub fn saturation(self) -> f64 {
        self.to_hsl().1
    }

    /// Retrieve the lightness value for this `Color`. The returned value is between
    /// 0.0 and 1.0 (inclusive) that indicates the lightness percentage.
    ///
    /// ```rust
    /// use turtle::Color;
    ///
    /// let c: Color = "blue".into();
    /// assert_eq!(0.39215686274509803, c.lightness());
    /// ```
    pub fn lightness(self) -> f64 {
        self.to_hsl().2
    }

    /// Changes the hue of a color. Takes a color and a number of degrees
    /// (usually between -360° and 360°), and returns a color with the hue
    /// rotated along the color wheel by that amount.
    ///
    /// ```rust
    /// use turtle::Color;
    ///
    /// // Positive values
    /// let c = Color::hsl(120., 0.3, 0.9);
    /// assert_eq!(c.rotate_hue(60.), Color::hsl(180.0, 0.3, 0.9));
    ///
    /// // Negative values
    /// assert_eq!(c.rotate_hue(-60.), Color::hsl(60.0, 0.3, 0.9));
    /// ```
    ///
    /// Passing hue values outside the range of -360 and 360 will result in a `panic`.
    ///
    /// So, passing a value that is too small:
    ///
    /// ```should_panic
    /// use turtle::Color;
    ///
    /// let red: Color = "red".into();
    ///
    /// // This will panic, as -361 is outside the allowed range
    /// let improper = red.rotate_hue(-361.0);
    /// ```
    ///
    /// Or one that is too large:
    ///
    /// ```should_panic
    /// use turtle::Color;
    ///
    /// let blue: Color = "blue".into();
    ///
    /// // This will panic as 361 is outside the allowed range
    /// let improper = blue.rotate_hue(361.0);
    /// ```
    pub fn rotate_hue(self, hue: f64) -> Self {
        assert_value_in_range!("hue", hue, -360., 360.);

        let (h, s, l) = self.to_hsl();
        // Normalize the hue to within -360 and +360
        let mut hue_mod = (h + hue) % 360.;
        if hue_mod < 0. {
            hue_mod += 360.;
        }

        Color::hsla(hue_mod, s, l, self.alpha)
    }

    /// Create a new `Color` by increasing the lightness of this `Color` by
    /// the given percentage. The value is a float between 0.0 and 1.0
    /// indicating the percentage to increase the lightness. So, if you wish
    /// to make a `Color` 10% lighter, you would specify 0.1.
    ///
    /// ```rust
    /// use turtle::Color;
    ///
    /// let original = Color::hsl(158.0, 0.8, 0.3);
    ///
    /// // Now, let's increase the lightness of original by 50%
    /// let lighter = original.lighten(0.5);
    /// assert_eq!(lighter, Color::hsl(158.0, 0.8, 0.8));
    /// ```
    ///
    /// Now, as the maximum lightness a `Color` can have is 100%, trying
    /// to increase it beyond that point will result in a `Color` with
    /// a lightness value of 1.0
    ///
    /// ```rust
    /// use turtle::Color;
    ///
    /// let original = Color::hsl(200.0, 0.7, 0.8);
    ///
    /// // Now, we'll increase the lightness by 50%, which would go beyond 100%
    /// let lighter = original.lighten(0.5);
    ///
    /// // But, the lightness of the color will still max out at 1.0
    /// assert_eq!(lighter, Color::hsl(200.0, 0.7, 1.0));
    /// ```
    ///
    /// Providing values greater than 1.0 will result in a panic
    ///
    /// ```should_panic
    /// use turtle::Color;
    ///
    /// let original = Color::hsl(150.0, 0.5, 0.3);
    ///
    /// // This will panic as a value of 1.1 is greater than the acceptable
    /// // maximum of 1.0
    /// let incorrect = original.lighten(1.1);
    /// ```
    ///
    /// Negative values will also result in a panic
    ///
    /// ```should_panic
    /// use turtle::Color;
    ///
    /// let original = Color::hsl(150.0, 0.5, 0.8);
    ///
    /// // This will panic, as a negative value is less than the acceptable
    /// // minimum of 0.0
    /// let incorrect = original.lighten(-0.3);
    /// ```
    ///
    /// If you want to lighten by a negative amount, please see [`darken`].
    ///
    /// ```rust
    /// use turtle::Color;
    ///
    /// let original = Color::hsl(25.0, 1.0, 0.8);
    ///
    /// // Instead of -0.3 (-30%) as in the previous example, we will just darken by 30%
    /// let darker = original.darken(0.3);
    /// assert_eq!(darker, Color::hsl(25.0, 1.0, 0.5));
    /// ```
    /// [`darken`]: ./struct.Color.html#method.darken
    pub fn lighten(self, lighter: f64) -> Self {
        assert_value_in_range!("lighter", lighter, 0., 1.);

        let (h, s, l) = self.to_hsl();

        let l_mod = if l + lighter <= 1. { l + lighter } else { 1. };

        Color::hsla(h, s, l_mod, self.alpha)
    }

    /// Create a new `Color` by decreasing the lightness of this `Color` by
    /// the given percentage. The value is a float between 0.0 and 1.0
    /// indicating the percentage to decrease the lightness. So, if you wish
    /// to make a `Color` 10% darker, you would specify 0.1.
    ///
    /// ```rust
    /// use turtle::Color;
    ///
    /// let original = Color::hsl(25.0, 1.0, 0.8);
    ///
    /// // Let's make the color 30% darker.
    /// let darker = original.darken(0.3);
    /// assert_eq!(darker, Color::hsl(25.0, 1.0, 0.5));
    /// ```
    ///
    /// As the minimum lightness a `Color` can have is 0%, attempting to
    /// decrease beyond that will result in a `Color` with a lightness of 0.
    ///
    /// ```rust
    /// use turtle::Color;
    ///
    /// let original = Color::hsl(100.0, 1.0, 0.3);
    ///
    /// // Let's try to decrease by 40%, which would result in a negative lightness
    /// let darker = original.darken(0.4);
    ///
    /// // Since we can't go below 0%, the lightness of the resulting `Color` will be 0
    /// assert_eq!(darker, Color::hsl(100.0, 1.0, 0.0));
    /// ```
    ///
    /// Providing values greater than 1.0 will result in a panic
    ///
    /// ```should_panic
    /// use turtle::Color;
    ///
    /// let original = Color::hsl(150.0, 0.3, 0.5);
    ///
    /// // This will panic as 1.1 is greater than the acceptable maximum of 1.0
    /// let incorrect = original.darken(1.1);
    /// ```
    ///
    /// Negative values will also result in a panic
    ///
    /// ```should_panic
    /// use turtle::Color;
    ///
    /// let original = Color::hsl(150.0, 0.3, 0.5);
    ///
    /// // This will panic as a negative value is less than the acceptable
    /// // minimum of 0.0
    /// let incorrect = original.darken(-0.1);
    /// ```
    ///
    /// If you want to darken by a negative value please see [`lighten`].
    ///
    /// ```rust
    /// use turtle::Color;
    ///
    /// let original = Color::hsl(158.0, 0.8, 0.3);
    ///
    /// // Now, we'll increase the lightness by 10%, instead of trying to darken by
    /// // a negative amount
    /// let lighter = original.lighten(0.1);
    /// assert_eq!(lighter, Color::hsl(158.0, 0.8, 0.4));
    /// ```
    /// [`lighten`]: ./struct.Color.html#method.lighten
    pub fn darken(self, darker: f64) -> Self {
        assert_value_in_range!("darker", darker, 0., 1.);

        let (h, s, l) = self.to_hsl();

        let l_mod = if l - darker >= 0. { l - darker } else { 0. };
        Color::hsla(h, s, l_mod, self.alpha)
    }

    /// Create a new `Color` by increasing the saturation level of this
    /// `Color` by the given percentage. The value is a float between
    /// 0.0 and 1.0 indicating the percentage to increase the saturation.
    /// So, if you wish to create a `Color` that is 30% more saturated than
    /// this one, you would specify 0.3.
    ///
    /// For more information on what saturation is in relation to HSL colors,
    /// please see this [Wikipedia Article].
    ///
    /// ```rust
    /// use turtle::Color;
    ///
    /// let original = Color::hsl(120.0, 0.3, 0.9);
    ///
    /// // Now, let's increase the saturation by 20%
    /// let saturated = original.saturate(0.2);
    /// assert_eq!(saturated, Color::hsl(120.0, 0.5, 0.9));
    /// ```
    ///
    /// The maximum saturation level a `Color` can have is 100%. If you try
    /// to increase beyond that level using this method, it will result in a
    /// `Color` with a `saturation` value of 1.0.
    ///
    /// ```rust
    /// use turtle::Color;
    ///
    /// let original = Color::hsl(120.0, 0.8, 0.9);
    ///
    /// // We try to increase the saturation by 50%, which would equal 130%
    /// let saturated = original.saturate(0.5);
    ///
    /// // But the resulting color only has a saturation value of 1.0
    /// assert_eq!(saturated, Color::hsl(120.0, 1.0, 0.9));
    /// ```
    ///
    /// Passing values that are greater than 1.0 will result in a panic
    ///
    /// ```should_panic
    /// use turtle::Color;
    ///
    /// let original = Color::hsl(120.0, 0.5, 0.9);
    ///
    /// // This will panic, as 1.1 is greater than the maximum accepted value of 1.0
    /// let incorrect = original.saturate(1.1);
    /// ```
    ///
    /// Negative values will also result in a panic
    ///
    /// ```should_panic
    /// use turtle::Color;
    ///
    /// let original = Color::hsl(120.0, 0.5, 0.9);
    ///
    /// // This will panic, as a negative value is less than the minimum acceptable
    /// // value of 0.0
    /// let incorrect = original.saturate(-0.2);
    /// ```
    ///
    /// If you wish to desaturate a `Color` please see [`desaturate`].
    ///
    /// ```rust
    /// use turtle::Color;
    ///
    /// let original = Color::hsl(120.0, 0.3, 0.9);
    ///
    /// // Instead of trying to saturate with a negative value as in the
    /// // previous example, we simply desaturate by the same positive amount
    /// let desaturated = original.desaturate(0.2);
    /// assert_eq!(desaturated, Color::hsl(120.0, 0.1, 0.9));
    /// ```
    /// [Wikipedia Article]: https://en.wikipedia.org/wiki/HSL_and_HSV#Saturation
    /// [`desaturate`]: ./struct.Color.html#method.desaturate
    pub fn saturate(self, saturation: f64) -> Self {
        assert_value_in_range!("saturation", saturation, 0., 1.);

        let (h, s, l) = self.to_hsl();
        let s_mod = if s + saturation <= 1. { s + saturation } else { 1. };
        Color::hsla(h, s_mod, l, self.alpha)
    }

    /// Create a new `Color` by decreasing the saturation level of this `Color`
    /// by the given percentage. The value is a float between
    /// 0.0 and 1.0 indicating the percentage to decrease the saturation.
    /// So, if you wish to create a `Color` that is 30% less saturated than
    /// this one, you would specify 0.3.
    ///
    /// For more information on what saturation is in relation to HSL colors,
    /// please see this [Wikipedia Article].
    ///
    /// ```rust
    /// use turtle::Color;
    ///
    /// let original = Color::hsl(120.0, 0.3, 0.9);
    ///
    /// // Now, desaturate the original by 20%
    /// let desaturated = original.desaturate(0.2);
    /// assert_eq!(desaturated, Color::hsl(120.0, 0.1, 0.9));
    /// ```
    ///
    /// Since the minimum saturation value a color can have is 0%, attempting
    /// to desaturate beyond that level will simply result in a `Color` with a
    /// `saturation` value of 0.
    ///
    /// ```rust
    /// use turtle::Color;
    ///
    /// let original = Color::hsl(120.0, 0.3, 0.9);
    ///
    /// // Now, desaturate the color by 50%, which would equal -20%
    /// let desaturated = original.desaturate(0.5);
    ///
    /// // However the resulting color simply has a saturation of 0.0
    /// assert_eq!(desaturated, Color::hsl(120.0, 0.0, 0.9));
    /// ```
    ///
    /// A color with a saturation level of 0.0 is known
    /// as an '[achromatic]' color. As they have no hue, they are the range
    /// of all gray colors, ranging from white to black.
    ///
    /// Passing values that are greater than 1.0 will result in a panic
    ///
    /// ```should_panic
    /// use turtle::Color;
    ///
    /// let original = Color::hsl(120.0, 0.3, 0.9);
    ///
    /// // This will panic, as 1.1 is greater than the acceptable maximum of 1.0
    /// let incorrect = original.desaturate(1.1);
    /// ```
    ///
    /// Passing negative values will also panic
    ///
    /// ```should_panic
    /// use turtle::Color;
    ///
    /// let original = Color::hsl(120.0, 0.3, 0.7);
    ///
    /// // This will panic, as a negative value is less than the acceptable
    /// // minimum value of 0.0
    /// let incorrect = original.desaturate(-0.2);
    /// ```
    ///
    /// If you wish to saturate a `Color`, please see [`saturate`]
    ///
    /// ```rust
    /// use turtle::Color;
    ///
    /// let original = Color::hsl(120.0, 0.3, 0.9);
    ///
    /// // Now, let's increase the saturation by 20% instead of trying to
    /// // desaturate by a negative number
    /// let saturated = original.saturate(0.2);
    /// assert_eq!(saturated, Color::hsl(120.0, 0.5, 0.9));
    /// ```
    /// [Wikipedia Article]: https://en.wikipedia.org/wiki/HSL_and_HSV#Saturation
    /// [achromatic]: https://en.wikipedia.org/wiki/Color_scheme#Achromatic_colors
    /// [`saturate`]: ./struct.Color.html#method.saturate
    pub fn desaturate(self, saturation: f64) -> Self {
        assert_value_in_range!("saturation", saturation, 0., 1.);

        let (h, s, l) = self.to_hsl();
        let s_mod = if s - saturation >= 0. { s - saturation } else { 0. };
        Color::hsla(h, s_mod, l, self.alpha)
    }

    /// Convert this `Color` to grayscale, which is essentailly desaturating
    /// it by 100%. For more information on desaturation please see [`desaturate`].
    ///
    /// ```rust
    /// use turtle::Color;
    ///
    /// // Let's start with a standard HSL color
    /// let original = Color::hsl(0.0, 0.5, 0.5);
    ///
    /// // Now, when we switch this to grayscale, we will end up with Turtle 'grey'
    /// assert_eq!(original.grayscale(), "grey".into());
    /// ```
    ///
    /// A `Color` that has no saturation is known as an '[achromatic]' `Color`, which
    /// are gray colors ranging from white to black.
    ///
    /// Since this is essentally removing all saturation you can verify that
    /// the grayscale version of any `Color` is the same `hue` and `lightness` values
    /// with 0 `saturation`.
    ///
    /// ```rust
    /// use turtle::Color;
    ///
    /// // An arbitrary HSL color
    /// let original = Color::hsl(200.0, 0.9, 1.0);
    ///
    /// // The grayscale version simply has a saturation of 0
    /// assert_eq!(original.grayscale(), Color::hsl(200.0, 0.0, 1.0));
    /// ```
    /// [`desaturate`]: ./struct.Color.html#method.desaturate
    /// [achromatic]: https://en.wikipedia.org/wiki/Color_scheme#Achromatic_colors
    pub fn grayscale(self) -> Self {
        self.desaturate(1.0)
    }

    /// Create a new `Color` by obtaining the complement (opposite) of this `Color`.
    /// The complement of a color is 180 degrees around the color wheel. For more
    /// information on rotating the hue of a `Color` please see [`rotate_hue`].
    ///
    /// This [Wikipedia Article] contains more information about complementary colors.
    ///
    /// ```rust
    /// use turtle::Color;
    ///
    /// // A standard HSL color
    /// let original = Color::hsl(100.0, 0.7, 1.0);
    ///
    /// // The complement will be have a hue value that is 180 degrees greater
    /// assert_eq!(original.complement(), Color::hsl(280.0, 0.7, 1.0));
    /// ```
    /// [`rotate_hue`]: ./struct.Color.html#method.rotate_hue
    /// [Wikipedia Article]: https://en.wikipedia.org/wiki/Complementary_colors
    pub fn complement(self) -> Self {
        self.rotate_hue(180.)
    }

    /// Create a `Color` that is the inverse (negative) of this
    /// `Color`. The `red`, `green`, and `blue` values of this
    /// color are inverted but `alpha` is not touched.
    ///
    /// For more information about what the inverse of a `Color`
    /// is, please see this [StackExchange Answer].
    ///
    /// This `Color` is mixed with the inverted values to produce
    /// the inverse of it. The mix is done with a weight of 1.0 for
    /// the invert values. For more information see [`mix`].
    ///
    /// ```rust
    /// use turtle::Color;
    ///
    /// // We will start with blue. Turtle blue is rgb(0, 130, 200)
    /// let blue: Color = "blue".into();
    ///
    /// // The inverse of blue is a light orange color. Note that the
    /// // original color had a default alpha value of 1.0, so we can
    /// // make sure the resulting color also has the same alpha value
    /// assert_eq!(blue.invert(), [255.0, 125.0, 55.0, 1.0].into());
    /// ```
    /// [`mix`]: ./struct.Color.html#method.mix
    /// [StackExchange Answer]: https://graphicdesign.stackexchange.com/a/95100
    pub fn invert(self) -> Self {
        assert_color_valid!(self);

        let inv_r = 255. - self.red;
        let inv_g = 255. - self.green;
        let inv_b = 255. - self.blue;

        let inv_color = Color::rgba(inv_r, inv_g, inv_b, self.alpha);
        inv_color.mix(self, 1.0)
    }

    /// Helper to switch a given RGB `Color` to HSL values.
    ///
    /// Answer adapted from this SO answer (https://stackoverflow.com/a/9493060)
    /// and more information about the underlying algorithm can be found on
    /// https://en.wikipedia.org/wiki/HSL_and_HSV
    fn to_hsl(&self) -> (f64, f64, f64) {
        /* Check that the color is valid here, this covers the following methods:
         *  - saturate
         *  - desaturate
         *  - grayscale
         *  - darken
         *  - lighten
         *  - rotate_hue
         *  - hue
         *  - saturation
         *  - lightness
         *  - complement
         */
        assert_color_valid!(self);
        let div_color = |c| c / 255.0;
        let (r, g, b) = (div_color(self.red), div_color(self.green), div_color(self.blue));

        let max = r.max(g.max(b));
        let min = r.min(g.min(b));
        let hue: f64;
        let saturation: f64;
        let lightness = (max + min) * 0.5;

        if f64_eq(max, min) {
            hue = 0.;
            saturation = 0.;
        } else {
            let d = max - min;
            saturation = if lightness > 0.5 { d / (2. - max - min) } else { d / (max + min) };

            hue = match max {
                _ if f64_eq(max, r) => (g - b) / d + if g < b { 6. } else { 0. },
                _ if f64_eq(max, g) => (b - r) / d + 2.,
                _ => (r - g) / d + 4.,
            } * 60.;
        }

        (hue.round(), saturation, lightness)
    }
}

impl Random for Color {
    fn random() -> Self {
        Self {
            red: <f64 as Random>::random() * 255.,
            green: <f64 as Random>::random() * 255.,
            blue: <f64 as Random>::random() * 255.,
            alpha: <f64 as Random>::random(),
        }
    }
}

impl<B: Into<Color>> RandomRange<B> for Color {
    fn random_range(low: B, high: B) -> Self {
        let low = low.into();
        let high = high.into();
        Self {
            red: RandomRange::random_range(low.red, high.red),
            green: RandomRange::random_range(low.green, high.green),
            blue: RandomRange::random_range(low.blue, high.blue),
            alpha: RandomRange::random_range(low.alpha, high.alpha),
        }
    }
}

impl From<[f64; 3]> for Color {
    fn from(color_values: [f64; 3]) -> Self {
        Self::rgb(color_values[0], color_values[1], color_values[2])
    }
}

impl From<[f64; 4]> for Color {
    fn from(color_values: [f64; 4]) -> Self {
        Self::rgba(color_values[0], color_values[1], color_values[2], color_values[3])
    }
}

impl<'a> From<&'a str> for Color {
    fn from(s: &'a str) -> Self {
        if s.starts_with('#') {
            let color_str = &s[1..];
            // Color strings can either be of size 3 (rgb) or 6 (rrggbb)
            // e.g. 3366ff == 36f
            let color_str = match color_str.len() {
                3 => color_str.chars().flat_map(|c| repeat(c).take(2)).collect(),
                6 => color_str.to_owned(),
                _ => panic!("Invalid color literal: {}", s),
            };

            // Use closure here as 's' cannot be captured when using nested fn form
            let extract_color_value = |v| i64::from_str_radix(v, 16)
                .unwrap_or_else(|_| panic!("Invalid color literal: {}", s)) as f64;

            let red = extract_color_value(&color_str[0..2]);
            let green = extract_color_value(&color_str[2..4]);
            let blue = extract_color_value(&color_str[4..6]);

            Self::rgb(red, green, blue)
        } else {
            from_color_name(s)
                .or_else(|| color_consts::from_color_name(s))
                .unwrap_or_else(|| panic!("Unknown color name: {}", s))
        }
    }
}

macro_rules! color_consts {
    ($($name:expr, $id:ident, ($r:expr, $g:expr, $b:expr, $a:expr);)*) => {
        /// See [`extra_colors`](extra_colors/index.html) for more color names
        /// that you can use.
        impl Color {
            $(
                /// Use the name `
                #[doc = $name]
                /// ` to specify this color
                pub const $id: Color = Color {red: $r, green: $g, blue: $b, alpha: $a};
            )*

            /// Return a list of all of the colors.
            //TODO: Hiding this for now because it doesn't actually contain all
            // the available colors from the entire crate
            #[allow(dead_code)]
            fn all_colors() -> &'static [Color] {
                &[$(Color::$id, )*]
            }

            /// Return a list of all of the color names.
            //TODO: Hiding this for now because it doesn't actually contain all
            // the available colors from the entire crate
            #[allow(dead_code)]
            fn all_color_names() -> &'static [&'static str] {
                &[$($name, )*]
            }
        }

        pub(crate) fn from_color_name(s: &str) -> Option<Color> {
            match s {
                $(
                    $name => Some(Color::$id),
                )*
                _ => None,
            }
        }
    }
}

// Most important colors are put in the main module, the remaining are in `extra_colors`.
// We do this so that documentation doesn't get overloaded with constants.
color_consts! {
    "transparent", TRANSPARENT, (0.0, 0.0, 0.0, 0.0);
    "red", RED,	(230.0, 25.0, 75.0, 1.0);
    "green", GREEN,	(60.0, 180.0, 75.0, 1.0);
    "yellow", YELLOW,	(255.0, 225.0, 25.0, 1.0);
    "blue", BLUE,	(0.0, 130.0, 200.0, 1.0);
    "orange", ORANGE,	(245.0, 130.0, 48.0, 1.0);
    "purple", PURPLE,	(145.0, 30.0, 180.0, 1.0);
    "cyan", CYAN,	(70.0, 240.0, 240.0, 1.0);
    "magenta", MAGENTA,	(240.0, 50.0, 230.0, 1.0);
    "lime", LIME,	(210.0, 245.0, 60.0, 1.0);
    "pink", PINK,	(250.0, 190.0, 190.0, 1.0);
    "teal", TEAL,	(0.0, 128.0, 128.0, 1.0);
    "lavender", LAVENDER,	(230.0, 190.0, 255.0, 1.0);
    "brown", BROWN,	(170.0, 110.0, 40.0, 1.0);
    "beige", BEIGE,	(255.0, 250.0, 200.0, 1.0);
    "maroon", MAROON,	(128.0, 0.0, 0.0, 1.0);
    "mint", MINT,	(170.0, 255.0, 195.0, 1.0);
    "olive", OLIVE,	(128.0, 128.0, 0.0, 1.0);
    "coral", CORAL,	(255.0, 215.0, 180.0, 1.0);
    "navy", NAVY,	(0.0, 0.0, 128.0, 1.0);
    "grey", GREY,	(128.0, 128.0, 128.0, 1.0);
    "white", WHITE,	(255.0, 255.0, 255.0, 1.0);
    "black", BLACK,	(0.0, 0.0, 0.0, 1.0);
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::f64::{EPSILON, INFINITY as INF, NAN};

    #[test]
    fn color_equivalence() {
        let c = Color {
            red: 51.0,
            green: 85.0,
            blue: 255.0,
            alpha: 1.0,
        };
        let c1: Color = "#35f".into();
        let c2: Color = "#3355ff".into();
        let c3: Color = "#35F".into();
        let c4: Color = "#3355FF".into();
        assert_eq!(c, c1);
        assert_eq!(c1, c2);
        assert_eq!(c2, c3);
        assert_eq!(c3, c4);
    }

    #[test]
    fn fields_mapped_correctly() {
        // Check if array syntax maps color values to the correct fields
        let c: Color = [2.4, 3.2, 4.6, 0.67].into();
        assert_eq!(
            c,
            Color {
                red: 2.4,
                green: 3.2,
                blue: 4.6,
                alpha: 0.67
            }
        );

        let c: Color = [2.4, 3.2, 4.6].into();
        assert_eq!(
            c,
            Color {
                red: 2.4,
                green: 3.2,
                blue: 4.6,
                alpha: 1.0
            }
        );
    }

    #[test]
    #[should_panic(expected = "Invalid color literal: #fffff")]
    fn invalid_color1() {
        // Wrong number of digits
        Color::from("#fffff");
    }

    #[test]
    #[should_panic(expected = "Invalid color literal: #www")]
    fn invalid_color2() {
        // Invalid hex character
        Color::from("#www");
    }

    #[test]
    fn valid_colors() {
        // Test that all colors in their valid ranges are valid
        // Floating-point numbers have infinite ranges, so this is not an exhaustive test
        for i in 0..(255 * 2 + 1) {
            let i = i as f64 / 2.0;

            let color = Color {
                red: i,
                green: 0.0,
                blue: 0.0,
                alpha: 0.0,
            };
            assert!(color.is_valid(), "{:?}", color);
            let color = Color {
                red: 0.0,
                green: i,
                blue: 0.0,
                alpha: 0.0,
            };
            assert!(color.is_valid(), "{:?}", color);
            let color = Color {
                red: 0.0,
                green: 0.0,
                blue: i,
                alpha: 0.0,
            };
            assert!(color.is_valid(), "{:?}", color);
            let color = Color {
                red: 0.0,
                green: 0.0,
                blue: 0.0,
                alpha: i / 255.0,
            };
            assert!(color.is_valid(), "{:?}", color);

            let color = Color {
                red: 255.0 - i,
                green: i,
                blue: 255.0 - i,
                alpha: i / 255.0,
            };
            assert!(color.is_valid(), "{:?}", color);
        }
    }

    #[test]
    fn invalid_color3() {
        assert!(
            !Color {
                red: NAN,
                green: 0.0,
                blue: 0.0,
                alpha: 0.0
            }.is_valid()
        );
        assert!(
            !Color {
                red: 0.0,
                green: NAN,
                blue: 0.0,
                alpha: 0.0
            }.is_valid()
        );
        assert!(
            !Color {
                red: 0.0,
                green: 0.0,
                blue: NAN,
                alpha: 0.0
            }.is_valid()
        );
        assert!(
            !Color {
                red: 0.0,
                green: 0.0,
                blue: 0.0,
                alpha: NAN
            }.is_valid()
        );
        assert!(
            !Color {
                red: NAN,
                green: NAN,
                blue: NAN,
                alpha: NAN
            }.is_valid()
        );

        assert!(
            !Color {
                red: INF,
                green: 0.0,
                blue: 0.0,
                alpha: 0.0
            }.is_valid()
        );
        assert!(
            !Color {
                red: 0.0,
                green: INF,
                blue: 0.0,
                alpha: 0.0
            }.is_valid()
        );
        assert!(
            !Color {
                red: 0.0,
                green: 0.0,
                blue: INF,
                alpha: 0.0
            }.is_valid()
        );
        assert!(
            !Color {
                red: 0.0,
                green: 0.0,
                blue: 0.0,
                alpha: INF
            }.is_valid()
        );
        assert!(
            !Color {
                red: INF,
                green: INF,
                blue: INF,
                alpha: INF
            }.is_valid()
        );

        assert!(
            !Color {
                red: -INF,
                green: 0.0,
                blue: 0.0,
                alpha: 0.0
            }.is_valid()
        );
        assert!(
            !Color {
                red: 0.0,
                green: -INF,
                blue: 0.0,
                alpha: 0.0
            }.is_valid()
        );
        assert!(
            !Color {
                red: 0.0,
                green: 0.0,
                blue: -INF,
                alpha: 0.0
            }.is_valid()
        );
        assert!(
            !Color {
                red: 0.0,
                green: 0.0,
                blue: 0.0,
                alpha: -INF
            }.is_valid()
        );
        assert!(
            !Color {
                red: -INF,
                green: -INF,
                blue: -INF,
                alpha: -INF
            }.is_valid()
        );

        // Out of valid range
        assert!(
            !Color {
                red: -EPSILON,
                green: 0.0,
                blue: 0.0,
                alpha: 0.0
            }.is_valid()
        );
        assert!(
            !Color {
                red: 0.0,
                green: -EPSILON,
                blue: 0.0,
                alpha: 0.0
            }.is_valid()
        );
        assert!(
            !Color {
                red: 0.0,
                green: 0.0,
                blue: -EPSILON,
                alpha: 0.0
            }.is_valid()
        );
        assert!(
            !Color {
                red: 0.0,
                green: 0.0,
                blue: 0.0,
                alpha: -EPSILON
            }.is_valid()
        );
        assert!(
            !Color {
                red: -EPSILON,
                green: -EPSILON,
                blue: -EPSILON,
                alpha: -EPSILON
            }.is_valid()
        );

        assert!(
            !Color {
                red: 255.0001,
                green: 0.0,
                blue: 0.0,
                alpha: 0.0
            }.is_valid()
        );
        assert!(
            !Color {
                red: 0.0,
                green: 255.0001,
                blue: 0.0,
                alpha: 0.0
            }.is_valid()
        );
        assert!(
            !Color {
                red: 0.0,
                green: 0.0,
                blue: 255.0001,
                alpha: 0.0
            }.is_valid()
        );
        assert!(
            !Color {
                red: 0.0,
                green: 0.0,
                blue: 0.0,
                alpha: 1.0001
            }.is_valid()
        );
        assert!(
            !Color {
                red: 255.0001,
                green: 255.0001,
                blue: 255.0001,
                alpha: 1.0001
            }.is_valid()
        );
    }

    #[test]
    fn ensure_value_mapping() {
        let expected = Color {
            red: 65.,
            green: 122.,
            blue: 200.,
            alpha: 1.,
        };
        let actual_rgb = Color::rgb(65., 122., 200.);
        let actual_rgba = Color::rgba(65., 122., 200., 1.);
        assert_eq!(expected, actual_rgb);
        assert_eq!(expected, actual_rgba);
    }

    #[test]
    fn ensure_achromatic_hsl() {
        let expected = Color {
            red: 26.,
            green: 26.,
            blue: 26.,
            alpha: 1.,
        };
        let actual = Color::hsl(180., 0., 0.1);
        assert_eq!(expected, actual);
    }

    #[test]
    #[should_panic(expected = "-0.0000001 is not a valid value for red, values must be between 0.0 and 255.0")]
    fn ensure_rgb_invalid_red_negative_panic() {
        Color::rgb(-0.0000001, 20., 20.);
    }

    #[test]
    #[should_panic(expected = "255.0000001 is not a valid value for red, values must be between 0.0 and 255.0")]
    fn ensure_rgb_invalid_red_positive_panic() {
        Color::rgb(255.0000001, 20., 20.);
    }

    #[test]
    #[should_panic(expected = "-0.0000001 is not a valid value for green, values must be between 0.0 and 255.0")]
    fn ensure_rgb_invalid_green_negative_panic() {
        Color::rgb(20., -0.0000001, 20.);
    }

    #[test]
    #[should_panic(expected = "255.0000001 is not a valid value for green, values must be between 0.0 and 255.0")]
    fn ensure_rgb_invalid_green_positive_panic() {
        Color::rgb(20., 255.0000001, 20.);
    }

    #[test]
    #[should_panic(expected = "-0.0000001 is not a valid value for blue, values must be between 0.0 and 255.0")]
    fn ensure_rgb_invalid_blue_negative_panic() {
        Color::rgb(20., 20., -0.0000001);
    }

    #[test]
    #[should_panic(expected = "255.0000001 is not a valid value for blue, values must be between 0.0 and 255.0")]
    fn ensure_rgb_invalid_blue_positive_panic() {
        Color::rgb(20., 20., 255.0000001);
    }

    #[test]
    #[should_panic(expected = "-0.0000001 is not a valid value for red, values must be between 0.0 and 255.0")]
    fn ensure_rgba_invalid_red_negative_panic() {
        Color::rgba(-0.0000001, 20., 20., 1.);
    }

    #[test]
    #[should_panic(expected = "255.0000001 is not a valid value for red, values must be between 0.0 and 255.0")]
    fn ensure_rgba_invalid_red_positive_panic() {
        Color::rgba(255.0000001, 20., 20., 1.);
    }

    #[test]
    #[should_panic(expected = "-0.0000001 is not a valid value for green, values must be between 0.0 and 255.0")]
    fn ensure_rgba_invalid_green_negative_panic() {
        Color::rgba(20., -0.0000001, 20., 1.);
    }

    #[test]
    #[should_panic(expected = "255.0000001 is not a valid value for green, values must be between 0.0 and 255.0")]
    fn ensure_rgba_invalid_green_positive_panic() {
        Color::rgba(20., 255.0000001, 20., 1.);
    }

    #[test]
    #[should_panic(expected = "-0.0000001 is not a valid value for blue, values must be between 0.0 and 255.0")]
    fn ensure_rgba_invalid_blue_negative_panic() {
        Color::rgba(20., 20., -0.0000001, 1.);
    }

    #[test]
    #[should_panic(expected = "255.0000001 is not a valid value for blue, values must be between 0.0 and 255.0")]
    fn ensure_rgba_invalid_blue_positive_panic() {
        Color::rgba(20., 20., 255.0000001, 1.);
    }

    #[test]
    #[should_panic(expected = "-0.0000001 is not a valid value for alpha, values must be between 0.0 and 1.0")]
    fn ensure_rgba_invalid_alpha_negative_panic() {
        Color::rgba(20., 20., 20., -0.0000001);
    }

    #[test]
    #[should_panic(expected = "1.0000001 is not a valid value for alpha, values must be between 0.0 and 1.0")]
    fn ensure_rgba_invalid_alpha_positive_panic() {
        Color::rgba(20., 20., 20., 1.0000001);
    }

    #[test]
    #[should_panic(expected = "-0.0000001 is not a valid value for hue, values must be between 0.0 and 360.0")]
    fn ensure_hsl_invalid_hue_negative_panic() {
        Color::hsl(-0.0000001, 1., 1.);
    }

    #[test]
    #[should_panic(expected = "360.0000001 is not a valid value for hue, values must be between 0.0 and 360.0")]
    fn ensure_hsl_invalid_hue_positive_panic() {
        Color::hsl(360.0000001, 1., 1.);
    }

    #[test]
    #[should_panic(expected = "-0.0000001 is not a valid value for saturation, values must be between 0.0 and 1.0")]
    fn ensure_hsl_invalid_saturation_negative_panic() {
        Color::hsl(20., -0.0000001, 1.);
    }

    #[test]
    #[should_panic(expected = "1.0000001 is not a valid value for saturation, values must be between 0.0 and 1.0")]
    fn ensure_hsl_invalid_saturation_positive_panic() {
        Color::hsl(20., 1.0000001, 1.);
    }

    #[test]
    #[should_panic(expected = "-0.0000001 is not a valid value for lightness, values must be between 0.0 and 1.0")]
    fn ensure_hsl_invalid_lightness_negative_panic() {
        Color::hsl(20., 1., -0.0000001);
    }

    #[test]
    #[should_panic(expected = "1.0000001 is not a valid value for lightness, values must be between 0.0 and 1.0")]
    fn ensure_hsl_invalid_lightness_positive_panic() {
        Color::hsl(20., 1., 1.0000001);
    }

    #[test]
    #[should_panic(expected = "-0.0000001 is not a valid value for hue, values must be between 0.0 and 360.0")]
    fn ensure_hsla_invalid_hue_negative_panic() {
        Color::hsla(-0.0000001, 1., 1., 1.);
    }

    #[test]
    #[should_panic(expected = "360.0000001 is not a valid value for hue, values must be between 0.0 and 360.0")]
    fn ensure_hsla_invalid_hue_positive_panic() {
        Color::hsla(360.0000001, 1., 1., 1.);
    }

    #[test]
    #[should_panic(expected = "-0.0000001 is not a valid value for saturation, values must be between 0.0 and 1.0")]
    fn ensure_hsla_invalid_saturation_negative_panic() {
        Color::hsla(20., -0.0000001, 1., 1.);
    }

    #[test]
    #[should_panic(expected = "1.0000001 is not a valid value for saturation, values must be between 0.0 and 1.0")]
    fn ensure_hsla_invalid_saturation_positive_panic() {
        Color::hsla(20., 1.0000001, 1., 1.);
    }

    #[test]
    #[should_panic(expected = "-0.0000001 is not a valid value for lightness, values must be between 0.0 and 1.0")]
    fn ensure_hsla_invalid_lightness_negative_panic() {
        Color::hsla(20., 1., -0.0000001, 1.);
    }

    #[test]
    #[should_panic(expected = "1.0000001 is not a valid value for lightness, values must be between 0.0 and 1.0")]
    fn ensure_hsla_invalid_lightness_positive_panic() {
        Color::hsla(20., 1., 1.0000001, 1.);
    }

    #[test]
    #[should_panic(expected = "-0.0000001 is not a valid value for alpha, values must be between 0.0 and 1.0")]
    fn ensure_hsla_invalid_alpha_negative_panic() {
        Color::hsla(20., 1., 1., -0.0000001);
    }

    #[test]
    #[should_panic(expected = "1.0000001 is not a valid value for alpha, values must be between 0.0 and 1.0")]
    fn ensure_hsla_invalid_alpha_positive_panic() {
        Color::hsla(20., 1., 1., 1.0000001);
    }

    #[test]
    #[should_panic(expected = "-0.0000001 is not a valid value for red, values must be between 0.0 and 255.0")]
    fn ensure_rgb_arr_invalid_red_negative_panic() {
        let _: Color = [-0.0000001, 20., 20.].into();
    }

    #[test]
    #[should_panic(expected = "255.0000001 is not a valid value for red, values must be between 0.0 and 255.0")]
    fn ensure_rgb_arr_invalid_red_positive_panic() {
        let _: Color = [255.0000001, 20., 20.].into();
    }

    #[test]
    #[should_panic(expected = "-0.0000001 is not a valid value for green, values must be between 0.0 and 255.0")]
    fn ensure_rgb_arr_invalid_green_negative_panic() {
        let _: Color = [20., -0.0000001, 20.].into();
    }

    #[test]
    #[should_panic(expected = "255.0000001 is not a valid value for green, values must be between 0.0 and 255.0")]
    fn ensure_rgb_arr_invalid_green_positive_panic() {
        let _: Color = [20., 255.0000001, 20.].into();
    }

    #[test]
    #[should_panic(expected = "-0.0000001 is not a valid value for blue, values must be between 0.0 and 255.0")]
    fn ensure_rgb_arr_invalid_blue_negative_panic() {
        let _: Color = [20., 20., -0.0000001].into();
    }

    #[test]
    #[should_panic(expected = "255.0000001 is not a valid value for blue, values must be between 0.0 and 255.0")]
    fn ensure_rgb_arr_invalid_blue_positive_panic() {
        let _: Color = [20., 20., 255.0000001].into();
    }

    #[test]
    #[should_panic(expected = "-0.0000001 is not a valid value for red, values must be between 0.0 and 255.0")]
    fn ensure_rgba_arr_invalid_red_negative_panic() {
        let _: Color = [-0.0000001, 20., 20., 1.].into();
    }

    #[test]
    #[should_panic(expected = "255.0000001 is not a valid value for red, values must be between 0.0 and 255.0")]
    fn ensure_rgba_arr_invalid_red_positive_panic() {
        let _: Color = [255.0000001, 20., 20., 1.].into();
    }

    #[test]
    #[should_panic(expected = "-0.0000001 is not a valid value for green, values must be between 0.0 and 255.0")]
    fn ensure_rgba_arr_invalid_green_negative_panic() {
        let _: Color = [20., -0.0000001, 20., 1.].into();
    }

    #[test]
    #[should_panic(expected = "255.0000001 is not a valid value for green, values must be between 0.0 and 255.0")]
    fn ensure_rgba_arr_invalid_green_positive_panic() {
        let _: Color = [20., 255.0000001, 20., 1.].into();
    }

    #[test]
    #[should_panic(expected = "-0.0000001 is not a valid value for blue, values must be between 0.0 and 255.0")]
    fn ensure_rgba_arr_invalid_blue_negative_panic() {
        let _: Color = [20., 20., -0.0000001, 1.].into();
    }

    #[test]
    #[should_panic(expected = "255.0000001 is not a valid value for blue, values must be between 0.0 and 255.0")]
    fn ensure_rgba_arr_invalid_blue_positive_panic() {
        let _: Color = [20., 20., 255.0000001, 1.].into();
    }

    #[test]
    #[should_panic(expected = "-0.0000001 is not a valid value for alpha, values must be between 0.0 and 1.0")]
    fn ensure_rgba_arr_invalid_alpha_negative_panic() {
        let _: Color = [20., 20., 20., -0.0000001].into();
    }

    #[test]
    #[should_panic(expected = "1.0000001 is not a valid value for alpha, values must be between 0.0 and 1.0")]
    fn ensure_rgba_arr_invalid_alpha_positive_panic() {
        let _: Color = [20., 20., 20., 1.0000001].into();
    }

    #[test]
    #[should_panic(expected = "-360.0000001 is not a valid value for hue, values must be between -360.0 and 360.0.")]
    fn ensure_rotate_hue_invalid_negative_panic() {
        let c: Color = "blue".into();
        let _ = c.rotate_hue(-360.0000001);
    }

    #[test]
    #[should_panic(expected = "360.0000001 is not a valid value for hue, values must be between -360.0 and 360.0.")]
    fn ensure_rotate_hue_invalid_positive_panic() {
        let c: Color = "blue".into();
        let _ = c.rotate_hue(360.0000001);
    }

    #[test]
    #[should_panic(
        expected = "Color { red: 256.0, green: 1.0, blue: 1.0, alpha: 1.0 } is not a valid Color. Please see color module documentation."
    )]
    fn ensure_mix_invalid_color_panic() {
        let c: Color = "green".into();
        let invalid_color = Color {
            red: 256.,
            green: 1.,
            blue: 1.,
            alpha: 1.,
        };
        let _ = c.mix(invalid_color, 0.50);
    }

    #[test]
    #[should_panic(expected = "1.0000001 is not a valid value for weight, values must be between 0.0 and 1.0.")]
    fn ensure_mix_invalid_weight_panic() {
        let o: Color = "orange".into();
        let r: Color = "red".into();
        let _ = o.mix(r, 1.0000001);
    }

    #[test]
    #[should_panic(expected = "-0.0000001 is not a valid value for weight, values must be between 0.0 and 1.0.")]
    fn ensure_mix_negative_weight_panic() {
        let o: Color = "orange".into();
        let r: Color = "red".into();
        let _ = o.mix(r, -0.0000001);
    }

    #[test]
    fn check_rgb_values() {
        rgb_mapping_values()
            .iter()
            .for_each(|&(expected, (r, g, b))| assert_eq!(Color::from(expected), Color::rgb(r, g, b)));
    }

    #[test]
    fn check_rgba_values() {
        rgb_mapping_values()
            .iter()
            .for_each(|&(expected, (r, g, b))| assert_eq!(Color::from(expected), Color::rgba(r, g, b, 1.0)));
    }

    #[test]
    fn check_hsl_values() {
        hsl_mapping_values()
            .iter()
            .for_each(|&((r, g, b), (h, s, l))| assert_eq!(Color::rgb(r, g, b), Color::hsl(h, s, l)));
    }

    #[test]
    fn check_hsla_values() {
        hsl_mapping_values()
            .iter()
            .for_each(|&((r, g, b), (h, s, l))| assert_eq!(Color::rgba(r, g, b, 1.0), Color::hsla(h, s, l, 1.0)));
    }

    /// Some mappings from color name to rgb values to pass through the rgb(a) constructor methods
    fn rgb_mapping_values() -> Vec<(&'static str, (f64, f64, f64))> {
        vec![
            ("red", (230.0, 25.0, 75.0)),
            ("green", (60.0, 180.0, 75.0)),
            ("yellow", (255.0, 225.0, 25.0)),
            ("blue", (0.0, 130.0, 200.0)),
            ("orange", (245.0, 130.0, 48.0)),
            ("purple", (145.0, 30.0, 180.0)),
            ("cyan", (70.0, 240.0, 240.0)),
            ("magenta", (240.0, 50.0, 230.0)),
            ("lime", (210.0, 245.0, 60.0)),
            ("pink", (250.0, 190.0, 190.0)),
            ("teal", (0.0, 128.0, 128.0)),
            ("lavender", (230.0, 190.0, 255.0)),
            ("brown", (170.0, 110.0, 40.0)),
            ("beige", (255.0, 250.0, 200.0)),
            ("maroon", (128.0, 0.0, 0.0)),
            ("mint", (170.0, 255.0, 195.0)),
            ("olive", (128.0, 128.0, 0.0)),
            ("coral", (255.0, 215.0, 180.0)),
            ("navy", (0.0, 0.0, 128.0)),
            ("grey", (128.0, 128.0, 128.0)),
            ("white", (255.0, 255.0, 255.0)),
            ("black", (0.0, 0.0, 0.0)),
        ]
    }

    /// Some mappings from rgb values to hsl to pass into the hsl(a) constructor methods.
    /// N.B. There is some wiggle for HSL -> RGB on some values, so these colors end up not being
    /// exact as if rgb values were given.
    ///
    /// This can be seen by going to
    /// https://www.rapidtables.com/convert/color/rgb-to-hsl.html and entering (for rgb)
    /// 230, 25, 75 (turtle Red) and getting the HSL (345, 80.4, 50.0) then going to
    /// https://www.rapidtables.com/convert/color/hsl-to-rgb.html and entering (for hsl)
    /// 345, 80.4, 50.0 and noting that the returned RGB is (230, 25, 76)
    fn hsl_mapping_values() -> Vec<((f64, f64, f64), (f64, f64, f64))> {
        // All hsl colors converted by hand at https://www.rapidtables.com/convert/color/rgb-to-hsl.html
        vec![
            ((230.0, 25.0, 76.0), (345.0, 0.804, 0.5)),
            ((60.0, 180.0, 76.0), (128.0, 0.5, 0.471)),
            ((255.0, 224.0, 25.0), (52.0, 1.0, 0.549)),
            ((0.0, 130.0, 200.0), (201.0, 1.0, 0.392)),
            ((245.0, 130.0, 48.0), (25.0, 0.908, 0.575)),
            ((145.0, 30.0, 180.0), (286.0, 0.714, 0.412)),
            ((70.0, 240.0, 240.0), (180.0, 0.85, 0.608)),
            ((240.0, 50.0, 231.0), (303.0, 0.864, 0.569)),
            ((211.0, 245.0, 60.0), (71.0, 0.902, 0.598)),
            ((250.0, 190.0, 190.0), (0.0, 0.857, 0.863)),
            ((0.0, 128.0, 128.0), (180.0, 1.0, 0.251)),
            ((230.0, 190.0, 255.0), (277.0, 1.0, 0.873)),
            ((170.0, 109.0, 40.0), (32.0, 0.619, 0.412)),
            ((255.0, 250.0, 200.0), (55.0, 1.0, 0.892)),
            ((128.0, 0.0, 0.0), (0.0, 1.0, 0.251)),
            ((170.0, 255.0, 195.0), (138.0, 1.0, 0.833)),
            ((128.0, 128.0, 0.0), (60.0, 1.0, 0.251)),
            ((255.0, 215.0, 180.0), (28.0, 1.0, 0.853)),
            ((0.0, 0.0, 128.0), (240.0, 1.0, 0.251)),
            ((128.0, 128.0, 128.0), (0.0, 0.0, 0.502)),
            ((255.0, 255.0, 255.0), (0.0, 0.0, 1.0)),
            ((0.0, 0.0, 0.0), (0.0, 0.0, 0.0)),
        ]
    }

    #[test]
    fn ensure_color_mix() {
        let mix_1 = Color::rgba(18., 55., 125., 1.0);
        let mix_2 = Color::rgba(125., 33., 200., 0.8);
        let expected = Color::rgba(72., 44., 163., 0.88);
        let mix_res = mix_1.mix(mix_2, 0.40);
        assert_eq!(expected, mix_res);
    }

    #[test]
    fn check_rgb_to_hsl() {
        let c = Color::rgb(251., 206., 33.);
        let result = c.to_hsl();

        assert_eq!((48., 0.9646017699115044, 0.5568627450980392), result);
    }

    #[test]
    fn check_hsl_getters() {
        let c = Color::hsl(345.0, 0.804, 0.5);
        // Check hue
        assert_eq!(345., c.hue());
        // Check saturation, rounded to account floats
        assert_eq!(804., (c.saturation() * 1000.).round());
        // Check lightness
        assert_eq!(500., (c.lightness() * 1000.).round());
    }

    #[test]
    fn check_rotate_hue_positive() {
        let c = Color::hsl(25.0, 0.908, 0.575);
        assert_eq!(c.rotate_hue(70.), Color::hsl(95., 0.908, 0.575));
    }

    #[test]
    fn check_rotate_hue_negative() {
        let c = Color::hsl(25.0, 0.908, 0.575);
        assert_eq!(c.rotate_hue(-10.), Color::hsl(15., 0.908, 0.575));
    }

    #[test]
    fn check_rotate_hue_wrap_positive() {
        let c = Color::hsl(350., 0.5, 0.75);
        assert_eq!(c.rotate_hue(50.), Color::hsl(40., 0.5, 0.75));
    }

    #[test]
    fn check_rotate_hue_wrap_negative() {
        let c = Color::hsl(10., 0.8, 0.9);
        assert_eq!(c.rotate_hue(-50.), Color::hsl(320., 0.8, 0.9));
    }

    #[test]
    fn check_rotate_hue_keeps_alpha() {
        let c = Color::hsla(25.0, 0.908, 0.575, 0.3);
        assert_eq!(c.rotate_hue(70.), Color::hsla(95., 0.908, 0.575, 0.3));
    }

    #[test]
    fn check_lighten_positive() {
        let c = Color::hsl(200., 0.8, 0.4);
        assert_eq!(c.lighten(0.2), Color::hsl(200., 0.8, 0.6));
    }

    #[test]
    fn check_lighten_stops_at_max() {
        let c = Color::hsl(300., 0.5, 0.8);
        assert_eq!(c.lighten(0.5), Color::hsl(300., 0.5, 1.0));
    }

    #[test]
    fn check_lighten_keeps_alpha() {
        let c = Color::hsla(200., 1.0, 0.4, 0.3);
        assert_eq!(c.lighten(0.4), Color::hsla(200., 1.0, 0.8, 0.3));
    }

    #[test]
    #[should_panic(expected = "1.1 is not a valid value for lighter, values must be between 0.0 and 1.0")]
    fn ensure_lighten_invalid_positive_panic() {
        let c = Color::hsl(150., 0.4, 0.2);
        let _ = c.lighten(1.1);
    }

    #[test]
    #[should_panic(expected = "-0.1 is not a valid value for lighter, values must be between 0.0 and 1.0")]
    fn ensure_lighten_invalid_negative_panic() {
        let c = Color::hsl(150., 0.4, 0.2);
        let _ = c.lighten(-0.1);
    }

    #[test]
    fn check_darken_positive() {
        let c = Color::hsl(50., 1.0, 0.8);
        assert_eq!(c.darken(0.2), Color::hsl(50., 1.0, 0.6));
    }

    #[test]
    fn check_darken_stops_at_min() {
        let c = Color::hsl(80., 0.9, 0.4);
        assert_eq!(c.darken(0.6), Color::hsl(80., 0.9, 0.0));
    }

    #[test]
    fn check_darken_keeps_alpha() {
        let c = Color::hsla(50., 1.0, 0.8, 0.4);
        assert_eq!(c.darken(0.2), Color::hsla(50., 1.0, 0.6, 0.4));
    }

    #[test]
    #[should_panic(expected = "1.1 is not a valid value for darker, values must be between 0.0 and 1.0")]
    fn ensure_darken_invalid_positive_panic() {
        let c = Color::hsl(150., 0.4, 0.2);
        let _ = c.darken(1.1);
    }

    #[test]
    #[should_panic(expected = "-0.1 is not a valid value for darker, values must be between 0.0 and 1.0")]
    fn ensure_darken_invalid_negative_panic() {
        let c = Color::hsl(150., 0.4, 0.2);
        let _ = c.darken(-0.1);
    }

    #[test]
    fn check_saturate_positive() {
        let c = Color::hsl(210., 0.3, 0.9);
        assert_eq!(c.saturate(0.2), Color::hsl(210., 0.5, 0.9));
    }

    #[test]
    fn check_saturate_stops_at_max() {
        let c = Color::hsl(120., 0.3, 0.9);
        assert_eq!(c.saturate(0.9), Color::hsl(120., 1.0, 0.9));
    }

    #[test]
    fn check_saturate_keeps_alpha() {
        let c = Color::hsla(120., 0.3, 0.9, 0.4);
        assert_eq!(c.saturate(0.2), Color::hsla(120., 0.5, 0.9, 0.4));
    }

    #[test]
    #[should_panic(expected = "1.1 is not a valid value for saturation, values must be between 0.0 and 1.0")]
    fn ensure_saturation_invalid_positive_panic() {
        let c = Color::hsl(210., 0.5, 0.9);
        let _ = c.saturate(1.1);
    }

    #[test]
    #[should_panic(expected = "-0.1 is not a valid value for saturation, values must be between 0.0 and 1.0")]
    fn ensure_saturation_invalid_negative_panic() {
        let c = Color::hsl(210., 0.5, 0.9);
        let _ = c.saturate(-0.1);
    }

    #[test]
    fn check_desaturate_positive() {
        let c = Color::hsl(210., 0.3, 0.9);
        assert_eq!(c.desaturate(0.2), Color::hsl(210., 0.1, 0.9));
    }

    #[test]
    fn check_desaturate_stops_at_max() {
        let c = Color::hsl(120., 0.3, 0.9);
        assert_eq!(c.desaturate(0.9), Color::hsl(120., 0.0, 0.9));
    }

    #[test]
    fn check_desaturate_keeps_alpha() {
        let c = Color::hsla(120., 0.3, 0.9, 0.4);
        assert_eq!(c.desaturate(0.2), Color::hsla(120., 0.1, 0.9, 0.4));
    }

    #[test]
    #[should_panic(expected = "1.1 is not a valid value for saturation, values must be between 0.0 and 1.0")]
    fn ensure_desaturate_invalid_positive_panic() {
        let c = Color::hsl(210., 0.5, 0.9);
        let _ = c.desaturate(1.1);
    }

    #[test]
    #[should_panic(expected = "-0.1 is not a valid value for saturation, values must be between 0.0 and 1.0")]
    fn ensure_desaturate_invalid_negative_panic() {
        let c = Color::hsl(210., 0.5, 0.9);
        let _ = c.desaturate(-0.1);
    }

    #[test]
    fn check_grayscale() {
        let c = Color::hsl(180., 0.9, 0.5);
        assert_eq!(c.grayscale(), Color::hsl(180., 0.0, 0.5));
    }

    #[test]
    fn check_grayscale_keeps_alpha() {
        let c = Color::hsla(200., 0.5, 0.5, 0.3);
        assert_eq!(c.grayscale(), Color::hsla(200., 0.0, 0.5, 0.3));
    }

    #[test]
    #[should_panic(
        expected = "Color { red: 255.0, green: 256.0, blue: 255.0, alpha: 1.0 } is not a valid Color. Please see color module documentation."
    )]
    fn ensure_grayscale_invalid_color_panics() {
        let c = Color {
            red: 255.,
            green: 256.,
            blue: 255.,
            alpha: 1.0,
        };
        let _ = c.grayscale();
    }

    #[test]
    fn check_complement() {
        let c = Color::hsl(100., 0.5, 0.5);
        assert_eq!(c.complement(), Color::hsl(280., 0.5, 0.5));
    }

    #[test]
    fn check_complement_wraps() {
        let c = Color::hsl(300., 0.5, 0.5);
        assert_eq!(c.complement(), Color::hsl(120., 0.5, 0.5));
    }

    #[test]
    fn check_complement_keeps_alpha() {
        let c = Color::hsla(80., 0.5, 0.5, 0.4);
        assert_eq!(c.complement(), Color::hsla(260., 0.5, 0.5, 0.4))
    }

    #[test]
    #[should_panic(
        expected = "Color { red: 255.0, green: 256.0, blue: 255.0, alpha: 1.0 } is not a valid Color. Please see color module documentation."
    )]
    fn ensure_complement_invalid_color_panics() {
        let c = Color {
            red: 255.,
            green: 256.,
            blue: 255.,
            alpha: 1.0,
        };
        let _ = c.complement();
    }

    #[test]
    fn check_invert() {
        let c: Color = "red".into();
        assert_eq!(c.invert(), [25., 230., 180.].into());
    }

    #[test]
    fn check_invert_keeps_alpha() {
        let c: Color = [100., 250., 175., 0.2].into();
        assert_eq!(c.invert(), [155., 5., 80., 0.2].into());
    }

    #[test]
    #[should_panic(
        expected = "Color { red: 255.0, green: 256.0, blue: 255.0, alpha: 1.0 } is not a valid Color. Please see color module documentation."
    )]
    fn ensure_invert_invalid_color_panics() {
        let c = Color {
            red: 255.,
            green: 256.,
            blue: 255.,
            alpha: 1.0,
        };
        let _ = c.invert();
    }

    #[test]
    #[should_panic(
        expected = "Color { red: 255.0, green: 256.0, blue: 255.0, alpha: 1.0 } is not a valid Color. Please see color module documentation."
    )]
    fn ensure_hue_invalid_color_panics() {
        let c = Color {
            red: 255.,
            green: 256.,
            blue: 255.,
            alpha: 1.0,
        };
        let _ = c.hue();
    }

    #[test]
    #[should_panic(
        expected = "Color { red: 255.0, green: 256.0, blue: 255.0, alpha: 1.0 } is not a valid Color. Please see color module documentation."
    )]
    fn ensure_saturation_invalid_color_panics() {
        let c = Color {
            red: 255.,
            green: 256.,
            blue: 255.,
            alpha: 1.0,
        };
        let _ = c.saturation();
    }

    #[test]
    #[should_panic(
        expected = "Color { red: 255.0, green: 256.0, blue: 255.0, alpha: 1.0 } is not a valid Color. Please see color module documentation."
    )]
    fn ensure_lightness_invalid_color_panics() {
        let c = Color {
            red: 255.,
            green: 256.,
            blue: 255.,
            alpha: 1.0,
        };
        let _ = c.lightness();
    }

    #[test]
    #[should_panic(
        expected = "Color { red: 255.0, green: 256.0, blue: 255.0, alpha: 1.0 } is not a valid Color. Please see color module documentation."
    )]
    fn ensure_saturate_invalid_color_panics() {
        let c = Color {
            red: 255.,
            green: 256.,
            blue: 255.,
            alpha: 1.0,
        };
        let _ = c.saturate(0.5);
    }

    #[test]
    #[should_panic(
        expected = "Color { red: 255.0, green: 256.0, blue: 255.0, alpha: 1.0 } is not a valid Color. Please see color module documentation."
    )]
    fn ensure_desaturate_invalid_color_panics() {
        let c = Color {
            red: 255.,
            green: 256.,
            blue: 255.,
            alpha: 1.0,
        };
        let _ = c.desaturate(0.5);
    }

    #[test]
    #[should_panic(
        expected = "Color { red: 255.0, green: 256.0, blue: 255.0, alpha: 1.0 } is not a valid Color. Please see color module documentation."
    )]
    fn ensure_darken_invalid_color_panics() {
        let c = Color {
            red: 255.,
            green: 256.,
            blue: 255.,
            alpha: 1.0,
        };
        let _ = c.darken(0.5);
    }

    #[test]
    #[should_panic(
        expected = "Color { red: 255.0, green: 256.0, blue: 255.0, alpha: 1.0 } is not a valid Color. Please see color module documentation."
    )]
    fn ensure_lighten_invalid_color_panics() {
        let c = Color {
            red: 255.,
            green: 256.,
            blue: 255.,
            alpha: 1.0,
        };
        let _ = c.lighten(0.5);
    }

    #[test]
    #[should_panic(
        expected = "Color { red: 255.0, green: 256.0, blue: 255.0, alpha: 1.0 } is not a valid Color. Please see color module documentation."
    )]
    fn ensure_rotate_hue_invalid_color_panics() {
        let c = Color {
            red: 255.,
            green: 256.,
            blue: 255.,
            alpha: 1.0,
        };
        let _ = c.rotate_hue(20.);
    }
}
