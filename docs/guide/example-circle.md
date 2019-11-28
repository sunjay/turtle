---
layout: docs
title: "Example: Circle"
docs_title: "Example: Circle"
permalink: /guide/example-circle/
---

So far, we've only been generating drawings with straight lined shapes. This
lesson will cover how we can use turtle to draw curves and circles. We'll start
by drawing some arcs, and then build up to the point where we can draw a full
circle.

**Here's the trick:** We can draw a line that appears to be curved by drawing a
lot of much shorter lines and rotating only a little bit between each one. We'll
illustrate this principle in detail in the next section.

## Drawing Arcs

Let's start with the following program:

```rust
use turtle::Turtle;

fn main() {
    let mut turtle = Turtle::new();

    for _ in 0..3 {
        turtle.forward(90.0);
        turtle.right(30.0);
    }
}
```

Here, we move forward

```rust
use turtle::Turtle;

fn main() {
    let mut turtle = Turtle::new();

    for _ in 0..5 {
        turtle.forward(54.0);
        turtle.right(18.0);
    }
}
```

```rust
use turtle::Turtle;

fn main() {
    let mut turtle = Turtle::new();

    for _ in 0..9 {
        turtle.forward(30.0);
        turtle.right(10.0);
    }
}
```

```rust
use turtle::Turtle;

fn main() {
    let mut turtle = Turtle::new();

    for _ in 0..18 {
        turtle.forward(15.0);
        turtle.right(5.0);
    }
}
```

- The trick: if we keep drawing smaller and smaller curved lines, we can get
  something that is indistinguishable from an actual curve
- Show variations of a curve with more lines that become smaller and smaller
  - Notice that a lower number of lines is a worse approximation of the arc
  - Very noticable change in final position if you look at the gif
- Curvy lines via very small steps and rotation
- Curve -> circle
- To draw a quarter of a circle, you need to go 90 degrees - explain why visually
- Go over code from circle example

```rust
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

## Exercises

These exercises are designed to help you reinforce what you've learned
throughout this lesson. All exercises are completely **optional**. If you get
stuck on an exercise, it is totally okay to move on and come back to it later.

If you need help, see the [Getting Help] section of the guide.

- (*) change radius of circle
- (*) change angle to something bigger or smaller without changing anything
  else and see what happens
- (**) draw a circle backwards
- (**) draw two circles side by side
- (**) draw half of a circle on the left and the other half on the right (s-curve)
- (***) draw a flower of s-curves
- (***) draw a flower with circular petals with a stem
- (***) draw a flower with oval petals with a stem and oval leaves
  - Bonus: color the flower with [`begin_fill`] and [`end_fill`]
- (***) circle with dashed lines
  - use [`pen_up`] and [`pen_down`]
