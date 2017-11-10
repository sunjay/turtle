//! Color types and constants
//!
//! When setting a color, you can use a variety of different color names.
//! This module contains many of the most common colors that you might want to use. There is an
//! even more comprehensive list in the [`extended`](extended/index.html) module. Any of the color names
//! listed in this module or in the `extended` module can be used as a color. You only need to
//! reference the `color::extended` module if you want to use a specific color constant from that
//! module.
//!
//! You can refer to a color by using its color name as a string literal. For example:
//!
//! ```rust
//! # extern crate turtle;
//! # use turtle::color;
//! # fn main() {
//! # let mut turtle = turtle::Turtle::new();
//! // This will set the turtle's pen color to BLACK
//! turtle.set_pen_color("black");
//! // This is the same as the previous line
//! turtle.set_pen_color(color::BLACK);
//! // You can use any of the supported color names (including the ones from extended)
//! turtle.set_pen_color("deep lilac");
//! # }
//! ```
//!
//! You can also use hexadecimal color strings to get any color you want
//! (even ones that aren't listed here).
//!
//! ```rust
//! # extern crate turtle;
//! # fn main() {
//! # let mut turtle = turtle::Turtle::new();
//! turtle.set_pen_color("#3366ff");
//! turtle.set_pen_color("#36f");
//! # }
//! ```
//!
//! Each color's constant name is in uppercase in the list below. The color name you should use to
//! refer to it is in lower case next to the constant.
//!
//! For your convenience, there are two static variables [`COLORS`](static.COLORS.html) and
//! [`COLOR_NAMES`](static.COLOR_NAMES.html) which contain the values of all the color constants
//! and each of their names as strings. These static variables only contain the colors from this
//! module. The [`extended`](extended/index.html) module has its own static `COLOR` and
//! `COLOR_NAMES` variables.
//!
//! ## Random Colors
//!
//! You can also generate random colors. Here's an example:
//!
//! ```rust
//! # extern crate turtle;
//! # fn main() {
//! # let mut turtle = turtle::Turtle::new();
//! use turtle::{random, Color};
//! turtle.set_pen_color(random::<Color>().opaque());
//! # }
//! ```
//!
//! The syntax used in `random::<Color>()` is referred to as
//! ["turbofish" syntax](https://doc.rust-lang.org/book/first-edition/generics.html#resolving-ambiguities).
//! See that documentation for more information.
//!
//! Notice that you need to explicitly call the [`opaque()`](struct.Color.html#method.opaque)
//! method on the color in order to make sure that the color has an alpha value of 1.0. By default,
//! when you generate a random color, it's alpha value will be random as well.
//!
//! See the [examples directory on GitHub](https://github.com/sunjay/turtle/tree/master/examples)
//! for more information.
//!
//! ## Creating a Color from Values
//!
//! Usually, you won't need to initialize a color this way since the above methods are quite
//! convenient. However, in some situations it may be necessary to create a color with specific
//! red, green, and blue values. The following example illustrates how to do that.
//!
//! ```rust
//! # extern crate turtle;
//! # fn main() {
//! use turtle::Color;
//! let my_color = Color {red: 255.0, green: 55.0, blue: 11.0, alpha: 1.0};
//! # }
//! ```
//!
//! Note that when creating a color this way, we **do not** check if the values of each property are
//! within their valid ranges.

use std::iter::repeat;

use piston_window::types;
use rand::{Rand, Rng};

/// Type for representing a color.
///
/// See [the module level documentation](index.html) for more.
#[derive(Debug, Clone, Copy, PartialEq)]
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
    /// Return a new color with all of the same values except with opacity (alpha) set to 1.0
    ///
    /// ```rust
    /// # extern crate turtle;
    /// # fn main() {
    /// use turtle::Color;
    /// let color = Color {red: 43.0, green: 79.0, blue: 23.0, alpha: 0.5};
    /// assert_eq!(color.alpha, 0.5);
    /// let color2 = color.opaque();
    /// assert_eq!(color.alpha, 0.5);
    /// assert_eq!(color2.alpha, 1.0);
    /// # }
    /// ```
    pub fn opaque(mut self) -> Color {
        self.alpha = 1.0;
        self
    }

    /// Return a new color with all of the same values except with opacity (alpha) set to 0.0
    ///
    /// ```rust
    /// # extern crate turtle;
    /// # fn main() {
    /// use turtle::Color;
    /// let color = Color {red: 43.0, green: 79.0, blue: 23.0, alpha: 0.5};
    /// assert_eq!(color.alpha, 0.5);
    /// let color2 = color.transparent();
    /// assert_eq!(color.alpha, 0.5);
    /// assert_eq!(color2.alpha, 0.0);
    /// # }
    /// ```
    pub fn transparent(mut self) -> Color {
        self.alpha = 0.0;
        self
    }
}

impl Rand for Color {
    fn rand<R: Rng>(rng: &mut R) -> Self {
        Self {
            red: rng.gen::<f64>() * 255.0,
            green: rng.gen::<f64>() * 255.0,
            blue: rng.gen::<f64>() * 255.0,
            alpha: rng.gen::<f64>(),
        }
    }
}

impl From<Color> for types::Color {
    fn from(color: Color) -> Self {
        [
            color.red as f32 / 255.0,
            color.green as f32 / 255.0,
            color.blue as f32 / 255.0,
            color.alpha as f32,
        ]
    }
}

impl<'a> From<&'a str> for Color {
    fn from(s: &'a str) -> Self {
        match s {
            s if s.starts_with('#') => {
                let color_str = &s[1..];
                // Color strings can either be of size 3 (rgb) or 6 (rrggbb)
                // e.g. 3366ff == 36f
                let color_str = match color_str.len() {
                    3 => color_str.chars().flat_map(|c| repeat(c).take(2)).collect(),
                    6 => color_str.to_owned(),
                    _ => panic!("Invalid color literal: {}", s),
                };

                let red = i64::from_str_radix(&color_str[0..2], 16);
                let green = i64::from_str_radix(&color_str[2..4], 16);
                let blue = i64::from_str_radix(&color_str[4..6], 16);

                Color {
                    red: red.expect(&format!("Invalid color literal: {}", s)) as f64,
                    green: green.expect(&format!("Invalid color literal: {}", s)) as f64,
                    blue: blue.expect(&format!("Invalid color literal: {}", s)) as f64,
                    alpha: 1.0,
                }
            },
            s => {
                from_color_name(s)
                    .or_else(|| extended::from_color_name(s))
                    .expect(&format!("Unknown color name: {}", s))
            },
        }
    }
}

macro_rules! color_consts {
    ($($name:expr, $id:ident, ($r:expr, $g:expr, $b:expr, $a:expr);)*) => {
        $(
            #[doc = $name]
            pub const $id: Color = Color {red: $r, green: $g, blue: $b, alpha: $a};
        )*

        /// A list of all of the colors
        pub static COLORS: &[Color] = &[$($id, )*];
        /// A list of all of the color names
        pub static COLOR_NAMES: &[&str] = &[$($name, )*];

        pub(crate) fn from_color_name(s: &str) -> Option<Color> {
            match s {
                $(
                    $name => Some($id),
                )*
                _ => None,
            }
        }
    }
}

// Most important colors are put in the main module, the remaining are in extended.
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

    #[test]
    fn color_equivalence() {
        let c = Color {red: 51.0, green: 85.0, blue: 255.0, alpha: 1.0};
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
}

pub mod extended {
    //! Even more colors!!
    //!
    //! This extended list of colors is from: <https://xkcd.com/color/rgb/>
    //!
    //! Each color's constant name is in uppercase in the list below. The color name you should use to
    //! refer to it is in lower case next to the constant.
    //!
    //! For your convenience, there are two static variables [`COLORS`](static.COLORS.html) and
    //! [`COLOR_NAMES`](static.COLOR_NAMES.html) which contain the values of all the color constants
    //! and each of their names as strings. These static variables only contain the colors from this
    //! module.
    use super::Color;

    color_consts! {
        "grey", GREY, (146.0, 149.0, 145.0, 1.0);
        "sky blue", SKY_BLUE, (117.0, 187.0, 253.0, 1.0);
        "yellow", YELLOW, (255.0, 255.0, 20.0, 1.0);
        "magenta", MAGENTA, (194.0, 0.0, 120.0, 1.0);
        "light green", LIGHT_GREEN, (150.0, 249.0, 123.0, 1.0);
        "orange", ORANGE, (249.0, 115.0, 6.0, 1.0);
        "teal", TEAL, (2.0, 147.0, 134.0, 1.0);
        "light blue", LIGHT_BLUE, (149.0, 208.0, 252.0, 1.0);
        "red", RED, (229.0, 0.0, 0.0, 1.0);
        "brown", BROWN, (101.0, 55.0, 0.0, 1.0);
        "pink", PINK, (255.0, 129.0, 192.0, 1.0);
        "blue", BLUE, (3.0, 67.0, 223.0, 1.0);
        "green", GREEN, (21.0, 176.0, 26.0, 1.0);
        "purple", PURPLE, (126.0, 30.0, 156.0, 1.0);

        "cloudy blue", CLOUDY_BLUE, (172.0, 194.0, 217.0, 1.0);
        "dark pastel green", DARK_PASTEL_GREEN, (86.0, 174.0, 87.0, 1.0);
        "dust", DUST, (178.0, 153.0, 110.0, 1.0);
        "electric lime", ELECTRIC_LIME, (168.0, 255.0, 4.0, 1.0);
        "fresh green", FRESH_GREEN, (105.0, 216.0, 79.0, 1.0);
        "light eggplant", LIGHT_EGGPLANT, (137.0, 69.0, 133.0, 1.0);
        "nasty green", NASTY_GREEN, (112.0, 178.0, 63.0, 1.0);
        "really light blue", REALLY_LIGHT_BLUE, (212.0, 255.0, 255.0, 1.0);
        "tea", TEA, (101.0, 171.0, 124.0, 1.0);
        "warm purple", WARM_PURPLE, (149.0, 46.0, 143.0, 1.0);
        "yellowish tan", YELLOWISH_TAN, (252.0, 252.0, 129.0, 1.0);
        "cement", CEMENT, (165.0, 163.0, 145.0, 1.0);
        "dark grass green", DARK_GRASS_GREEN, (56.0, 128.0, 4.0, 1.0);
        "dusty teal", DUSTY_TEAL, (76.0, 144.0, 133.0, 1.0);
        "grey teal", GREY_TEAL, (94.0, 155.0, 138.0, 1.0);
        "macaroni and cheese", MACARONI_AND_CHEESE, (239.0, 180.0, 53.0, 1.0);
        "pinkish tan", PINKISH_TAN, (217.0, 155.0, 130.0, 1.0);
        "spruce", SPRUCE, (10.0, 95.0, 56.0, 1.0);
        "strong blue", STRONG_BLUE, (12.0, 6.0, 247.0, 1.0);
        "toxic green", TOXIC_GREEN, (97.0, 222.0, 42.0, 1.0);
        "windows blue", WINDOWS_BLUE, (55.0, 120.0, 191.0, 1.0);
        "blue blue", BLUE_BLUE, (34.0, 66.0, 199.0, 1.0);
        "blue with a hint of purple", BLUE_WITH_A_HINT_OF_PURPLE, (83.0, 60.0, 198.0, 1.0);
        "booger", BOOGER, (155.0, 181.0, 60.0, 1.0);
        "bright sea green", BRIGHT_SEA_GREEN, (5.0, 255.0, 166.0, 1.0);
        "dark green blue", DARK_GREEN_BLUE, (31.0, 99.0, 87.0, 1.0);
        "deep turquoise", DEEP_TURQUOISE, (1.0, 115.0, 116.0, 1.0);
        "green teal", GREEN_TEAL, (12.0, 181.0, 119.0, 1.0);
        "strong pink", STRONG_PINK, (255.0, 7.0, 137.0, 1.0);
        "bland", BLAND, (175.0, 168.0, 139.0, 1.0);
        "deep aqua", DEEP_AQUA, (8.0, 120.0, 127.0, 1.0);
        "lavender pink", LAVENDER_PINK, (221.0, 133.0, 215.0, 1.0);
        "light moss green", LIGHT_MOSS_GREEN, (166.0, 200.0, 117.0, 1.0);
        "light seafoam green", LIGHT_SEAFOAM_GREEN, (167.0, 255.0, 181.0, 1.0);
        "olive yellow", OLIVE_YELLOW, (194.0, 183.0, 9.0, 1.0);
        "pig pink", PIG_PINK, (231.0, 142.0, 165.0, 1.0);
        "deep lilac", DEEP_LILAC, (150.0, 110.0, 189.0, 1.0);
        "desert", DESERT, (204.0, 173.0, 96.0, 1.0);
        "dusty lavender", DUSTY_LAVENDER, (172.0, 134.0, 168.0, 1.0);
        "purpley grey", PURPLEY_GREY, (148.0, 126.0, 148.0, 1.0);
        "purply", PURPLY, (152.0, 63.0, 178.0, 1.0);
        "candy pink", CANDY_PINK, (255.0, 99.0, 233.0, 1.0);
        "light pastel green", LIGHT_PASTEL_GREEN, (178.0, 251.0, 165.0, 1.0);
        "boring green", BORING_GREEN, (99.0, 179.0, 101.0, 1.0);
        "kiwi green", KIWI_GREEN, (142.0, 229.0, 63.0, 1.0);
        "light grey green", LIGHT_GREY_GREEN, (183.0, 225.0, 161.0, 1.0);
        "orange pink", ORANGE_PINK, (255.0, 111.0, 82.0, 1.0);
        "tea green", TEA_GREEN, (189.0, 248.0, 163.0, 1.0);
        "very light brown", VERY_LIGHT_BROWN, (211.0, 182.0, 131.0, 1.0);
        "egg shell", EGG_SHELL, (255.0, 252.0, 196.0, 1.0);
        "eggplant purple", EGGPLANT_PURPLE, (67.0, 5.0, 65.0, 1.0);
        "powder pink", POWDER_PINK, (255.0, 178.0, 208.0, 1.0);
        "reddish grey", REDDISH_GREY, (153.0, 117.0, 112.0, 1.0);
        "baby shit brown", BABY_SHIT_BROWN, (173.0, 144.0, 13.0, 1.0);
        "liliac", LILIAC, (196.0, 142.0, 253.0, 1.0);
        "stormy blue", STORMY_BLUE, (80.0, 123.0, 156.0, 1.0);
        "ugly brown", UGLY_BROWN, (125.0, 113.0, 3.0, 1.0);
        "custard", CUSTARD, (255.0, 253.0, 120.0, 1.0);
        "darkish pink", DARKISH_PINK, (218.0, 70.0, 125.0, 1.0);
        "deep brown", DEEP_BROWN, (65.0, 2.0, 0.0, 1.0);
        "greenish beige", GREENISH_BEIGE, (201.0, 209.0, 121.0, 1.0);
        "manilla", MANILLA, (255.0, 250.0, 134.0, 1.0);
        "off blue", OFF_BLUE, (86.0, 132.0, 174.0, 1.0);
        "battleship grey", BATTLESHIP_GREY, (107.0, 124.0, 133.0, 1.0);
        "browny green", BROWNY_GREEN, (111.0, 108.0, 10.0, 1.0);
        "bruise", BRUISE, (126.0, 64.0, 113.0, 1.0);
        "kelley green", KELLEY_GREEN, (0.0, 147.0, 55.0, 1.0);
        "sickly yellow", SICKLY_YELLOW, (208.0, 228.0, 41.0, 1.0);
        "sunny yellow", SUNNY_YELLOW, (255.0, 249.0, 23.0, 1.0);
        "azul", AZUL, (29.0, 93.0, 236.0, 1.0);
        "darkgreen", DARKGREEN, (5.0, 73.0, 7.0, 1.0);
        "lichen", LICHEN, (143.0, 182.0, 123.0, 1.0);
        "light light green", LIGHT_LIGHT_GREEN, (200.0, 255.0, 176.0, 1.0);
        "pale gold", PALE_GOLD, (253.0, 222.0, 108.0, 1.0);
        "sun yellow", SUN_YELLOW, (255.0, 223.0, 34.0, 1.0);
        "tan green", TAN_GREEN, (169.0, 190.0, 112.0, 1.0);
        "burple", BURPLE, (104.0, 50.0, 227.0, 1.0);
        "butterscotch", BUTTERSCOTCH, (253.0, 177.0, 71.0, 1.0);
        "toupe", TOUPE, (199.0, 172.0, 125.0, 1.0);
        "dark cream", DARK_CREAM, (255.0, 243.0, 154.0, 1.0);
        "indian red", INDIAN_RED, (133.0, 14.0, 4.0, 1.0);
        "light lavendar", LIGHT_LAVENDAR, (239.0, 192.0, 254.0, 1.0);
        "poison green", POISON_GREEN, (64.0, 253.0, 20.0, 1.0);
        "baby puke green", BABY_PUKE_GREEN, (182.0, 196.0, 6.0, 1.0);
        "bright yellow green", BRIGHT_YELLOW_GREEN, (157.0, 255.0, 0.0, 1.0);
        "charcoal grey", CHARCOAL_GREY, (60.0, 65.0, 66.0, 1.0);
        "squash", SQUASH, (242.0, 171.0, 21.0, 1.0);
        "cinnamon", CINNAMON, (172.0, 79.0, 6.0, 1.0);
        "light pea green", LIGHT_PEA_GREEN, (196.0, 254.0, 130.0, 1.0);
        "radioactive green", RADIOACTIVE_GREEN, (44.0, 250.0, 31.0, 1.0);
        "raw sienna", RAW_SIENNA, (154.0, 98.0, 0.0, 1.0);
        "baby purple", BABY_PURPLE, (202.0, 155.0, 247.0, 1.0);
        "cocoa", COCOA, (135.0, 95.0, 66.0, 1.0);
        "light royal blue", LIGHT_ROYAL_BLUE, (58.0, 46.0, 254.0, 1.0);
        "orangeish", ORANGEISH, (253.0, 141.0, 73.0, 1.0);
        "rust brown", RUST_BROWN, (139.0, 49.0, 3.0, 1.0);
        "sand brown", SAND_BROWN, (203.0, 165.0, 96.0, 1.0);
        "swamp", SWAMP, (105.0, 131.0, 57.0, 1.0);
        "tealish green", TEALISH_GREEN, (12.0, 220.0, 115.0, 1.0);
        "burnt siena", BURNT_SIENA, (183.0, 82.0, 3.0, 1.0);
        "camo", CAMO, (127.0, 143.0, 78.0, 1.0);
        "dusk blue", DUSK_BLUE, (38.0, 83.0, 141.0, 1.0);
        "fern", FERN, (99.0, 169.0, 80.0, 1.0);
        "old rose", OLD_ROSE, (200.0, 127.0, 137.0, 1.0);
        "pale light green", PALE_LIGHT_GREEN, (177.0, 252.0, 153.0, 1.0);
        "peachy pink", PEACHY_PINK, (255.0, 154.0, 138.0, 1.0);
        "rosy pink", ROSY_PINK, (246.0, 104.0, 142.0, 1.0);
        "light bluish green", LIGHT_BLUISH_GREEN, (118.0, 253.0, 168.0, 1.0);
        "light bright green", LIGHT_BRIGHT_GREEN, (83.0, 254.0, 92.0, 1.0);
        "light neon green", LIGHT_NEON_GREEN, (78.0, 253.0, 84.0, 1.0);
        "light seafoam", LIGHT_SEAFOAM, (160.0, 254.0, 191.0, 1.0);
        "tiffany blue", TIFFANY_BLUE, (123.0, 242.0, 218.0, 1.0);
        "washed out green", WASHED_OUT_GREEN, (188.0, 245.0, 166.0, 1.0);
        "browny orange", BROWNY_ORANGE, (202.0, 107.0, 2.0, 1.0);
        "nice blue", NICE_BLUE, (16.0, 122.0, 176.0, 1.0);
        "sapphire", SAPPHIRE, (33.0, 56.0, 171.0, 1.0);
        "greyish teal", GREYISH_TEAL, (113.0, 159.0, 145.0, 1.0);
        "orangey yellow", ORANGEY_YELLOW, (253.0, 185.0, 21.0, 1.0);
        "parchment", PARCHMENT, (254.0, 252.0, 175.0, 1.0);
        "straw", STRAW, (252.0, 246.0, 121.0, 1.0);
        "very dark brown", VERY_DARK_BROWN, (29.0, 2.0, 0.0, 1.0);
        "terracota", TERRACOTA, (203.0, 104.0, 67.0, 1.0);
        "ugly blue", UGLY_BLUE, (49.0, 102.0, 138.0, 1.0);
        "clear blue", CLEAR_BLUE, (36.0, 122.0, 253.0, 1.0);
        "creme", CREME, (255.0, 255.0, 182.0, 1.0);
        "foam green", FOAM_GREEN, (144.0, 253.0, 169.0, 1.0);
        "light gold", LIGHT_GOLD, (253.0, 220.0, 92.0, 1.0);
        "seafoam blue", SEAFOAM_BLUE, (120.0, 209.0, 182.0, 1.0);
        "topaz", TOPAZ, (19.0, 187.0, 175.0, 1.0);
        "violet pink", VIOLET_PINK, (251.0, 95.0, 252.0, 1.0);
        "wintergreen", WINTERGREEN, (32.0, 249.0, 134.0, 1.0);
        "yellow tan", YELLOW_TAN, (255.0, 227.0, 110.0, 1.0);
        "dark fuchsia", DARK_FUCHSIA, (157.0, 7.0, 89.0, 1.0);
        "indigo blue", INDIGO_BLUE, (58.0, 24.0, 177.0, 1.0);
        "light yellowish green", LIGHT_YELLOWISH_GREEN, (194.0, 255.0, 137.0, 1.0);
        "pale magenta", PALE_MAGENTA, (215.0, 103.0, 173.0, 1.0);
        "rich purple", RICH_PURPLE, (114.0, 0.0, 88.0, 1.0);
        "sunflower yellow", SUNFLOWER_YELLOW, (255.0, 218.0, 3.0, 1.0);
        "leather", LEATHER, (172.0, 116.0, 52.0, 1.0);
        "racing green", RACING_GREEN, (1.0, 70.0, 0.0, 1.0);
        "vivid purple", VIVID_PURPLE, (153.0, 0.0, 250.0, 1.0);
        "dark royal blue", DARK_ROYAL_BLUE, (2.0, 6.0, 111.0, 1.0);
        "hazel", HAZEL, (142.0, 118.0, 24.0, 1.0);
        "muted pink", MUTED_PINK, (209.0, 118.0, 143.0, 1.0);
        "booger green", BOOGER_GREEN, (150.0, 180.0, 3.0, 1.0);
        "canary", CANARY, (253.0, 255.0, 99.0, 1.0);
        "cool grey", COOL_GREY, (149.0, 163.0, 166.0, 1.0);
        "dark taupe", DARK_TAUPE, (127.0, 104.0, 78.0, 1.0);
        "darkish purple", DARKISH_PURPLE, (117.0, 25.0, 115.0, 1.0);
        "true green", TRUE_GREEN, (8.0, 148.0, 4.0, 1.0);
        "coral pink", CORAL_PINK, (255.0, 97.0, 99.0, 1.0);
        "dark sage", DARK_SAGE, (89.0, 133.0, 86.0, 1.0);
        "dark slate blue", DARK_SLATE_BLUE, (33.0, 71.0, 97.0, 1.0);
        "flat blue", FLAT_BLUE, (60.0, 115.0, 168.0, 1.0);
        "mushroom", MUSHROOM, (186.0, 158.0, 136.0, 1.0);
        "rich blue", RICH_BLUE, (2.0, 27.0, 249.0, 1.0);
        "dirty purple", DIRTY_PURPLE, (115.0, 74.0, 101.0, 1.0);
        "greenblue", GREENBLUE, (35.0, 196.0, 139.0, 1.0);
        "icky green", ICKY_GREEN, (143.0, 174.0, 34.0, 1.0);
        "light khaki", LIGHT_KHAKI, (230.0, 242.0, 162.0, 1.0);
        "warm blue", WARM_BLUE, (75.0, 87.0, 219.0, 1.0);
        "dark hot pink", DARK_HOT_PINK, (217.0, 1.0, 102.0, 1.0);
        "deep sea blue", DEEP_SEA_BLUE, (1.0, 84.0, 130.0, 1.0);
        "carmine", CARMINE, (157.0, 2.0, 22.0, 1.0);
        "dark yellow green", DARK_YELLOW_GREEN, (114.0, 143.0, 2.0, 1.0);
        "pale peach", PALE_PEACH, (255.0, 229.0, 173.0, 1.0);
        "plum purple", PLUM_PURPLE, (78.0, 5.0, 80.0, 1.0);
        "golden rod", GOLDEN_ROD, (249.0, 188.0, 8.0, 1.0);
        "neon red", NEON_RED, (255.0, 7.0, 58.0, 1.0);
        "old pink", OLD_PINK, (199.0, 121.0, 134.0, 1.0);
        "very pale blue", VERY_PALE_BLUE, (214.0, 255.0, 254.0, 1.0);
        "blood orange", BLOOD_ORANGE, (254.0, 75.0, 3.0, 1.0);
        "grapefruit", GRAPEFRUIT, (253.0, 89.0, 86.0, 1.0);
        "sand yellow", SAND_YELLOW, (252.0, 225.0, 102.0, 1.0);
        "clay brown", CLAY_BROWN, (178.0, 113.0, 61.0, 1.0);
        "dark blue grey", DARK_BLUE_GREY, (31.0, 59.0, 77.0, 1.0);
        "flat green", FLAT_GREEN, (105.0, 157.0, 76.0, 1.0);
        "light green blue", LIGHT_GREEN_BLUE, (86.0, 252.0, 162.0, 1.0);
        "warm pink", WARM_PINK, (251.0, 85.0, 129.0, 1.0);
        "dodger blue", DODGER_BLUE, (62.0, 130.0, 252.0, 1.0);
        "gross green", GROSS_GREEN, (160.0, 191.0, 22.0, 1.0);
        "ice", ICE, (214.0, 255.0, 250.0, 1.0);
        "metallic blue", METALLIC_BLUE, (79.0, 115.0, 142.0, 1.0);
        "pale salmon", PALE_SALMON, (255.0, 177.0, 154.0, 1.0);
        "sap green", SAP_GREEN, (92.0, 139.0, 21.0, 1.0);
        "algae", ALGAE, (84.0, 172.0, 104.0, 1.0);
        "bluey grey", BLUEY_GREY, (137.0, 160.0, 176.0, 1.0);
        "greeny grey", GREENY_GREY, (126.0, 160.0, 122.0, 1.0);
        "highlighter green", HIGHLIGHTER_GREEN, (27.0, 252.0, 6.0, 1.0);
        "light light blue", LIGHT_LIGHT_BLUE, (202.0, 255.0, 251.0, 1.0);
        "light mint", LIGHT_MINT, (182.0, 255.0, 187.0, 1.0);
        "raw umber", RAW_UMBER, (167.0, 94.0, 9.0, 1.0);
        "vivid blue", VIVID_BLUE, (21.0, 46.0, 255.0, 1.0);
        "deep lavender", DEEP_LAVENDER, (141.0, 94.0, 183.0, 1.0);
        "dull teal", DULL_TEAL, (95.0, 158.0, 143.0, 1.0);
        "light greenish blue", LIGHT_GREENISH_BLUE, (99.0, 247.0, 180.0, 1.0);
        "mud green", MUD_GREEN, (96.0, 102.0, 2.0, 1.0);
        "pinky", PINKY, (252.0, 134.0, 170.0, 1.0);
        "red wine", RED_WINE, (140.0, 0.0, 52.0, 1.0);
        "shit green", SHIT_GREEN, (117.0, 128.0, 0.0, 1.0);
        "tan brown", TAN_BROWN, (171.0, 126.0, 76.0, 1.0);
        "darkblue", DARKBLUE, (3.0, 7.0, 100.0, 1.0);
        "rosa", ROSA, (254.0, 134.0, 164.0, 1.0);
        "lipstick", LIPSTICK, (213.0, 23.0, 78.0, 1.0);
        "pale mauve", PALE_MAUVE, (254.0, 208.0, 252.0, 1.0);
        "claret", CLARET, (104.0, 0.0, 24.0, 1.0);
        "dandelion", DANDELION, (254.0, 223.0, 8.0, 1.0);
        "orangered", ORANGERED, (254.0, 66.0, 15.0, 1.0);
        "poop green", POOP_GREEN, (111.0, 124.0, 0.0, 1.0);
        "ruby", RUBY, (202.0, 1.0, 71.0, 1.0);
        "dark", DARK, (27.0, 36.0, 49.0, 1.0);
        "greenish turquoise", GREENISH_TURQUOISE, (0.0, 251.0, 176.0, 1.0);
        "pastel red", PASTEL_RED, (219.0, 88.0, 86.0, 1.0);
        "piss yellow", PISS_YELLOW, (221.0, 214.0, 24.0, 1.0);
        "bright cyan", BRIGHT_CYAN, (65.0, 253.0, 254.0, 1.0);
        "dark coral", DARK_CORAL, (207.0, 82.0, 78.0, 1.0);
        "algae green", ALGAE_GREEN, (33.0, 195.0, 111.0, 1.0);
        "darkish red", DARKISH_RED, (169.0, 3.0, 8.0, 1.0);
        "reddy brown", REDDY_BROWN, (110.0, 16.0, 5.0, 1.0);
        "blush pink", BLUSH_PINK, (254.0, 130.0, 140.0, 1.0);
        "camouflage green", CAMOUFLAGE_GREEN, (75.0, 97.0, 19.0, 1.0);
        "lawn green", LAWN_GREEN, (77.0, 164.0, 9.0, 1.0);
        "putty", PUTTY, (190.0, 174.0, 138.0, 1.0);
        "vibrant blue", VIBRANT_BLUE, (3.0, 57.0, 248.0, 1.0);
        "dark sand", DARK_SAND, (168.0, 143.0, 89.0, 1.0);
        "saffron", SAFFRON, (254.0, 178.0, 9.0, 1.0);
        "twilight", TWILIGHT, (78.0, 81.0, 139.0, 1.0);
        "warm brown", WARM_BROWN, (150.0, 78.0, 2.0, 1.0);
        "bluegrey", BLUEGREY, (133.0, 163.0, 178.0, 1.0);
        "bubble gum pink", BUBBLE_GUM_PINK, (255.0, 105.0, 175.0, 1.0);
        "duck egg blue", DUCK_EGG_BLUE, (195.0, 251.0, 244.0, 1.0);
        "greenish cyan", GREENISH_CYAN, (42.0, 254.0, 183.0, 1.0);
        "petrol", PETROL, (0.0, 95.0, 106.0, 1.0);
        "royal", ROYAL, (12.0, 23.0, 147.0, 1.0);
        "butter", BUTTER, (255.0, 255.0, 129.0, 1.0);
        "dusty orange", DUSTY_ORANGE, (240.0, 131.0, 58.0, 1.0);
        "off yellow", OFF_YELLOW, (241.0, 243.0, 63.0, 1.0);
        "pale olive green", PALE_OLIVE_GREEN, (177.0, 210.0, 123.0, 1.0);
        "orangish", ORANGISH, (252.0, 130.0, 74.0, 1.0);
        "leaf", LEAF, (113.0, 170.0, 52.0, 1.0);
        "light blue grey", LIGHT_BLUE_GREY, (183.0, 201.0, 226.0, 1.0);
        "dried blood", DRIED_BLOOD, (75.0, 1.0, 1.0, 1.0);
        "lightish purple", LIGHTISH_PURPLE, (165.0, 82.0, 230.0, 1.0);
        "rusty red", RUSTY_RED, (175.0, 47.0, 13.0, 1.0);
        "lavender blue", LAVENDER_BLUE, (139.0, 136.0, 248.0, 1.0);
        "light grass green", LIGHT_GRASS_GREEN, (154.0, 247.0, 100.0, 1.0);
        "light mint green", LIGHT_MINT_GREEN, (166.0, 251.0, 178.0, 1.0);
        "sunflower", SUNFLOWER, (255.0, 197.0, 18.0, 1.0);
        "velvet", VELVET, (117.0, 8.0, 81.0, 1.0);
        "brick orange", BRICK_ORANGE, (193.0, 74.0, 9.0, 1.0);
        "lightish red", LIGHTISH_RED, (254.0, 47.0, 74.0, 1.0);
        "pure blue", PURE_BLUE, (2.0, 3.0, 226.0, 1.0);
        "twilight blue", TWILIGHT_BLUE, (10.0, 67.0, 122.0, 1.0);
        "violet red", VIOLET_RED, (165.0, 0.0, 85.0, 1.0);
        "yellowy brown", YELLOWY_BROWN, (174.0, 139.0, 12.0, 1.0);
        "carnation", CARNATION, (253.0, 121.0, 143.0, 1.0);
        "muddy yellow", MUDDY_YELLOW, (191.0, 172.0, 5.0, 1.0);
        "dark seafoam green", DARK_SEAFOAM_GREEN, (62.0, 175.0, 118.0, 1.0);
        "deep rose", DEEP_ROSE, (199.0, 71.0, 103.0, 1.0);
        "dusty red", DUSTY_RED, (185.0, 72.0, 78.0, 1.0);
        "lemon lime", LEMON_LIME, (191.0, 254.0, 40.0, 1.0);
        "brown yellow", BROWN_YELLOW, (178.0, 151.0, 5.0, 1.0);
        "purple brown", PURPLE_BROWN, (103.0, 58.0, 63.0, 1.0);
        "wisteria", WISTERIA, (168.0, 125.0, 194.0, 1.0);
        "banana yellow", BANANA_YELLOW, (250.0, 254.0, 75.0, 1.0);
        "lipstick red", LIPSTICK_RED, (192.0, 2.0, 47.0, 1.0);
        "water blue", WATER_BLUE, (14.0, 135.0, 204.0, 1.0);
        "brown grey", BROWN_GREY, (141.0, 132.0, 104.0, 1.0);
        "vibrant purple", VIBRANT_PURPLE, (173.0, 3.0, 222.0, 1.0);
        "baby green", BABY_GREEN, (140.0, 255.0, 158.0, 1.0);
        "barf green", BARF_GREEN, (148.0, 172.0, 2.0, 1.0);
        "eggshell blue", EGGSHELL_BLUE, (196.0, 255.0, 247.0, 1.0);
        "sandy yellow", SANDY_YELLOW, (253.0, 238.0, 115.0, 1.0);
        "cool green", COOL_GREEN, (51.0, 184.0, 100.0, 1.0);
        "pale", PALE, (255.0, 249.0, 208.0, 1.0);
        "hot magenta", HOT_MAGENTA, (245.0, 4.0, 201.0, 1.0);
        "greyblue", GREYBLUE, (119.0, 161.0, 181.0, 1.0);
        "purpley", PURPLEY, (135.0, 86.0, 228.0, 1.0);
        "baby shit green", BABY_SHIT_GREEN, (136.0, 151.0, 23.0, 1.0);
        "brownish pink", BROWNISH_PINK, (194.0, 126.0, 121.0, 1.0);
        "dark aquamarine", DARK_AQUAMARINE, (1.0, 115.0, 113.0, 1.0);
        "diarrhea", DIARRHEA, (159.0, 131.0, 3.0, 1.0);
        "light mustard", LIGHT_MUSTARD, (247.0, 213.0, 96.0, 1.0);
        "pale sky blue", PALE_SKY_BLUE, (189.0, 246.0, 254.0, 1.0);
        "turtle green", TURTLE_GREEN, (117.0, 184.0, 79.0, 1.0);
        "bright olive", BRIGHT_OLIVE, (156.0, 187.0, 4.0, 1.0);
        "dark grey blue", DARK_GREY_BLUE, (41.0, 70.0, 91.0, 1.0);
        "greeny brown", GREENY_BROWN, (105.0, 96.0, 6.0, 1.0);
        "lemon green", LEMON_GREEN, (173.0, 248.0, 2.0, 1.0);
        "light periwinkle", LIGHT_PERIWINKLE, (193.0, 198.0, 252.0, 1.0);
        "seaweed green", SEAWEED_GREEN, (53.0, 173.0, 107.0, 1.0);
        "sunshine yellow", SUNSHINE_YELLOW, (255.0, 253.0, 55.0, 1.0);
        "ugly purple", UGLY_PURPLE, (164.0, 66.0, 160.0, 1.0);
        "medium pink", MEDIUM_PINK, (243.0, 97.0, 150.0, 1.0);
        "puke brown", PUKE_BROWN, (148.0, 119.0, 6.0, 1.0);
        "very light pink", VERY_LIGHT_PINK, (255.0, 244.0, 242.0, 1.0);
        "viridian", VIRIDIAN, (30.0, 145.0, 103.0, 1.0);
        "bile", BILE, (181.0, 195.0, 6.0, 1.0);
        "faded yellow", FADED_YELLOW, (254.0, 255.0, 127.0, 1.0);
        "very pale green", VERY_PALE_GREEN, (207.0, 253.0, 188.0, 1.0);
        "vibrant green", VIBRANT_GREEN, (10.0, 221.0, 8.0, 1.0);
        "bright lime", BRIGHT_LIME, (135.0, 253.0, 5.0, 1.0);
        "spearmint", SPEARMINT, (30.0, 248.0, 118.0, 1.0);
        "light aquamarine", LIGHT_AQUAMARINE, (123.0, 253.0, 199.0, 1.0);
        "light sage", LIGHT_SAGE, (188.0, 236.0, 172.0, 1.0);
        "yellowgreen", YELLOWGREEN, (187.0, 249.0, 15.0, 1.0);
        "baby poo", BABY_POO, (171.0, 144.0, 4.0, 1.0);
        "dark seafoam", DARK_SEAFOAM, (31.0, 181.0, 122.0, 1.0);
        "deep teal", DEEP_TEAL, (0.0, 85.0, 90.0, 1.0);
        "heather", HEATHER, (164.0, 132.0, 172.0, 1.0);
        "rust orange", RUST_ORANGE, (196.0, 85.0, 8.0, 1.0);
        "dirty blue", DIRTY_BLUE, (63.0, 130.0, 157.0, 1.0);
        "fern green", FERN_GREEN, (84.0, 141.0, 68.0, 1.0);
        "bright lilac", BRIGHT_LILAC, (201.0, 94.0, 251.0, 1.0);
        "weird green", WEIRD_GREEN, (58.0, 229.0, 127.0, 1.0);
        "peacock blue", PEACOCK_BLUE, (1.0, 103.0, 149.0, 1.0);
        "avocado green", AVOCADO_GREEN, (135.0, 169.0, 34.0, 1.0);
        "faded orange", FADED_ORANGE, (240.0, 148.0, 77.0, 1.0);
        "grape purple", GRAPE_PURPLE, (93.0, 20.0, 81.0, 1.0);
        "hot green", HOT_GREEN, (37.0, 255.0, 41.0, 1.0);
        "lime yellow", LIME_YELLOW, (208.0, 254.0, 29.0, 1.0);
        "mango", MANGO, (255.0, 166.0, 43.0, 1.0);
        "shamrock", SHAMROCK, (1.0, 180.0, 76.0, 1.0);
        "bubblegum", BUBBLEGUM, (255.0, 108.0, 181.0, 1.0);
        "purplish brown", PURPLISH_BROWN, (107.0, 66.0, 71.0, 1.0);
        "vomit yellow", VOMIT_YELLOW, (199.0, 193.0, 12.0, 1.0);
        "pale cyan", PALE_CYAN, (183.0, 255.0, 250.0, 1.0);
        "key lime", KEY_LIME, (174.0, 255.0, 110.0, 1.0);
        "tomato red", TOMATO_RED, (236.0, 45.0, 1.0, 1.0);
        "lightgreen", LIGHTGREEN, (118.0, 255.0, 123.0, 1.0);
        "merlot", MERLOT, (115.0, 0.0, 57.0, 1.0);
        "night blue", NIGHT_BLUE, (4.0, 3.0, 72.0, 1.0);
        "purpleish pink", PURPLEISH_PINK, (223.0, 78.0, 200.0, 1.0);
        "apple", APPLE, (110.0, 203.0, 60.0, 1.0);
        "baby poop green", BABY_POOP_GREEN, (143.0, 152.0, 5.0, 1.0);
        "green apple", GREEN_APPLE, (94.0, 220.0, 31.0, 1.0);
        "heliotrope", HELIOTROPE, (217.0, 79.0, 245.0, 1.0);
        "almost black", ALMOST_BLACK, (7.0, 13.0, 13.0, 1.0);
        "cool blue", COOL_BLUE, (73.0, 132.0, 184.0, 1.0);
        "leafy green", LEAFY_GREEN, (81.0, 183.0, 59.0, 1.0);
        "mustard brown", MUSTARD_BROWN, (172.0, 126.0, 4.0, 1.0);
        "dusk", DUSK, (78.0, 84.0, 129.0, 1.0);
        "dull brown", DULL_BROWN, (135.0, 110.0, 75.0, 1.0);
        "frog green", FROG_GREEN, (88.0, 188.0, 8.0, 1.0);
        "vivid green", VIVID_GREEN, (47.0, 239.0, 16.0, 1.0);
        "bright light green", BRIGHT_LIGHT_GREEN, (45.0, 254.0, 84.0, 1.0);
        "fluro green", FLURO_GREEN, (10.0, 255.0, 2.0, 1.0);
        "kiwi", KIWI, (156.0, 239.0, 67.0, 1.0);
        "seaweed", SEAWEED, (24.0, 209.0, 123.0, 1.0);
        "navy green", NAVY_GREEN, (53.0, 83.0, 10.0, 1.0);
        "ultramarine blue", ULTRAMARINE_BLUE, (24.0, 5.0, 219.0, 1.0);
        "iris", IRIS, (98.0, 88.0, 196.0, 1.0);
        "pastel orange", PASTEL_ORANGE, (255.0, 150.0, 79.0, 1.0);
        "yellowish orange", YELLOWISH_ORANGE, (255.0, 171.0, 15.0, 1.0);
        "perrywinkle", PERRYWINKLE, (143.0, 140.0, 231.0, 1.0);
        "tealish", TEALISH, (36.0, 188.0, 168.0, 1.0);
        "dark plum", DARK_PLUM, (63.0, 1.0, 44.0, 1.0);
        "pear", PEAR, (203.0, 248.0, 95.0, 1.0);
        "pinkish orange", PINKISH_ORANGE, (255.0, 114.0, 76.0, 1.0);
        "midnight purple", MIDNIGHT_PURPLE, (40.0, 1.0, 55.0, 1.0);
        "light urple", LIGHT_URPLE, (179.0, 111.0, 246.0, 1.0);
        "dark mint", DARK_MINT, (72.0, 192.0, 114.0, 1.0);
        "greenish tan", GREENISH_TAN, (188.0, 203.0, 122.0, 1.0);
        "light burgundy", LIGHT_BURGUNDY, (168.0, 65.0, 91.0, 1.0);
        "turquoise blue", TURQUOISE_BLUE, (6.0, 177.0, 196.0, 1.0);
        "ugly pink", UGLY_PINK, (205.0, 117.0, 132.0, 1.0);
        "sandy", SANDY, (241.0, 218.0, 122.0, 1.0);
        "electric pink", ELECTRIC_PINK, (255.0, 4.0, 144.0, 1.0);
        "muted purple", MUTED_PURPLE, (128.0, 91.0, 135.0, 1.0);
        "mid green", MID_GREEN, (80.0, 167.0, 71.0, 1.0);
        "greyish", GREYISH, (168.0, 164.0, 149.0, 1.0);
        "neon yellow", NEON_YELLOW, (207.0, 255.0, 4.0, 1.0);
        "banana", BANANA, (255.0, 255.0, 126.0, 1.0);
        "carnation pink", CARNATION_PINK, (255.0, 127.0, 167.0, 1.0);
        "tomato", TOMATO, (239.0, 64.0, 38.0, 1.0);
        "sea", SEA, (60.0, 153.0, 146.0, 1.0);
        "muddy brown", MUDDY_BROWN, (136.0, 104.0, 6.0, 1.0);
        "turquoise green", TURQUOISE_GREEN, (4.0, 244.0, 137.0, 1.0);
        "buff", BUFF, (254.0, 246.0, 158.0, 1.0);
        "fawn", FAWN, (207.0, 175.0, 123.0, 1.0);
        "muted blue", MUTED_BLUE, (59.0, 113.0, 159.0, 1.0);
        "pale rose", PALE_ROSE, (253.0, 193.0, 197.0, 1.0);
        "dark mint green", DARK_MINT_GREEN, (32.0, 192.0, 115.0, 1.0);
        "amethyst", AMETHYST, (155.0, 95.0, 192.0, 1.0);
        "chestnut", CHESTNUT, (116.0, 40.0, 2.0, 1.0);
        "sick green", SICK_GREEN, (157.0, 185.0, 44.0, 1.0);
        "pea", PEA, (164.0, 191.0, 32.0, 1.0);
        "rusty orange", RUSTY_ORANGE, (205.0, 89.0, 9.0, 1.0);
        "stone", STONE, (173.0, 165.0, 135.0, 1.0);
        "rose red", ROSE_RED, (190.0, 1.0, 60.0, 1.0);
        "pale aqua", PALE_AQUA, (184.0, 255.0, 235.0, 1.0);
        "deep orange", DEEP_ORANGE, (220.0, 77.0, 1.0, 1.0);
        "earth", EARTH, (162.0, 101.0, 62.0, 1.0);
        "mossy green", MOSSY_GREEN, (99.0, 139.0, 39.0, 1.0);
        "grassy green", GRASSY_GREEN, (65.0, 156.0, 3.0, 1.0);
        "pale lime green", PALE_LIME_GREEN, (177.0, 255.0, 101.0, 1.0);
        "light grey blue", LIGHT_GREY_BLUE, (157.0, 188.0, 212.0, 1.0);
        "pale grey", PALE_GREY, (253.0, 253.0, 254.0, 1.0);
        "asparagus", ASPARAGUS, (119.0, 171.0, 86.0, 1.0);
        "blueberry", BLUEBERRY, (70.0, 65.0, 150.0, 1.0);
        "purple red", PURPLE_RED, (153.0, 1.0, 71.0, 1.0);
        "pale lime", PALE_LIME, (190.0, 253.0, 115.0, 1.0);
        "greenish teal", GREENISH_TEAL, (50.0, 191.0, 132.0, 1.0);
        "caramel", CARAMEL, (175.0, 111.0, 9.0, 1.0);
        "deep magenta", DEEP_MAGENTA, (160.0, 2.0, 92.0, 1.0);
        "light peach", LIGHT_PEACH, (255.0, 216.0, 177.0, 1.0);
        "milk chocolate", MILK_CHOCOLATE, (127.0, 78.0, 30.0, 1.0);
        "ocher", OCHER, (191.0, 155.0, 12.0, 1.0);
        "off green", OFF_GREEN, (107.0, 163.0, 83.0, 1.0);
        "purply pink", PURPLY_PINK, (240.0, 117.0, 230.0, 1.0);
        "lightblue", LIGHTBLUE, (123.0, 200.0, 246.0, 1.0);
        "dusky blue", DUSKY_BLUE, (71.0, 95.0, 148.0, 1.0);
        "golden", GOLDEN, (245.0, 191.0, 3.0, 1.0);
        "light beige", LIGHT_BEIGE, (255.0, 254.0, 182.0, 1.0);
        "butter yellow", BUTTER_YELLOW, (255.0, 253.0, 116.0, 1.0);
        "dusky purple", DUSKY_PURPLE, (137.0, 91.0, 123.0, 1.0);
        "french blue", FRENCH_BLUE, (67.0, 107.0, 173.0, 1.0);
        "ugly yellow", UGLY_YELLOW, (208.0, 193.0, 1.0, 1.0);
        "greeny yellow", GREENY_YELLOW, (198.0, 248.0, 8.0, 1.0);
        "orangish red", ORANGISH_RED, (244.0, 54.0, 5.0, 1.0);
        "shamrock green", SHAMROCK_GREEN, (2.0, 193.0, 77.0, 1.0);
        "orangish brown", ORANGISH_BROWN, (178.0, 95.0, 3.0, 1.0);
        "tree green", TREE_GREEN, (42.0, 126.0, 25.0, 1.0);
        "deep violet", DEEP_VIOLET, (73.0, 6.0, 72.0, 1.0);
        "gunmetal", GUNMETAL, (83.0, 98.0, 103.0, 1.0);
        "cherry", CHERRY, (207.0, 2.0, 52.0, 1.0);
        "sandy brown", SANDY_BROWN, (196.0, 166.0, 97.0, 1.0);
        "warm grey", WARM_GREY, (151.0, 138.0, 132.0, 1.0);
        "dark indigo", DARK_INDIGO, (31.0, 9.0, 84.0, 1.0);
        "midnight", MIDNIGHT, (3.0, 1.0, 45.0, 1.0);
        "bluey green", BLUEY_GREEN, (43.0, 177.0, 121.0, 1.0);
        "grey pink", GREY_PINK, (195.0, 144.0, 155.0, 1.0);
        "soft purple", SOFT_PURPLE, (166.0, 111.0, 181.0, 1.0);
        "blood", BLOOD, (119.0, 0.0, 1.0, 1.0);
        "brown red", BROWN_RED, (146.0, 43.0, 5.0, 1.0);
        "medium grey", MEDIUM_GREY, (125.0, 127.0, 124.0, 1.0);
        "berry", BERRY, (153.0, 15.0, 75.0, 1.0);
        "poo", POO, (143.0, 115.0, 3.0, 1.0);
        "purpley pink", PURPLEY_PINK, (200.0, 60.0, 185.0, 1.0);
        "light salmon", LIGHT_SALMON, (254.0, 169.0, 147.0, 1.0);
        "snot", SNOT, (172.0, 187.0, 13.0, 1.0);
        "easter purple", EASTER_PURPLE, (192.0, 113.0, 254.0, 1.0);
        "light yellow green", LIGHT_YELLOW_GREEN, (204.0, 253.0, 127.0, 1.0);
        "dark navy blue", DARK_NAVY_BLUE, (0.0, 2.0, 46.0, 1.0);
        "drab", DRAB, (130.0, 131.0, 68.0, 1.0);
        "light rose", LIGHT_ROSE, (255.0, 197.0, 203.0, 1.0);
        "rouge", ROUGE, (171.0, 18.0, 57.0, 1.0);
        "purplish red", PURPLISH_RED, (176.0, 5.0, 75.0, 1.0);
        "slime green", SLIME_GREEN, (153.0, 204.0, 4.0, 1.0);
        "baby poop", BABY_POOP, (147.0, 124.0, 0.0, 1.0);
        "irish green", IRISH_GREEN, (1.0, 149.0, 41.0, 1.0);
        "dark navy", DARK_NAVY, (0.0, 4.0, 53.0, 1.0);
        "greeny blue", GREENY_BLUE, (66.0, 179.0, 149.0, 1.0);
        "light plum", LIGHT_PLUM, (157.0, 87.0, 131.0, 1.0);
        "pinkish grey", PINKISH_GREY, (200.0, 172.0, 169.0, 1.0);
        "dirty orange", DIRTY_ORANGE, (200.0, 118.0, 6.0, 1.0);
        "rust red", RUST_RED, (170.0, 39.0, 4.0, 1.0);
        "pale lilac", PALE_LILAC, (228.0, 203.0, 255.0, 1.0);
        "orangey red", ORANGEY_RED, (250.0, 66.0, 36.0, 1.0);
        "primary blue", PRIMARY_BLUE, (8.0, 4.0, 249.0, 1.0);
        "kermit green", KERMIT_GREEN, (92.0, 178.0, 0.0, 1.0);
        "brownish purple", BROWNISH_PURPLE, (118.0, 66.0, 78.0, 1.0);
        "murky green", MURKY_GREEN, (108.0, 122.0, 14.0, 1.0);
        "wheat", WHEAT, (251.0, 221.0, 126.0, 1.0);
        "very dark purple", VERY_DARK_PURPLE, (42.0, 1.0, 52.0, 1.0);
        "bottle green", BOTTLE_GREEN, (4.0, 74.0, 5.0, 1.0);
        "watermelon", WATERMELON, (253.0, 70.0, 89.0, 1.0);
        "deep sky blue", DEEP_SKY_BLUE, (13.0, 117.0, 248.0, 1.0);
        "fire engine red", FIRE_ENGINE_RED, (254.0, 0.0, 2.0, 1.0);
        "yellow ochre", YELLOW_OCHRE, (203.0, 157.0, 6.0, 1.0);
        "pumpkin orange", PUMPKIN_ORANGE, (251.0, 125.0, 7.0, 1.0);
        "pale olive", PALE_OLIVE, (185.0, 204.0, 129.0, 1.0);
        "light lilac", LIGHT_LILAC, (237.0, 200.0, 255.0, 1.0);
        "lightish green", LIGHTISH_GREEN, (97.0, 225.0, 96.0, 1.0);
        "carolina blue", CAROLINA_BLUE, (138.0, 184.0, 254.0, 1.0);
        "mulberry", MULBERRY, (146.0, 10.0, 78.0, 1.0);
        "shocking pink", SHOCKING_PINK, (254.0, 2.0, 162.0, 1.0);
        "auburn", AUBURN, (154.0, 48.0, 1.0, 1.0);
        "bright lime green", BRIGHT_LIME_GREEN, (101.0, 254.0, 8.0, 1.0);
        "celadon", CELADON, (190.0, 253.0, 183.0, 1.0);
        "pinkish brown", PINKISH_BROWN, (177.0, 114.0, 97.0, 1.0);
        "poo brown", POO_BROWN, (136.0, 95.0, 1.0, 1.0);
        "bright sky blue", BRIGHT_SKY_BLUE, (2.0, 204.0, 254.0, 1.0);
        "celery", CELERY, (193.0, 253.0, 149.0, 1.0);
        "dirt brown", DIRT_BROWN, (131.0, 101.0, 57.0, 1.0);
        "strawberry", STRAWBERRY, (251.0, 41.0, 67.0, 1.0);
        "dark lime", DARK_LIME, (132.0, 183.0, 1.0, 1.0);
        "copper", COPPER, (182.0, 99.0, 37.0, 1.0);
        "medium brown", MEDIUM_BROWN, (127.0, 81.0, 18.0, 1.0);
        "muted green", MUTED_GREEN, (95.0, 160.0, 82.0, 1.0);
        "robin's egg", ROBINS_EGG, (109.0, 237.0, 253.0, 1.0);
        "bright aqua", BRIGHT_AQUA, (11.0, 249.0, 234.0, 1.0);
        "bright lavender", BRIGHT_LAVENDER, (199.0, 96.0, 255.0, 1.0);
        "ivory", IVORY, (255.0, 255.0, 203.0, 1.0);
        "very light purple", VERY_LIGHT_PURPLE, (246.0, 206.0, 252.0, 1.0);
        "light navy", LIGHT_NAVY, (21.0, 80.0, 132.0, 1.0);
        "pink red", PINK_RED, (245.0, 5.0, 79.0, 1.0);
        "olive brown", OLIVE_BROWN, (100.0, 84.0, 3.0, 1.0);
        "poop brown", POOP_BROWN, (122.0, 89.0, 1.0, 1.0);
        "mustard green", MUSTARD_GREEN, (168.0, 181.0, 4.0, 1.0);
        "ocean green", OCEAN_GREEN, (61.0, 153.0, 115.0, 1.0);
        "very dark blue", VERY_DARK_BLUE, (0.0, 1.0, 51.0, 1.0);
        "dusty green", DUSTY_GREEN, (118.0, 169.0, 115.0, 1.0);
        "light navy blue", LIGHT_NAVY_BLUE, (46.0, 90.0, 136.0, 1.0);
        "minty green", MINTY_GREEN, (11.0, 247.0, 125.0, 1.0);
        "adobe", ADOBE, (189.0, 108.0, 72.0, 1.0);
        "barney", BARNEY, (172.0, 29.0, 184.0, 1.0);
        "jade green", JADE_GREEN, (43.0, 175.0, 106.0, 1.0);
        "bright light blue", BRIGHT_LIGHT_BLUE, (38.0, 247.0, 253.0, 1.0);
        "light lime", LIGHT_LIME, (174.0, 253.0, 108.0, 1.0);
        "dark khaki", DARK_KHAKI, (155.0, 143.0, 85.0, 1.0);
        "orange yellow", ORANGE_YELLOW, (255.0, 173.0, 1.0, 1.0);
        "ocre", OCRE, (198.0, 156.0, 4.0, 1.0);
        "maize", MAIZE, (244.0, 208.0, 84.0, 1.0);
        "faded pink", FADED_PINK, (222.0, 157.0, 172.0, 1.0);
        "british racing green", BRITISH_RACING_GREEN, (5.0, 72.0, 13.0, 1.0);
        "sandstone", SANDSTONE, (201.0, 174.0, 116.0, 1.0);
        "mud brown", MUD_BROWN, (96.0, 70.0, 15.0, 1.0);
        "light sea green", LIGHT_SEA_GREEN, (152.0, 246.0, 176.0, 1.0);
        "robin egg blue", ROBIN_EGG_BLUE, (138.0, 241.0, 254.0, 1.0);
        "aqua marine", AQUA_MARINE, (46.0, 232.0, 187.0, 1.0);
        "dark sea green", DARK_SEA_GREEN, (17.0, 135.0, 93.0, 1.0);
        "soft pink", SOFT_PINK, (253.0, 176.0, 192.0, 1.0);
        "orangey brown", ORANGEY_BROWN, (177.0, 96.0, 2.0, 1.0);
        "cherry red", CHERRY_RED, (247.0, 2.0, 42.0, 1.0);
        "burnt yellow", BURNT_YELLOW, (213.0, 171.0, 9.0, 1.0);
        "brownish grey", BROWNISH_GREY, (134.0, 119.0, 95.0, 1.0);
        "camel", CAMEL, (198.0, 159.0, 89.0, 1.0);
        "purplish grey", PURPLISH_GREY, (122.0, 104.0, 127.0, 1.0);
        "marine", MARINE, (4.0, 46.0, 96.0, 1.0);
        "greyish pink", GREYISH_PINK, (200.0, 141.0, 148.0, 1.0);
        "pale turquoise", PALE_TURQUOISE, (165.0, 251.0, 213.0, 1.0);
        "pastel yellow", PASTEL_YELLOW, (255.0, 254.0, 113.0, 1.0);
        "bluey purple", BLUEY_PURPLE, (98.0, 65.0, 199.0, 1.0);
        "canary yellow", CANARY_YELLOW, (255.0, 254.0, 64.0, 1.0);
        "faded red", FADED_RED, (211.0, 73.0, 78.0, 1.0);
        "sepia", SEPIA, (152.0, 94.0, 43.0, 1.0);
        "coffee", COFFEE, (166.0, 129.0, 76.0, 1.0);
        "bright magenta", BRIGHT_MAGENTA, (255.0, 8.0, 232.0, 1.0);
        "mocha", MOCHA, (157.0, 118.0, 81.0, 1.0);
        "ecru", ECRU, (254.0, 255.0, 202.0, 1.0);
        "purpleish", PURPLEISH, (152.0, 86.0, 141.0, 1.0);
        "cranberry", CRANBERRY, (158.0, 0.0, 58.0, 1.0);
        "darkish green", DARKISH_GREEN, (40.0, 124.0, 55.0, 1.0);
        "brown orange", BROWN_ORANGE, (185.0, 105.0, 2.0, 1.0);
        "dusky rose", DUSKY_ROSE, (186.0, 104.0, 115.0, 1.0);
        "melon", MELON, (255.0, 120.0, 85.0, 1.0);
        "sickly green", SICKLY_GREEN, (148.0, 178.0, 28.0, 1.0);
        "silver", SILVER, (197.0, 201.0, 199.0, 1.0);
        "purply blue", PURPLY_BLUE, (102.0, 26.0, 238.0, 1.0);
        "purpleish blue", PURPLEISH_BLUE, (97.0, 64.0, 239.0, 1.0);
        "hospital green", HOSPITAL_GREEN, (155.0, 229.0, 170.0, 1.0);
        "shit brown", SHIT_BROWN, (123.0, 88.0, 4.0, 1.0);
        "mid blue", MID_BLUE, (39.0, 106.0, 179.0, 1.0);
        "amber", AMBER, (254.0, 179.0, 8.0, 1.0);
        "easter green", EASTER_GREEN, (140.0, 253.0, 126.0, 1.0);
        "soft blue", SOFT_BLUE, (100.0, 136.0, 234.0, 1.0);
        "cerulean blue", CERULEAN_BLUE, (5.0, 110.0, 238.0, 1.0);
        "golden brown", GOLDEN_BROWN, (178.0, 122.0, 1.0, 1.0);
        "bright turquoise", BRIGHT_TURQUOISE, (15.0, 254.0, 249.0, 1.0);
        "red pink", RED_PINK, (250.0, 42.0, 85.0, 1.0);
        "red purple", RED_PURPLE, (130.0, 7.0, 71.0, 1.0);
        "greyish brown", GREYISH_BROWN, (122.0, 106.0, 79.0, 1.0);
        "vermillion", VERMILLION, (244.0, 50.0, 12.0, 1.0);
        "russet", RUSSET, (161.0, 57.0, 5.0, 1.0);
        "steel grey", STEEL_GREY, (111.0, 130.0, 138.0, 1.0);
        "lighter purple", LIGHTER_PURPLE, (165.0, 90.0, 244.0, 1.0);
        "bright violet", BRIGHT_VIOLET, (173.0, 10.0, 253.0, 1.0);
        "prussian blue", PRUSSIAN_BLUE, (0.0, 69.0, 119.0, 1.0);
        "slate green", SLATE_GREEN, (101.0, 141.0, 109.0, 1.0);
        "dirty pink", DIRTY_PINK, (202.0, 123.0, 128.0, 1.0);
        "dark blue green", DARK_BLUE_GREEN, (0.0, 82.0, 73.0, 1.0);
        "pine", PINE, (43.0, 93.0, 52.0, 1.0);
        "yellowy green", YELLOWY_GREEN, (191.0, 241.0, 40.0, 1.0);
        "dark gold", DARK_GOLD, (181.0, 148.0, 16.0, 1.0);
        "bluish", BLUISH, (41.0, 118.0, 187.0, 1.0);
        "darkish blue", DARKISH_BLUE, (1.0, 65.0, 130.0, 1.0);
        "dull red", DULL_RED, (187.0, 63.0, 63.0, 1.0);
        "pinky red", PINKY_RED, (252.0, 38.0, 71.0, 1.0);
        "bronze", BRONZE, (168.0, 121.0, 0.0, 1.0);
        "pale teal", PALE_TEAL, (130.0, 203.0, 178.0, 1.0);
        "military green", MILITARY_GREEN, (102.0, 124.0, 62.0, 1.0);
        "barbie pink", BARBIE_PINK, (254.0, 70.0, 165.0, 1.0);
        "bubblegum pink", BUBBLEGUM_PINK, (254.0, 131.0, 204.0, 1.0);
        "pea soup green", PEA_SOUP_GREEN, (148.0, 166.0, 23.0, 1.0);
        "dark mustard", DARK_MUSTARD, (168.0, 137.0, 5.0, 1.0);
        "shit", SHIT, (127.0, 95.0, 0.0, 1.0);
        "medium purple", MEDIUM_PURPLE, (158.0, 67.0, 162.0, 1.0);
        "very dark green", VERY_DARK_GREEN, (6.0, 46.0, 3.0, 1.0);
        "dirt", DIRT, (138.0, 110.0, 69.0, 1.0);
        "dusky pink", DUSKY_PINK, (204.0, 122.0, 139.0, 1.0);
        "red violet", RED_VIOLET, (158.0, 1.0, 104.0, 1.0);
        "lemon yellow", LEMON_YELLOW, (253.0, 255.0, 56.0, 1.0);
        "pistachio", PISTACHIO, (192.0, 250.0, 139.0, 1.0);
        "dull yellow", DULL_YELLOW, (238.0, 220.0, 91.0, 1.0);
        "dark lime green", DARK_LIME_GREEN, (126.0, 189.0, 1.0, 1.0);
        "denim blue", DENIM_BLUE, (59.0, 91.0, 146.0, 1.0);
        "teal blue", TEAL_BLUE, (1.0, 136.0, 159.0, 1.0);
        "lightish blue", LIGHTISH_BLUE, (61.0, 122.0, 253.0, 1.0);
        "purpley blue", PURPLEY_BLUE, (95.0, 52.0, 231.0, 1.0);
        "light indigo", LIGHT_INDIGO, (109.0, 90.0, 207.0, 1.0);
        "swamp green", SWAMP_GREEN, (116.0, 133.0, 0.0, 1.0);
        "brown green", BROWN_GREEN, (112.0, 108.0, 17.0, 1.0);
        "dark maroon", DARK_MAROON, (60.0, 0.0, 8.0, 1.0);
        "hot purple", HOT_PURPLE, (203.0, 0.0, 245.0, 1.0);
        "dark forest green", DARK_FOREST_GREEN, (0.0, 45.0, 4.0, 1.0);
        "faded blue", FADED_BLUE, (101.0, 140.0, 187.0, 1.0);
        "drab green", DRAB_GREEN, (116.0, 149.0, 81.0, 1.0);
        "light lime green", LIGHT_LIME_GREEN, (185.0, 255.0, 102.0, 1.0);
        "snot green", SNOT_GREEN, (157.0, 193.0, 0.0, 1.0);
        "yellowish", YELLOWISH, (250.0, 238.0, 102.0, 1.0);
        "light blue green", LIGHT_BLUE_GREEN, (126.0, 251.0, 179.0, 1.0);
        "bordeaux", BORDEAUX, (123.0, 0.0, 44.0, 1.0);
        "light mauve", LIGHT_MAUVE, (194.0, 146.0, 161.0, 1.0);
        "ocean", OCEAN, (1.0, 123.0, 146.0, 1.0);
        "marigold", MARIGOLD, (252.0, 192.0, 6.0, 1.0);
        "muddy green", MUDDY_GREEN, (101.0, 116.0, 50.0, 1.0);
        "dull orange", DULL_ORANGE, (216.0, 134.0, 59.0, 1.0);
        "steel", STEEL, (115.0, 133.0, 149.0, 1.0);
        "electric purple", ELECTRIC_PURPLE, (170.0, 35.0, 255.0, 1.0);
        "fluorescent green", FLUORESCENT_GREEN, (8.0, 255.0, 8.0, 1.0);
        "yellowish brown", YELLOWISH_BROWN, (155.0, 122.0, 1.0, 1.0);
        "blush", BLUSH, (242.0, 158.0, 142.0, 1.0);
        "soft green", SOFT_GREEN, (111.0, 194.0, 118.0, 1.0);
        "bright orange", BRIGHT_ORANGE, (255.0, 91.0, 0.0, 1.0);
        "lemon", LEMON, (253.0, 255.0, 82.0, 1.0);
        "purple grey", PURPLE_GREY, (134.0, 111.0, 133.0, 1.0);
        "acid green", ACID_GREEN, (143.0, 254.0, 9.0, 1.0);
        "pale lavender", PALE_LAVENDER, (238.0, 207.0, 254.0, 1.0);
        "violet blue", VIOLET_BLUE, (81.0, 10.0, 201.0, 1.0);
        "light forest green", LIGHT_FOREST_GREEN, (79.0, 145.0, 83.0, 1.0);
        "burnt red", BURNT_RED, (159.0, 35.0, 5.0, 1.0);
        "khaki green", KHAKI_GREEN, (114.0, 134.0, 57.0, 1.0);
        "cerise", CERISE, (222.0, 12.0, 98.0, 1.0);
        "faded purple", FADED_PURPLE, (145.0, 110.0, 153.0, 1.0);
        "apricot", APRICOT, (255.0, 177.0, 109.0, 1.0);
        "dark olive green", DARK_OLIVE_GREEN, (60.0, 77.0, 3.0, 1.0);
        "grey brown", GREY_BROWN, (127.0, 112.0, 83.0, 1.0);
        "green grey", GREEN_GREY, (119.0, 146.0, 111.0, 1.0);
        "true blue", TRUE_BLUE, (1.0, 15.0, 204.0, 1.0);
        "pale violet", PALE_VIOLET, (206.0, 174.0, 250.0, 1.0);
        "periwinkle blue", PERIWINKLE_BLUE, (143.0, 153.0, 251.0, 1.0);
        "light sky blue", LIGHT_SKY_BLUE, (198.0, 252.0, 255.0, 1.0);
        "blurple", BLURPLE, (85.0, 57.0, 204.0, 1.0);
        "green brown", GREEN_BROWN, (84.0, 78.0, 3.0, 1.0);
        "bluegreen", BLUEGREEN, (1.0, 122.0, 121.0, 1.0);
        "bright teal", BRIGHT_TEAL, (1.0, 249.0, 198.0, 1.0);
        "brownish yellow", BROWNISH_YELLOW, (201.0, 176.0, 3.0, 1.0);
        "pea soup", PEA_SOUP, (146.0, 153.0, 1.0, 1.0);
        "forest", FOREST, (11.0, 85.0, 9.0, 1.0);
        "barney purple", BARNEY_PURPLE, (160.0, 4.0, 152.0, 1.0);
        "ultramarine", ULTRAMARINE, (32.0, 0.0, 177.0, 1.0);
        "purplish", PURPLISH, (148.0, 86.0, 140.0, 1.0);
        "puke yellow", PUKE_YELLOW, (194.0, 190.0, 14.0, 1.0);
        "bluish grey", BLUISH_GREY, (116.0, 139.0, 151.0, 1.0);
        "dark periwinkle", DARK_PERIWINKLE, (102.0, 95.0, 209.0, 1.0);
        "dark lilac", DARK_LILAC, (156.0, 109.0, 165.0, 1.0);
        "reddish", REDDISH, (196.0, 66.0, 64.0, 1.0);
        "light maroon", LIGHT_MAROON, (162.0, 72.0, 87.0, 1.0);
        "dusty purple", DUSTY_PURPLE, (130.0, 95.0, 135.0, 1.0);
        "terra cotta", TERRA_COTTA, (201.0, 100.0, 59.0, 1.0);
        "avocado", AVOCADO, (144.0, 177.0, 52.0, 1.0);
        "marine blue", MARINE_BLUE, (1.0, 56.0, 106.0, 1.0);
        "teal green", TEAL_GREEN, (37.0, 163.0, 111.0, 1.0);
        "slate grey", SLATE_GREY, (89.0, 101.0, 109.0, 1.0);
        "lighter green", LIGHTER_GREEN, (117.0, 253.0, 99.0, 1.0);
        "electric green", ELECTRIC_GREEN, (33.0, 252.0, 13.0, 1.0);
        "dusty blue", DUSTY_BLUE, (90.0, 134.0, 173.0, 1.0);
        "golden yellow", GOLDEN_YELLOW, (254.0, 198.0, 21.0, 1.0);
        "bright yellow", BRIGHT_YELLOW, (255.0, 253.0, 1.0, 1.0);
        "light lavender", LIGHT_LAVENDER, (223.0, 197.0, 254.0, 1.0);
        "umber", UMBER, (178.0, 100.0, 0.0, 1.0);
        "poop", POOP, (127.0, 94.0, 0.0, 1.0);
        "dark peach", DARK_PEACH, (222.0, 126.0, 93.0, 1.0);
        "jungle green", JUNGLE_GREEN, (4.0, 130.0, 67.0, 1.0);
        "eggshell", EGGSHELL, (255.0, 255.0, 212.0, 1.0);
        "denim", DENIM, (59.0, 99.0, 140.0, 1.0);
        "yellow brown", YELLOW_BROWN, (183.0, 148.0, 0.0, 1.0);
        "dull purple", DULL_PURPLE, (132.0, 89.0, 126.0, 1.0);
        "chocolate brown", CHOCOLATE_BROWN, (65.0, 25.0, 0.0, 1.0);
        "wine red", WINE_RED, (123.0, 3.0, 35.0, 1.0);
        "neon blue", NEON_BLUE, (4.0, 217.0, 255.0, 1.0);
        "dirty green", DIRTY_GREEN, (102.0, 126.0, 44.0, 1.0);
        "light tan", LIGHT_TAN, (251.0, 238.0, 172.0, 1.0);
        "ice blue", ICE_BLUE, (215.0, 255.0, 254.0, 1.0);
        "cadet blue", CADET_BLUE, (78.0, 116.0, 150.0, 1.0);
        "dark mauve", DARK_MAUVE, (135.0, 76.0, 98.0, 1.0);
        "very light blue", VERY_LIGHT_BLUE, (213.0, 255.0, 255.0, 1.0);
        "grey purple", GREY_PURPLE, (130.0, 109.0, 140.0, 1.0);
        "pastel pink", PASTEL_PINK, (255.0, 186.0, 205.0, 1.0);
        "very light green", VERY_LIGHT_GREEN, (209.0, 255.0, 189.0, 1.0);
        "dark sky blue", DARK_SKY_BLUE, (68.0, 142.0, 228.0, 1.0);
        "evergreen", EVERGREEN, (5.0, 71.0, 42.0, 1.0);
        "dull pink", DULL_PINK, (213.0, 134.0, 157.0, 1.0);
        "aubergine", AUBERGINE, (61.0, 7.0, 52.0, 1.0);
        "mahogany", MAHOGANY, (74.0, 1.0, 0.0, 1.0);
        "reddish orange", REDDISH_ORANGE, (248.0, 72.0, 28.0, 1.0);
        "deep green", DEEP_GREEN, (2.0, 89.0, 15.0, 1.0);
        "vomit green", VOMIT_GREEN, (137.0, 162.0, 3.0, 1.0);
        "purple pink", PURPLE_PINK, (224.0, 63.0, 216.0, 1.0);
        "dusty pink", DUSTY_PINK, (213.0, 138.0, 148.0, 1.0);
        "faded green", FADED_GREEN, (123.0, 178.0, 116.0, 1.0);
        "camo green", CAMO_GREEN, (82.0, 101.0, 37.0, 1.0);
        "pinky purple", PINKY_PURPLE, (201.0, 76.0, 190.0, 1.0);
        "pink purple", PINK_PURPLE, (219.0, 75.0, 218.0, 1.0);
        "brownish red", BROWNISH_RED, (158.0, 54.0, 35.0, 1.0);
        "dark rose", DARK_ROSE, (181.0, 72.0, 93.0, 1.0);
        "mud", MUD, (115.0, 92.0, 18.0, 1.0);
        "brownish", BROWNISH, (156.0, 109.0, 87.0, 1.0);
        "emerald green", EMERALD_GREEN, (2.0, 143.0, 30.0, 1.0);
        "pale brown", PALE_BROWN, (177.0, 145.0, 110.0, 1.0);
        "dull blue", DULL_BLUE, (73.0, 117.0, 156.0, 1.0);
        "burnt umber", BURNT_UMBER, (160.0, 69.0, 14.0, 1.0);
        "medium green", MEDIUM_GREEN, (57.0, 173.0, 72.0, 1.0);
        "clay", CLAY, (182.0, 106.0, 80.0, 1.0);
        "light aqua", LIGHT_AQUA, (140.0, 255.0, 219.0, 1.0);
        "light olive green", LIGHT_OLIVE_GREEN, (164.0, 190.0, 92.0, 1.0);
        "brownish orange", BROWNISH_ORANGE, (203.0, 119.0, 35.0, 1.0);
        "dark aqua", DARK_AQUA, (5.0, 105.0, 107.0, 1.0);
        "purplish pink", PURPLISH_PINK, (206.0, 93.0, 174.0, 1.0);
        "dark salmon", DARK_SALMON, (200.0, 90.0, 83.0, 1.0);
        "greenish grey", GREENISH_GREY, (150.0, 174.0, 141.0, 1.0);
        "jade", JADE, (31.0, 167.0, 116.0, 1.0);
        "ugly green", UGLY_GREEN, (122.0, 151.0, 3.0, 1.0);
        "dark beige", DARK_BEIGE, (172.0, 147.0, 98.0, 1.0);
        "emerald", EMERALD, (1.0, 160.0, 73.0, 1.0);
        "pale red", PALE_RED, (217.0, 84.0, 77.0, 1.0);
        "light magenta", LIGHT_MAGENTA, (250.0, 95.0, 247.0, 1.0);
        "sky", SKY, (130.0, 202.0, 252.0, 1.0);
        "light cyan", LIGHT_CYAN, (172.0, 255.0, 252.0, 1.0);
        "yellow orange", YELLOW_ORANGE, (252.0, 176.0, 1.0, 1.0);
        "reddish purple", REDDISH_PURPLE, (145.0, 9.0, 81.0, 1.0);
        "reddish pink", REDDISH_PINK, (254.0, 44.0, 84.0, 1.0);
        "orchid", ORCHID, (200.0, 117.0, 196.0, 1.0);
        "dirty yellow", DIRTY_YELLOW, (205.0, 197.0, 10.0, 1.0);
        "orange red", ORANGE_RED, (253.0, 65.0, 30.0, 1.0);
        "deep red", DEEP_RED, (154.0, 2.0, 0.0, 1.0);
        "orange brown", ORANGE_BROWN, (190.0, 100.0, 0.0, 1.0);
        "cobalt blue", COBALT_BLUE, (3.0, 10.0, 167.0, 1.0);
        "neon pink", NEON_PINK, (254.0, 1.0, 154.0, 1.0);
        "rose pink", ROSE_PINK, (247.0, 135.0, 154.0, 1.0);
        "greyish purple", GREYISH_PURPLE, (136.0, 113.0, 145.0, 1.0);
        "raspberry", RASPBERRY, (176.0, 1.0, 73.0, 1.0);
        "aqua green", AQUA_GREEN, (18.0, 225.0, 147.0, 1.0);
        "salmon pink", SALMON_PINK, (254.0, 123.0, 124.0, 1.0);
        "tangerine", TANGERINE, (255.0, 148.0, 8.0, 1.0);
        "brownish green", BROWNISH_GREEN, (106.0, 110.0, 9.0, 1.0);
        "red brown", RED_BROWN, (139.0, 46.0, 22.0, 1.0);
        "greenish brown", GREENISH_BROWN, (105.0, 97.0, 18.0, 1.0);
        "pumpkin", PUMPKIN, (225.0, 119.0, 1.0, 1.0);
        "pine green", PINE_GREEN, (10.0, 72.0, 30.0, 1.0);
        "charcoal", CHARCOAL, (52.0, 56.0, 55.0, 1.0);
        "baby pink", BABY_PINK, (255.0, 183.0, 206.0, 1.0);
        "cornflower", CORNFLOWER, (106.0, 121.0, 247.0, 1.0);
        "blue violet", BLUE_VIOLET, (93.0, 6.0, 233.0, 1.0);
        "chocolate", CHOCOLATE, (61.0, 28.0, 2.0, 1.0);
        "greyish green", GREYISH_GREEN, (130.0, 166.0, 125.0, 1.0);
        "scarlet", SCARLET, (190.0, 1.0, 25.0, 1.0);
        "green yellow", GREEN_YELLOW, (201.0, 255.0, 39.0, 1.0);
        "dark olive", DARK_OLIVE, (55.0, 62.0, 2.0, 1.0);
        "sienna", SIENNA, (169.0, 86.0, 30.0, 1.0);
        "pastel purple", PASTEL_PURPLE, (202.0, 160.0, 255.0, 1.0);
        "terracotta", TERRACOTTA, (202.0, 102.0, 65.0, 1.0);
        "aqua blue", AQUA_BLUE, (2.0, 216.0, 233.0, 1.0);
        "sage green", SAGE_GREEN, (136.0, 179.0, 120.0, 1.0);
        "blood red", BLOOD_RED, (152.0, 0.0, 2.0, 1.0);
        "deep pink", DEEP_PINK, (203.0, 1.0, 98.0, 1.0);
        "grass", GRASS, (92.0, 172.0, 45.0, 1.0);
        "moss", MOSS, (118.0, 153.0, 88.0, 1.0);
        "pastel blue", PASTEL_BLUE, (162.0, 191.0, 254.0, 1.0);
        "bluish green", BLUISH_GREEN, (16.0, 166.0, 116.0, 1.0);
        "green blue", GREEN_BLUE, (6.0, 180.0, 139.0, 1.0);
        "dark tan", DARK_TAN, (175.0, 136.0, 74.0, 1.0);
        "greenish blue", GREENISH_BLUE, (11.0, 139.0, 135.0, 1.0);
        "pale orange", PALE_ORANGE, (255.0, 167.0, 86.0, 1.0);
        "vomit", VOMIT, (162.0, 164.0, 21.0, 1.0);
        "forrest green", FORREST_GREEN, (21.0, 68.0, 6.0, 1.0);
        "dark lavender", DARK_LAVENDER, (133.0, 103.0, 152.0, 1.0);
        "dark violet", DARK_VIOLET, (52.0, 1.0, 63.0, 1.0);
        "purple blue", PURPLE_BLUE, (99.0, 45.0, 233.0, 1.0);
        "dark cyan", DARK_CYAN, (10.0, 136.0, 138.0, 1.0);
        "olive drab", OLIVE_DRAB, (111.0, 118.0, 50.0, 1.0);
        "pinkish", PINKISH, (212.0, 106.0, 126.0, 1.0);
        "cobalt", COBALT, (30.0, 72.0, 143.0, 1.0);
        "neon purple", NEON_PURPLE, (188.0, 19.0, 254.0, 1.0);
        "light turquoise", LIGHT_TURQUOISE, (126.0, 244.0, 204.0, 1.0);
        "apple green", APPLE_GREEN, (118.0, 205.0, 38.0, 1.0);
        "dull green", DULL_GREEN, (116.0, 166.0, 98.0, 1.0);
        "wine", WINE, (128.0, 1.0, 63.0, 1.0);
        "powder blue", POWDER_BLUE, (177.0, 209.0, 252.0, 1.0);
        "off white", OFF_WHITE, (255.0, 255.0, 228.0, 1.0);
        "electric blue", ELECTRIC_BLUE, (6.0, 82.0, 255.0, 1.0);
        "dark turquoise", DARK_TURQUOISE, (4.0, 92.0, 90.0, 1.0);
        "blue purple", BLUE_PURPLE, (87.0, 41.0, 206.0, 1.0);
        "azure", AZURE, (6.0, 154.0, 243.0, 1.0);
        "bright red", BRIGHT_RED, (255.0, 0.0, 13.0, 1.0);
        "pinkish red", PINKISH_RED, (241.0, 12.0, 69.0, 1.0);
        "cornflower blue", CORNFLOWER_BLUE, (81.0, 112.0, 215.0, 1.0);
        "light olive", LIGHT_OLIVE, (172.0, 191.0, 105.0, 1.0);
        "grape", GRAPE, (108.0, 52.0, 97.0, 1.0);
        "greyish blue", GREYISH_BLUE, (94.0, 129.0, 157.0, 1.0);
        "purplish blue", PURPLISH_BLUE, (96.0, 30.0, 249.0, 1.0);
        "yellowish green", YELLOWISH_GREEN, (176.0, 221.0, 22.0, 1.0);
        "greenish yellow", GREENISH_YELLOW, (205.0, 253.0, 2.0, 1.0);
        "medium blue", MEDIUM_BLUE, (44.0, 111.0, 187.0, 1.0);
        "dusty rose", DUSTY_ROSE, (192.0, 115.0, 122.0, 1.0);
        "light violet", LIGHT_VIOLET, (214.0, 180.0, 252.0, 1.0);
        "midnight blue", MIDNIGHT_BLUE, (2.0, 0.0, 53.0, 1.0);
        "bluish purple", BLUISH_PURPLE, (112.0, 59.0, 231.0, 1.0);
        "red orange", RED_ORANGE, (253.0, 60.0, 6.0, 1.0);
        "dark magenta", DARK_MAGENTA, (150.0, 0.0, 86.0, 1.0);
        "greenish", GREENISH, (64.0, 163.0, 104.0, 1.0);
        "ocean blue", OCEAN_BLUE, (3.0, 113.0, 156.0, 1.0);
        "coral", CORAL, (252.0, 90.0, 80.0, 1.0);
        "cream", CREAM, (255.0, 255.0, 194.0, 1.0);
        "reddish brown", REDDISH_BROWN, (127.0, 43.0, 10.0, 1.0);
        "burnt sienna", BURNT_SIENNA, (176.0, 78.0, 15.0, 1.0);
        "brick", BRICK, (160.0, 54.0, 35.0, 1.0);
        "sage", SAGE, (135.0, 174.0, 115.0, 1.0);
        "grey green", GREY_GREEN, (120.0, 155.0, 115.0, 1.0);
        "robin's egg blue", ROBINS_EGG_BLUE, (152.0, 239.0, 249.0, 1.0);
        "moss green", MOSS_GREEN, (101.0, 139.0, 56.0, 1.0);
        "steel blue", STEEL_BLUE, (90.0, 125.0, 154.0, 1.0);
        "eggplant", EGGPLANT, (56.0, 8.0, 53.0, 1.0);
        "light yellow", LIGHT_YELLOW, (255.0, 254.0, 122.0, 1.0);
        "leaf green", LEAF_GREEN, (92.0, 169.0, 4.0, 1.0);
        "light grey", LIGHT_GREY, (216.0, 220.0, 214.0, 1.0);
        "puke", PUKE, (165.0, 165.0, 2.0, 1.0);
        "pinkish purple", PINKISH_PURPLE, (214.0, 72.0, 215.0, 1.0);
        "sea blue", SEA_BLUE, (4.0, 116.0, 149.0, 1.0);
        "pale purple", PALE_PURPLE, (183.0, 144.0, 212.0, 1.0);
        "slate blue", SLATE_BLUE, (91.0, 124.0, 153.0, 1.0);
        "blue grey", BLUE_GREY, (96.0, 124.0, 142.0, 1.0);
        "hunter green", HUNTER_GREEN, (11.0, 64.0, 8.0, 1.0);
        "fuchsia", FUCHSIA, (237.0, 13.0, 217.0, 1.0);
        "crimson", CRIMSON, (140.0, 0.0, 15.0, 1.0);
        "pale yellow", PALE_YELLOW, (255.0, 255.0, 132.0, 1.0);
        "ochre", OCHRE, (191.0, 144.0, 5.0, 1.0);
        "mustard yellow", MUSTARD_YELLOW, (210.0, 189.0, 10.0, 1.0);
        "light red", LIGHT_RED, (255.0, 71.0, 76.0, 1.0);
        "cerulean", CERULEAN, (4.0, 133.0, 209.0, 1.0);
        "pale pink", PALE_PINK, (255.0, 207.0, 220.0, 1.0);
        "deep blue", DEEP_BLUE, (4.0, 2.0, 115.0, 1.0);
        "rust", RUST, (168.0, 60.0, 9.0, 1.0);
        "light teal", LIGHT_TEAL, (144.0, 228.0, 193.0, 1.0);
        "slate", SLATE, (81.0, 101.0, 114.0, 1.0);
        "goldenrod", GOLDENROD, (250.0, 194.0, 5.0, 1.0);
        "dark yellow", DARK_YELLOW, (213.0, 182.0, 10.0, 1.0);
        "dark grey", DARK_GREY, (54.0, 55.0, 55.0, 1.0);
        "army green", ARMY_GREEN, (75.0, 93.0, 22.0, 1.0);
        "grey blue", GREY_BLUE, (107.0, 139.0, 164.0, 1.0);
        "seafoam", SEAFOAM, (128.0, 249.0, 173.0, 1.0);
        "puce", PUCE, (165.0, 126.0, 82.0, 1.0);
        "spring green", SPRING_GREEN, (169.0, 249.0, 113.0, 1.0);
        "dark orange", DARK_ORANGE, (198.0, 81.0, 2.0, 1.0);
        "sand", SAND, (226.0, 202.0, 118.0, 1.0);
        "pastel green", PASTEL_GREEN, (176.0, 255.0, 157.0, 1.0);
        "mint", MINT, (159.0, 254.0, 176.0, 1.0);
        "light orange", LIGHT_ORANGE, (253.0, 170.0, 72.0, 1.0);
        "bright pink", BRIGHT_PINK, (254.0, 1.0, 177.0, 1.0);
        "chartreuse", CHARTREUSE, (193.0, 248.0, 10.0, 1.0);
        "deep purple", DEEP_PURPLE, (54.0, 1.0, 63.0, 1.0);
        "dark brown", DARK_BROWN, (52.0, 28.0, 2.0, 1.0);
        "taupe", TAUPE, (185.0, 162.0, 129.0, 1.0);
        "pea green", PEA_GREEN, (142.0, 171.0, 18.0, 1.0);
        "puke green", PUKE_GREEN, (154.0, 174.0, 7.0, 1.0);
        "kelly green", KELLY_GREEN, (2.0, 171.0, 46.0, 1.0);
        "seafoam green", SEAFOAM_GREEN, (122.0, 249.0, 171.0, 1.0);
        "blue green", BLUE_GREEN, (19.0, 126.0, 109.0, 1.0);
        "khaki", KHAKI, (170.0, 166.0, 98.0, 1.0);
        "burgundy", BURGUNDY, (97.0, 0.0, 35.0, 1.0);
        "dark teal", DARK_TEAL, (1.0, 77.0, 78.0, 1.0);
        "brick red", BRICK_RED, (143.0, 20.0, 2.0, 1.0);
        "royal purple", ROYAL_PURPLE, (75.0, 0.0, 110.0, 1.0);
        "plum", PLUM, (88.0, 15.0, 65.0, 1.0);
        "mint green", MINT_GREEN, (143.0, 255.0, 159.0, 1.0);
        "gold", GOLD, (219.0, 180.0, 12.0, 1.0);
        "baby blue", BABY_BLUE, (162.0, 207.0, 254.0, 1.0);
        "yellow green", YELLOW_GREEN, (192.0, 251.0, 45.0, 1.0);
        "bright purple", BRIGHT_PURPLE, (190.0, 3.0, 253.0, 1.0);
        "dark red", DARK_RED, (132.0, 0.0, 0.0, 1.0);
        "pale blue", PALE_BLUE, (208.0, 254.0, 254.0, 1.0);
        "grass green", GRASS_GREEN, (63.0, 155.0, 11.0, 1.0);
        "navy", NAVY, (1.0, 21.0, 62.0, 1.0);
        "aquamarine", AQUAMARINE, (4.0, 216.0, 178.0, 1.0);
        "burnt orange", BURNT_ORANGE, (192.0, 78.0, 1.0, 1.0);
        "neon green", NEON_GREEN, (12.0, 255.0, 12.0, 1.0);
        "bright blue", BRIGHT_BLUE, (1.0, 101.0, 252.0, 1.0);
        "rose", ROSE, (207.0, 98.0, 117.0, 1.0);
        "light pink", LIGHT_PINK, (255.0, 209.0, 223.0, 1.0);
        "mustard", MUSTARD, (206.0, 179.0, 1.0, 1.0);
        "indigo", INDIGO, (56.0, 2.0, 130.0, 1.0);
        "lime", LIME, (170.0, 255.0, 50.0, 1.0);
        "sea green", SEA_GREEN, (83.0, 252.0, 161.0, 1.0);
        "periwinkle", PERIWINKLE, (142.0, 130.0, 254.0, 1.0);
        "dark pink", DARK_PINK, (203.0, 65.0, 107.0, 1.0);
        "olive green", OLIVE_GREEN, (103.0, 122.0, 4.0, 1.0);
        "peach", PEACH, (255.0, 176.0, 124.0, 1.0);
        "pale green", PALE_GREEN, (199.0, 253.0, 181.0, 1.0);
        "light brown", LIGHT_BROWN, (173.0, 129.0, 80.0, 1.0);
        "hot pink", HOT_PINK, (255.0, 2.0, 141.0, 1.0);
        "lilac", LILAC, (206.0, 162.0, 253.0, 1.0);
        "navy blue", NAVY_BLUE, (0.0, 17.0, 70.0, 1.0);
        "royal blue", ROYAL_BLUE, (5.0, 4.0, 170.0, 1.0);
        "beige", BEIGE, (230.0, 218.0, 166.0, 1.0);
        "salmon", SALMON, (255.0, 121.0, 108.0, 1.0);
        "olive", OLIVE, (110.0, 117.0, 14.0, 1.0);
        "maroon", MAROON, (101.0, 0.0, 33.0, 1.0);
        "bright green", BRIGHT_GREEN, (1.0, 255.0, 7.0, 1.0);
        "dark purple", DARK_PURPLE, (53.0, 6.0, 62.0, 1.0);
        "mauve", MAUVE, (174.0, 113.0, 129.0, 1.0);
        "forest green", FOREST_GREEN, (6.0, 71.0, 12.0, 1.0);
        "aqua", AQUA, (19.0, 234.0, 201.0, 1.0);
        "cyan", CYAN, (0.0, 255.0, 255.0, 1.0);
        "tan", TAN, (209.0, 178.0, 111.0, 1.0);
        "dark blue", DARK_BLUE, (0.0, 3.0, 91.0, 1.0);
        "lavender", LAVENDER, (199.0, 159.0, 239.0, 1.0);
        "turquoise", TURQUOISE, (6.0, 194.0, 172.0, 1.0);
        "dark green", DARK_GREEN, (3.0, 53.0, 0.0, 1.0);
        "violet", VIOLET, (154.0, 14.0, 234.0, 1.0);
        "light purple", LIGHT_PURPLE, (191.0, 119.0, 246.0, 1.0);
        "lime green", LIME_GREEN, (137.0, 254.0, 5.0, 1.0);
    }
}
