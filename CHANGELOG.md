# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

* New `new` and `add_turtle` methods on `Drawing` allow you to create a drawing
  and a turtle at the center of it
  * This API will eventually enable us to have support for multiple turtles
    drawing at the same time
* First few pages of the new guide have been published on https://turtle.rs/guide

### Changed

* The recommended way to add the turtle crate to your `Cargo.toml` file has
  changed to include `opt-level = 3` for dependencies only. This is necessary
  because debug performance of turtle and other crates is not very good. This
  change increases the initial build time for any application using the turtle
  crate as a dependency, but should not affect incremental builds after that.
  See the [docs] or the [guide] for more detailed instructions.
  ```toml
  [profile.dev.package."*"]
  opt-level = 3
  ```
* Closing the window no longer calls `process::exit`. That means that you can
  create multiple windows which can be closed independently. If you continue to
  use a turtle after the window has closed, the thread will panic since there is
  no where to continue drawing the picture. This panic stops the current thread,
  but is not necessarily an "error". Make sure to use `turtle::start()` if you
  plan to call `Turtle::new()` or `Drawing::new()` in a separate thread.
* The `Event` enum has changed considerably now that we've moved to glutin, but
  most of the same events are still available with minor changes to existing code
* `Event` and some of its related enums are now marked [`#[non_exhaustive]`](https://doc.rust-lang.org/reference/attributes/type_system.html#the-non_exhaustive-attribute).
  That means that we will be able to add further variants in the future if
  needed without it being a breaking change.
* `Event` and its related enums are now entirely part of the turtle crate. We no
  longer re-export parts of the `piston_window` crate (and in fact do not rely
  on `piston_window` at all anymore).
* The `event` module, `Event`, and all related enums are now unstable and you
  need to use the `unstable` feature in your `Cargo.toml` to enable them. These
  may change as we figure out what event handling should look like in the turtle
  crate.
* The pen used by the turtle is now circular and has circular ends at higher pen
  thicknesses. This matches the behaviour of the Python turtle module and is
  useful for being able to smoothly draw curves and circles using the turtle crate.
* The `save_svg` method on `Drawing` returns a `Result` type now
* The `set_pen_size` method no doubles the given thickness value
  * This means that you may have to adjust your program to set the pen thickness
    to double the value you were setting previously
* The `Speed` levels change the amount of time it takes to draw something linearly
  * This may change again in the future as we decide how to best use the different
    speed levels
* The `set_fill_color` method updates the currently filling shape, even after
  `begin_fill` has been called
  * Previously, you could only set the fill color *before* `begin_fill` was called
* The `home` method is no longer an instantaneous change in the turtle's position
  * To move the turtle instantly, use `turtle.set_speed("instant")`
  * To avoid drawing a line while moving back to the origin, use `turtle.pen_up()`
* The bits example in `examples/bits.rs` has been updated to bitvec 0.17

### Removed

* `drawing` and `drawing_mut` methods have been removed due to soundness issues
  that these methods would introduce when multiple turtle support is added.
  Use `Drawing::new()` and `drawing.add_turtle()` instead to get access to the
  drawing.

### Fixed

* [#30](https://github.com/sunjay/turtle/issues/30) - **potentially breaking:**
  fill polygons now start at the current position when `begin_fill()` is called.
  We previously had a bug where we would start at the end of the next line after
  `begin_fill()` was called.

## [1.0.0-rc.3] - 2019-12-11

### Added

* New `is_filling` method on `Turtle` struct
* New simplified `rand` module which provides a simple, stable interface for
  generating random numbers
* New `save_svg` method on `Drawing` for exporting the current drawing to SVG
* New geometric art example in `examples/geometric_art.rs`
* New star example in `examples/star2.rs` for drawing a star with any number of
  points
* New bits example in `examples/bits.rs` to visualize binary numbers using
  turtle and the bitvec crate

### Changed

* Crate now uses the 2018 edition of Rust
* `turtle::rand::random_range` now has an inclusive upper bound

### Removed

* The `rand` crate is no longer re-exported from the `rand` module
* Methods from `rand` are no longer re-exported from the crate root
  * Import from the `rand` module instead

### Fixed

* [#92](https://github.com/sunjay/turtle/issues/92) - `wait_for_click` no longer
  results in very high CPU usage (fixed busy waiting)

## [1.0.0-rc.2] - 2018-08-15

### Added

* New website: https://turtle.rs
* New methods on `Color`
  * `rgb`
  * `rgba`
  * `hsl`
  * `hsla`
  * `hue`
  * `saturation`
  * `lightness`
  * `mix`
  * `rotate_hue`
  * `lighten`
  * `darken`
  * `saturate`
  * `desaturate`
  * `grayscale`
  * `complement`
  * `invert`
* LOGO interpreter example in `examples/logo_interpreter.rs` now supports the
  following additional commands:
  * `home`
  * `setx`
  * `sety`
  * `setheading` / `seth`
  * `showturtle` / `st`
  * `hideturtle` / `ht`
  * `clean`
  * `clearscreen` / `cs`
  * `pendown` / `pd`
  * `penup` / `pu`
  * `setpensize` / `setwidth` / `setpw`
* New colored spiral example in `examples/colored_spiral.rs`

### Changed

* squares example in `examples/squares.rs` now draws a colorful red/white flower
  with square petals

## [1.0.0-rc.1] - 2018-03-10

### Added

* New `Point` type that replaces the previous `Point = [f64; 2]` type alias
* New fractal tree example in `examples/tree.rs`
* New example in `examples/draw_turtle.rs` of drawing a child's drawing of a
  turtle

### Changed

* `0` can no longer be passed to `set_speed`, use `"instant"` or
  `Speed::instant()` instead

### Fixed

* [#63](https://github.com/sunjay/turtle/issues/63) - Disabling srgb to avoid
  pixel format errors

## [1.0.0-alpha.8] - 2017-12-22

### Added

* New `Drawing` struct to manage properties of the drawing/window
* New `drawing` and `drawing_mut` methods on the `Turtle` struct to allow you
  to access the new `Drawing` struct
* New methods on `Drawing` struct
  * `center`
  * `set_center`
  * `reset_center`
  * `title`
  * `set_title`
  * `size`
  * `set_size`
  * `reset_size`
  * `maximize` (unstable)
  * `unmaximize` (unstable)
  * `is_maximized` (unstable)
  * `enter_fullscreen`
  * `exit_fullscreen`
  * `is_fullscreen`
* New `Size` struct for use with `size` and `set_size` methods on `Drawing`
* `with_alpha` method to `Color` for creating a new color with a desired alpha
  value
* Added LOGO interpreter example in `examples/logo_interpreter.rs` with support
  for a few basic commands like `fd`/`forward`, `bk`/`back`, `lt`/`left`, and
  `rt`/`right`

### Changed

* The `background_color` and `set_background_color` methods on `Turtle` have
  moved to the new `Drawing` struct
  * Use `turtle.drawing().background_color()` instead of `turtle.background_color()`
  * Use `turtle.drawing_mut().set_background_color(...)` instead of `turtle.set_background_color(...)`
* The `poll_event` method on `Turtle` has moved to the new `Drawing` struct
  * Use `turtle.drawing_mut().poll_event()` instead of `turtle.poll_event()`
* Renamed fields of `WindowResized` event to `width` and `height` instead of
  `x` and `y`
* Code is now licensed under MPL 2.0 instead of MIT license

### Fixed

* [#8](https://github.com/sunjay/turtle/issues/8) - the `turn_towards` method
  now works correctly and as expected

## [1.0.0-alpha.7] - 2017-12-4

### Added

* New star example in `examples/empty_star.rs` that draws the outline of a star
  without needing any lines within it
* New dragon curve example in `examples/dragon.rs`

### Changed

* The maze example has moved from `examples/maze.rs` to `examples/maze.rs/main.rs`
  now that multi-file examples are supported in Rust 1.22

### Fixed

* [#27](https://github.com/sunjay/turtle/issues/27) - the turtle crate now has
  a two-process architecture which allows it to run on MacOS

## [1.0.0-alpha.6] - 2017-11-14

### Added

* New methods on the `Turtle` struct:
  * `go_to`
  * `set_x`
  * `set_y`
  * `set_heading`
  * `home`
  * `reset`
  * `wait`
* New `turtle::rand::random_range` function for easily generating a random value
  within a certain range
* New maze solving example in `examples/maze.rs`
* New snowflake example in `examples/snowflake.rs` using fractals
* New squares example in `examples/squares.rs` that creates a flower with square petals

### Changed

* Instead of re-exporting the `rand` crate from the root of the turtle crate,
  it is now exposed from its own separate `rand` module
  * Write `use turtle::rand::{Rand, Rng}` to use the new module

### Fixed

* Negative angles passed to `right()` and `left()` no longer result in very
  long animation times (caused by integer overflow)

## [1.0.0-alpha.5] - 2017-10-29

### Added

* Added more documentation and tests for methods on the `Turtle` struct

### Removed

* `to_absolute` and `to_rotation` methods on `Speed` are private now as they
  represent unstable implementation details that should not be relied on

## [1.0.0-alpha.4] - 2017-10-28

### Added

* Hexadecimal colors are now supported for use with the `Color` struct
* Floating point numbers (specifically `f64` values) can now be used in `set_speed`
* Added more documentation and tests for methods on the `Turtle` struct

## [1.0.0-alpha.3] - 2017-10-27

### Added

* Added `examples/nestedcubes.rs`
* Documentation and examples in crate root, `Turtle` struct, and `Color` struct

### Fixed

* [#5](https://github.com/sunjay/turtle/issues/5) - Using the line currently
  being drawn in the fill polygon

## [1.0.0-alpha.2] - 2017-10-21

### Added

* Added `clear()` method to `Turtle` struct

### Changed

* Renamed `visible()` method on the `Color` struct to `opaque`
* Renamed `invisible()` method on the `Color` struct to `transparent`

## [1.0.0-alpha.1] - 2017-10-20

### Fixed

* Using absolute URLs so that images in README.md work on crates.io

## [1.0.0-alpha.0] - 2017-10-20

Initial release of first version of turtle crate.

[Unreleased]: https://github.com/sunjay/turtle/compare/v1.0.0-rc.3...HEAD
[1.0.0-rc.3]: https://github.com/sunjay/turtle/compare/v1.0.0-rc.2...v1.0.0-rc.3
[1.0.0-rc.2]: https://github.com/sunjay/turtle/compare/v1.0.0-rc.1...v1.0.0-rc.2
[1.0.0-rc.1]: https://github.com/sunjay/turtle/compare/v1.0.0-alpha.8...v1.0.0-rc.1
[1.0.0-alpha.8]: https://github.com/sunjay/turtle/compare/v1.0.0-alpha.7...v1.0.0-alpha.8
[1.0.0-alpha.7]: https://github.com/sunjay/turtle/compare/v1.0.0-alpha.6...v1.0.0-alpha.7
[1.0.0-alpha.6]: https://github.com/sunjay/turtle/compare/v1.0.0-alpha.5...v1.0.0-alpha.6
[1.0.0-alpha.5]: https://github.com/sunjay/turtle/compare/v1.0.0-alpha.4...v1.0.0-alpha.5
[1.0.0-alpha.4]: https://github.com/sunjay/turtle/compare/v1.0.0-alpha.3...v1.0.0-alpha.4
[1.0.0-alpha.3]: https://github.com/sunjay/turtle/compare/v1.0.0-alpha.2...v1.0.0-alpha.3
[1.0.0-alpha.2]: https://github.com/sunjay/turtle/compare/v1.0.0-alpha.1...v1.0.0-alpha.2
[1.0.0-alpha.1]: https://github.com/sunjay/turtle/compare/v1.0.0-alpha.0...v1.0.0-alpha.1
[1.0.0-alpha.0]: https://github.com/sunjay/turtle/compare/2e9a4e93ea376404b313d7139a4d4cc23f46d4d3...v1.0.0-alpha.0

[docs]: https://docs.rs/turtle
[guide]: https://turtle.rs/guide/quickstart/
