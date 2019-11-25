---
layout: docs
title: "Example: Square"
docs_title: "Example: Square"
permalink: /guide/example-square/
---

In [Basic Movement], we learned about a number of different movement commands
like [`forward`], [`backward`], [`right`], and [`left`]. Then, we walked through
the process of figuring out how to draw a triangle. In this lesson, we'll go
through a similar process when figuring out how to draw a square. To start,
let's look at the final image we're trying to create:

![turtle square]({% link assets/images/guide/example-square/square.png %}){: .figborder .figlimitsize}

All squares have four sides. Each of their inner angles are 90 degrees. That
means that much like with the triangle, we'll be trying to draw our square by
drawing a line, rotating, and then drawing again until we complete the shape.

The question is, what should we use as the angle when we're turning? Remember:
the turtle always rotates from its current orientation. That means that we
always need to take into account where the turtle currently is and what
direction it is facing. Initially, when the window first opens, the turtle is
facing the top of the screen. From that orientation, to continue drawing a
square, we would need to turn to the right. To go from facing the top of the
screen to facing the right of the screen, we need to turn 90 degrees to the
right.

Translating this logic to a program would give us:

```rust
use turtle::Turtle;

fn main() {
    let mut turtle = Turtle::new();

    turtle.forward(200.0);
    turtle.right(90.0);
}
```

This gives us a turtle that has moved forward, and turned to the right:

![turtle facing right]({% link assets/images/guide/example-square/facing-right.png %}){: .figborder .figlimitsize}

With the triangle, we repeated the code to draw one side three times because
triangles have three sides. Let's try the same thing with the square. We'll
repeat the code to draw one side four times:

```rust
use turtle::Turtle;

fn main() {
    let mut turtle = Turtle::new();

    for _ in 0..4 {
        turtle.forward(200.0);
        turtle.right(90.0);
    }
}
```

And just like that, we've finished drawing the square!

![turtle square]({% link assets/images/guide/example-square/square.png %}){: .figborder .figlimitsize}

## Exercises

These exercises are designed to help you reinforce what you've learned
throughout this section. All exercises are completely **optional**. If you get
stuck on an exercise, it is totally okay to move on and come back to it later.

If you need help, see the [Getting Help] section of the guide.

- (*) squares of different sizes
- (*) unravel a single square
- (**) rotate the entire square image
- (**) draw 5 squares, rotating to the left by 360/5 = 72 degrees between each one
- (***) spiral of squares
  - Note that we used smaller squares (100) for this image
- (***) spiral of growing squares
  - Note that we used smaller squares (100) for this image
- (***) spiral of unravelling squares
  - Note that we used smaller squares (100) for this image
  - **Hint:** Use [`pen_up`] and [`pen_down`] to temporarily stop drawing while you
    move back to the center.
  - **Hint:** You can potentially avoid doing some complex math using the [`home`] method.
  - **Hint:** You can avoid using either of the previous hints by tracing backwards through
    the line you just drew. This will get you back to your starting position without any
    additional methods.

[Basic Movement]: {% link guide/basic-movement.md %}
[`forward`]: https://docs.rs/turtle/{{ site.data.lib.latest.version }}/turtle/struct.Turtle.html#method.forward
[`backward`]: https://docs.rs/turtle/{{ site.data.lib.latest.version }}/turtle/struct.Turtle.html#method.backward
[`right`]: https://docs.rs/turtle/{{ site.data.lib.latest.version }}/turtle/struct.Turtle.html#method.right
[`left`]: https://docs.rs/turtle/{{ site.data.lib.latest.version }}/turtle/struct.Turtle.html#method.left
[Getting Help]: {% link guide/getting-help.md %}
[`pen_up`]: https://docs.rs/turtle/{{ site.data.lib.latest.version }}/turtle/struct.Turtle.html#method.pen_up
[`pen_down`]: https://docs.rs/turtle/{{ site.data.lib.latest.version }}/turtle/struct.Turtle.html#method.pen_down
[`home`]: https://docs.rs/turtle/{{ site.data.lib.latest.version }}/turtle/struct.Turtle.html#method.home
