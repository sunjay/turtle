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

This is better illustrated if you can see the actual animation that takes place
if you run the final program from this lesson:

![turtle star animation]({% link assets/images/guide/example-star/star.gif %}){: .figborder .figlimitsize}

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
