---
layout: docs
title: "Basic Movement"
docs_title: "Basic Movement"
permalink: /guide/basic-movement/
---

## This page is under construction!

Please check back later! :)

- Already introduced forward and right
- Introduce other basic movement commands
- Movement is always relative to the **current** position and orientation of the turtle
- Example of drawing a triangle, then drawing 3 triangles using a nested loop

```rust
use turtle::Turtle;

fn main() {
    let mut turtle = Turtle::new();

    turtle.left(30.0);

    for _ in 0..3 {
        for _ in 0..3 {
            turtle.forward(100.0);
            turtle.right(120.0);
        }

        turtle.left(120.0);
    }
}
```
