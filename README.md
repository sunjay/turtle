# turtle

[![Crates.io](https://img.shields.io/crates/v/turtle.svg)](https://crates.io/crates/turtle)
[![Crates.io](https://img.shields.io/crates/l/turtle.svg)](https://crates.io/crates/turtle)
[![Crates.io](https://img.shields.io/crates/d/turtle.svg)](https://crates.io/crates/turtle)
[![Build Status](https://travis-ci.org/sunjay/turtle.svg?branch=master)](https://travis-ci.org/sunjay/turtle)
[![Build status](https://ci.appveyor.com/api/projects/status/scg3x6ti49o8sdii/branch/master?svg=true)](https://ci.appveyor.com/project/sunjay/turtle/branch/master)
[![Gitter](https://img.shields.io/gitter/room/nwjs/nw.js.svg)](https://gitter.im/rust-turtle/discuss)

Turtle graphics in [Rust][rust]. This library is a tool for teaching programming
by drawing pictures. Learning this way is both fun and interesting for students
of all ages!

**The idea:** You control a turtle with a pen tied to its tail. As it moves
across the screen, it draws the path that it follows. You can use this to draw
any picture you want just by moving the turtle across the screen.

![turtle start](https://github.com/sunjay/turtle/raw/master/turtle1.png)
![turtle moved](https://github.com/sunjay/turtle/raw/master/turtle2.png)

## Example

As a simple example, you can draw a circle with only the following code:

```rust
extern crate turtle;

use turtle::Turtle;

fn main() {
    let mut turtle = Turtle::new();

    for _ in 0..360 {
        // Move forward three steps
        turtle.forward(3.0);
        // Rotate to the right (clockwise) by 1 degree
        turtle.right(1.0);
    }
}
```

This will produce the following:

![turtle drawing a circle](https://github.com/sunjay/turtle/raw/master/turtle_circle.gif)

See the [`examples/`](https://github.com/sunjay/turtle/raw/master/examples) directory for more examples of how to use this
library.

## Need help?

The following are some resources you can use to find help when you run into a
problem. The links are listed in the order you should try each one, but feel
free to come to the [Turtle Gitter] anytime if you are lost.

* **Help with the Rust Language** - [Google], [Stack Overflow], [Rust Users Forum], [Turtle Gitter]
* **Help with Turtle** - [Stack Overflow], [Turtle Gitter], [Google], [Rust Users Forum]
* **Found a bug?** - [Open an issue][issues] (feel free to ask about your bug in the [Turtle Gitter])

[Google]: http://google.com/
[Stack Overflow]: https://stackoverflow.com/
[Rust Users Forum]: https://users.rust-lang.org/
[Turtle Gitter]: https://gitter.im/rust-turtle/discuss
[issues]: https://github.com/sunjay/turtle/issues

## Inspiration

This is inspired by the [Logo educational programming language][logo-lang] and
is featured in many programming languages. For example, [Python][python] comes
with a [built-in turtle module][turtle-py]. This library is based on that
module, but uses Rust conventions and best practices to accomplish the same
goals.

[rust]: https://www.rust-lang.org/
[logo-lang]: https://en.wikipedia.org/wiki/Logo_(programming_language)
[python]: https://www.python.org/
[turtle-py]: https://docs.python.org/2/library/turtle.html
