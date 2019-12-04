---
layout: docs
title: "Example: Polygons"
docs_title: "Example: Polygons"
permalink: /guide/example-polygons/
---

The goal of this lesson is to create a function that can draw any n-sided
polygon where n &ge; 3. We'll start with this program as the template for our
solution:

```rust
use turtle::Turtle;

fn main() {
    let mut turtle = Turtle::new();

    // The number of sides of the polygon to draw (>= 3)
    let sides = 3;
    polygon(&mut turtle, sides)
}

fn polygon(turtle: &mut Turtle, sides: usize) {
    assert!(sides >= 3, "The number of sides must be >= 3");

    //TODO
}
```

---

- Use this function to make other functions

```rust
fn triangle(turtle: &mut Turtle) {
    polygon(turtle, 3)
}

fn square(turtle: &mut Turtle) {
    polygon(turtle, 4)
}

fn pentagon(turtle: &mut Turtle) {
    polygon(turtle, 5)
}

fn hexagon(turtle: &mut Turtle) {
    polygon(turtle, 6)
}
```
