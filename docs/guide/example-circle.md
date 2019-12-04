---
layout: docs
title: "Example: Circle"
docs_title: "Example: Circle"
permalink: /guide/example-circle/
---

So far, we've only been generating drawings with straight lined shapes. This
lesson will cover how we can use turtle to draw curves and circles. We'll start
by drawing some arcs, and then build up to the point where we can draw a full,
360&deg; circle.

## Drawing Arcs

**Key Concept:** We can draw a line that appears to be curved by drawing a lot
of much shorter lines and rotating only a little bit between each one.

To demonstrate this, let's look at a program that draws the following 90&deg;
arc. This arc covers one quarter of 360&deg;, so this is often called a "quarter
arc".

![turtle 90 degree arc]({% link assets/images/guide/example-circle/arc.png %}){: .figborder .figlimitsize}

To figure out how to draw this quarter arc, let's look at the differences
between the start position of the turtle (shown below) and the image we're
trying to create above.

![turtle initial]({% link assets/images/guide/example-circle/initial-turtle.png %}){: .figborder .figlimitsize}

Notice that the turtle appears to go from facing the top of the window to facing
the right. It has moved along a quarter of a circle to get there. The program to
generate this arc is as follows:

```rust
use turtle::Turtle;

fn main() {
    let mut turtle = Turtle::new();

    for _ in 0..90 {
        turtle.forward(3.0);
        turtle.right(1.0);
    }
}
```

As you can see, we're doing exactly what we said we would do earlier: we're
drawing many very small lines and slightly adjusting our angle in every
iteration. We instruct the turtle to move 3 steps forward and turn 1&deg; to
the right every time.

To confirm that this is actually working as described, let's try decreasing the
number of iterations and increasing the size of the lines we're drawing. To draw
longer lines, we'll take more steps. We'll also increase the amount we're turning
so that we still reach 90&deg; by the time we're done iterating.

Here's the drawing that gets created with 3 iterations of the turtle drawing a
line for 90 steps and turning 30&deg; after each line.

![turtle 3 line arc]({% link assets/images/guide/example-circle/arc-3-lines.png %}){: .figborder .figlimitsize}

This is the code that generates this image:

```rust
use turtle::Turtle;

fn main() {
    let mut turtle = Turtle::new();

    // 30 degrees * 3 = 90 degrees
    for _ in 0..3 {
        turtle.forward(90.0);
        turtle.right(30.0);
    }
}
```

You can see that we're still turning the turtle 90&deg; in total, but the
curve doesn't exactly follow the same circular arc we were getting before. To
improve this, let's try 5 iterations with an 18&deg; turn every time:

![turtle 5 line arc]({% link assets/images/guide/example-circle/arc-5-lines.png %}){: .figborder .figlimitsize}

Here's the code that generates that image:

```rust
use turtle::Turtle;

fn main() {
    let mut turtle = Turtle::new();

    // 18 degrees * 5 = 90 degrees
    for _ in 0..5 {
        turtle.forward(54.0);
        turtle.right(18.0);
    }
}
```

This gets us a little closer! If we increase it to 9 iterations with a 10&deg;
turn, we get the following image:

![turtle 9 line arc]({% link assets/images/guide/example-circle/arc-9-lines.png %}){: .figborder .figlimitsize}

The code for this image:

```rust
use turtle::Turtle;

fn main() {
    let mut turtle = Turtle::new();

    // 10 degrees * 9 = 90 degrees
    for _ in 0..9 {
        turtle.forward(30.0);
        turtle.right(10.0);
    }
}
```

At this point, it's almost indistinguishable. However, if you look close enough,
you can still tell that there are 9 individual lines being drawn here. We can
make the curve even smoother using 18 iterations with a 5&deg; turn every time:

![turtle 18 line arc]({% link assets/images/guide/example-circle/arc-18-lines.png %}){: .figborder .figlimitsize}

The code for this image:

```rust
use turtle::Turtle;

fn main() {
    let mut turtle = Turtle::new();

    // 5 degrees * 18 = 90 degrees
    for _ in 0..18 {
        turtle.forward(15.0);
        turtle.right(5.0);
    }
}
```

With this many iterations, we get pretty close. Just to illustrate how much of
a difference each increase in iterations makes, here's a GIF that shows us
getting closer and closer to the final quarter arc:

![turtle arc progression gif]({% link assets/images/guide/example-circle/arc-lines.gif %}){: .figborder .figlimitsize}

The number of iterations you need for your own drawings will depend on the size
of the arc you are creating. To be safe, you can draw the most reliable and
accurate arc: 90 iterations with a 1&deg; turn every time:

```rust
use turtle::Turtle;

fn main() {
    let mut turtle = Turtle::new();

    for _ in 0..90 {
        turtle.forward(3.0);
        turtle.right(1.0);
    }
}
```

This is the same program from above that gets us the 90&deg; arc we initially
set out to create.

![turtle 90 degree arc]({% link assets/images/guide/example-circle/arc.png %}){: .figborder .figlimitsize}

## Drawing Circles

Now that we've figured out how to draw a 90&deg; arc, all we have to do to get a
circle is draw 4 of them (90&deg; * 4 = 360&deg;). Here's a program you could
use to do that:

```rust
use turtle::Turtle;

fn main() {
    let mut turtle = Turtle::new();

    // Try to avoid programs that look like this
    for _ in 0..4 {
        for _ in 0..90 {
            turtle.forward(3.0);
            turtle.right(1.0);
        }
    }
}
```

**This is probably not the program you want to write!** While this program will
work, let's see what we can do if we think through the problem given what we've
learned already.

We know that we want to draw a full circle. A full circle has 360&deg; in it. We
know that if we rotate the turtle 1&deg; for 90 iterations, we'll draw a quarter
arc. Extending that idea, we should try to write a program that performs 360
iterations, rotating the turtle 1&deg; every time.

That's how we get to the program below:

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

This produces the complete circle shown below:

![turtle circle]({% link assets/images/guide/example-circle/circle.png %}){: .figborder .figlimitsize}

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
- (***) draw the sun as a circle with triangles around it
  - Bonus: color the sun with [`begin_fill`] and [`end_fill`]
- (***) circle with dashed lines
  - use [`pen_up`] and [`pen_down`]

[Getting Help]: {% link guide/getting-help.md %}
[`begin_fill`]: https://docs.rs/turtle/{{ site.data.lib.latest.version }}/turtle/struct.Turtle.html#method.begin_fill
[`end_fill`]: https://docs.rs/turtle/{{ site.data.lib.latest.version }}/turtle/struct.Turtle.html#method.end_fill
[`pen_up`]: https://docs.rs/turtle/{{ site.data.lib.latest.version }}/turtle/struct.Turtle.html#method.pen_up
[`pen_down`]: https://docs.rs/turtle/{{ site.data.lib.latest.version }}/turtle/struct.Turtle.html#method.pen_down
