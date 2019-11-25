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

* **Exercise 1: `(*)`** Change the side length of the square to 100 instead of 200.
* **Exercise 2: `(*)`** Create an "unravelled" square by changing the angle that
  you rotate by in each loop iteration. The image your program generates may
  resemble the following:
  ![turtle unravelled square]({% link assets/images/guide/example-square/exercise-unravelled-square.png %}){: .figborder .figlimitsize}
* **Exercise 3: `(**)`** Starting at a side length of 100, draw 10 squares of
  increasing side length on top of each other. Your drawing may end up
  resembling the following image:
  ![turtle overlapping squares]({% link assets/images/guide/example-square/exercise-overlapping-squares.png %}){: .figborder .figlimitsize}
* **Exercise 4: `(**)`** Starting at the program that draws a single square,
  rotate the entire square by 30 degrees to the left or to the right. If you
  choose to rotate the square to the left, your drawing may resemble the following:
  ![turtle rotated square]({% link assets/images/guide/example-square/exercise-rotate-square.png %}){: .figborder .figlimitsize}
  **Hint:** If you use the techniques from [Basic Movement], you can do this by adding
  only a single line of code to the program above.
* **Exercise 5: `(**)`** Draw 5 squares, rotating to the left between each one
  to create the spiral pattern shown below:
  ![turtle spiral of 5 squares]({% link assets/images/guide/example-square/exercise-spiral-squares-5.png %}){: .figborder .figlimitsize}
  **Hint:** To rotate around an entire circle using 5 objects, you need to
  turn by 360/5 = 72 degrees each time.
* **Exercise 6: `(**)`** Draw a spiral of squares that resembles the image below.
  ![turtle spiral of squares]({% link assets/images/guide/example-square/exercise-spiral-squares.png %}){: .figborder .figlimitsize}
  **Tip:** You may want to use a smaller side length of 100 if you want to match
  this image exactly.
* **Exercise 7: `(***)`** Draw a spiral of squares where the side length increases
  for every square. The image you generate may resemble the following:
  ![turtle spiral of growing squares]({% link assets/images/guide/example-square/exercise-growing-spiral-squares.png %}){: .figborder .figlimitsize}
  **Tip:** You may want to use a smaller initial side length of 100 if you want
  to match this image exactly.
* **Exercise 8: `(***)`** Draw a spiral of squares where each square unravels
  more than the last. The image you produce may resemble the following:
  ![turtle spiral of unravelling squares]({% link assets/images/guide/example-square/exercise-unravelling-spiral-squares.png %}){: .figborder .figlimitsize}
  **Tip:** You may want to use a smaller side length of 100 if you want to match
  this image exactly.<br>
  **Hint:** You can get back to where the turtle started drawing the unravelled
  square by moving backward along the square you just drew.<br>
  **Bonus:** For an extra challenge, look up the [`pen_up`], [`pen_down`], and
  [`home`] methods in the documentation. You can use these to immediately return
  back to the center without drawing a line.

[Basic Movement]: {% link guide/basic-movement.md %}
[`forward`]: https://docs.rs/turtle/{{ site.data.lib.latest.version }}/turtle/struct.Turtle.html#method.forward
[`backward`]: https://docs.rs/turtle/{{ site.data.lib.latest.version }}/turtle/struct.Turtle.html#method.backward
[`right`]: https://docs.rs/turtle/{{ site.data.lib.latest.version }}/turtle/struct.Turtle.html#method.right
[`left`]: https://docs.rs/turtle/{{ site.data.lib.latest.version }}/turtle/struct.Turtle.html#method.left
[Getting Help]: {% link guide/getting-help.md %}
[`pen_up`]: https://docs.rs/turtle/{{ site.data.lib.latest.version }}/turtle/struct.Turtle.html#method.pen_up
[`pen_down`]: https://docs.rs/turtle/{{ site.data.lib.latest.version }}/turtle/struct.Turtle.html#method.pen_down
[`home`]: https://docs.rs/turtle/{{ site.data.lib.latest.version }}/turtle/struct.Turtle.html#method.home
