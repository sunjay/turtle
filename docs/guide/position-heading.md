---
layout: docs
title: "Position and Heading"
docs_title: "Position and Heading"
permalink: /guide/position-heading/
---

In this section, we'll discuss some more advanced methods of controlling the
turtle. So far, we've been using commands like [`forward`] and [`right`] to
move the turtle relative to its **current** position on the screen. This works
well for a lot of drawings, but what if we want more direct control?

Before we get into the details of how we can move the turtle to specific
coordinates or rotate it to specific orientations, let's discuss the world
that the turtle resides in and how it is structured.

## The Turtle Canvas

As covered in [Overview], the turtle crate allows you to control a turtle
with a pen tied to its tail. You can think about the window that the turtle
resides in as the "canvas" that it's drawing on.

![initial turtle window]({% link assets/images/guide/overview/initial-turtle.png %}){: .figborder}

Initially, this canvas is 800 pixels by 600 pixels. A [pixel] is roughly the
size of 1 turtle step. That means that when you instruct the turtle to move
forward by 100 steps, you're moving the turtle forward roughly 100 pixels.

![turtle cartesian coordinates]({% link assets/images/guide/todo.png %}){: .figborder .figlimitsize}

The canvas uses a [Cartesian coordinate system][cart-coords]. The turtle starts
at the position (0,0) facing the positive y axis. When you move the turtle
forward 100 steps, its new position is (0,100).

## Setting Position and Heading

TODO

## Exercises

- (***) draw bezier curve (see examples/bezier.rs)

[`forward`]: https://docs.rs/turtle/{{ site.data.lib.latest.version }}/turtle/struct.Turtle.html#method.forward
[`right`]: https://docs.rs/turtle/{{ site.data.lib.latest.version }}/turtle/struct.Turtle.html#method.right
[Overview]: {% link guide/overview.md %}
[pixel]: https://en.wikipedia.org/wiki/Pixel
[cart-coords]: https://en.wikipedia.org/wiki/Cartesian_coordinate_system
