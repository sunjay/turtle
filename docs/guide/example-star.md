---
layout: docs
title: "Example: Star"
docs_title: "Example: Star"
permalink: /guide/example-star/
---

In this lesson, we'll be drawing the following image:

![turtle star]({% link assets/images/guide/example-star/star.png %}){: .figborder .figlimitsize}

This image resembles the hand drawn stars that many people learn to draw in
early childhood. Usually, the goal is to draw the star in a single attempt,
without lifting your pen or pencil off the page. A common way to do this is to
use the following procedure:

1. Draw a horizontal line left to right
2. Draw a line from the right side of that line to where the bottom
   left corner of the star should be
3. Draw a line from there to where the top of the star should be
4. Draw a line to where the bottom right corner of the star should be
5. Finish the star by connecting the bottom right corner to the left side of
   the star

This is better illustrated by the actual animation created by the final program
from this lesson:

![turtle star animation]({% link assets/images/guide/example-star/star.gif %}){: .figborder .figlimitsize}

Much like our other lessons, we'll start drawing this one line at a time. The
first line is a horizontal line, starting at the turtle's start position. We can
draw that by turning to the right and going forward:

```rust
use turtle::Turtle;

fn main() {
    let mut turtle = Turtle::new();

    turtle.right(90.0);
    turtle.forward(300.0);
}
```

This produces the following image:

![turtle horizontal line]({% link assets/images/guide/example-star/horizontal-line.png %}){: .figborder .figlimitsize}

After that, we need to turn the turtle to start drawing the next line. The
trickiest part of this is figuring out what angle to use for that turn. After
the turtle has drawn its first line, it finishes facing the right side of the
screen. If we can figure out the angle inside the right point of the triangle,
we should be able to turn enough to create that angle.



https://puzzling.stackexchange.com/questions/17681/five-angles-in-a-star

---

- Start by implementing each of the 5 steps
  - Explain how to compute angles
- Remove duplication using a loop
  - Key insight: the star is essentially the same partial triangle ("v" shape)
    repeated 5 times
- Concave to convex shapes
- Hand-drawn star

```rust
use turtle::Turtle;

fn main() {
    let mut turtle = Turtle::new();

    turtle.right(90.0);

    for _ in 0..5 {
        turtle.forward(300.0);
        turtle.right(180.0 - (180.0 / 5.0));
    }
}
```

## Exercises

- (*) Draw a 7-pointed star
  ![turtle 7-pointed star]({% link assets/images/guide/example-star/7-star.png %}){: .figborder .figlimitsize}
- (*) Draw a 9-pointed star
  ![turtle 9-pointed star]({% link assets/images/guide/example-star/9-star.png %}){: .figborder .figlimitsize}
- (*) Draw a star rotated 45 degrees counterclockwise around the center of the screen
- (**) Draw a star in the bottom left corner of the window
- (**) Draw a star that is (approximately) centered in the window
- (**) Repeat the star 5 times, rotating each time to produce a bunch of stars rotated around the center of the screen
- (***) Draw a circle of stars around an actual circle where each star is equidistant along the star (hint: make a `star` function)
- (***) Extend your program from Exercise ## to draw any n-pointed star (where n is odd and n >= 5)
- (***) Write a program that draws a 5 pointed star without any lines in the middle (examples/empty_star.rs)
  - Bonus: Set the line color of your star and fill it to make a golden star
- `(***)` Write a program to draw an n-pointed star (where n >= 4 and n can be **even**)
  - Hint: Try drawing some stars on a piece of paper where n is even. You won't be able to do it without lifting your pen.
    - Hint: [`pen_up`] and [`pen_down`]
  - Hint: Divide your program into two cases: one for when n is even and one for when n is odd
  - You may want to come back to this exercise after completing more of this guide.
