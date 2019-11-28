---
layout: docs
title: Turtle Documentation
nav_title: Guide
docs_title: Welcome!
permalink: /guide/
---

The turtle crate allows you to make animated drawings using the [Rust
programming language][rust]. This guide will walk you through the different
features of the crate and teach you how to build your own drawings.

![TODO: Interesting image that demos the crate (maybe a grid of images/gifs?)]

Turtle was inspired by the [Logo educational programming language][logo-lang].
That language was originally designed as a teaching tool that could at one point
even be used to control a small turtle robot that drew pictures on physical
paper. Logo gave rise to the implementation of "turtle graphics" in many
different programming languages. For example, the [Python programming
language][python] comes with a [built-in turtle module][python-turtle]. This
crate largely takes after the Python implementation, but uses Rust conventions
and best practices to provide the best possible platform for learning and
teaching Rust.

We work hard to make sure the crate is well-documented and full of examples. Our
goal is to make sure we provide an approachable, friendly, and fun way to create
drawings and learn the Rust language.

## Audience

The Basics and Intermediate parts of this guide only require that you know the
basics of the Rust programming language. The main concepts we'll use are
functions/methods, variables, loops, and if conditions.

**If you're brand new to Rust, or if you run into a concept you don't understand,
we recommend that you read [The Rust Book][rust-book].**

You can create a great deal of drawings with turtle just knowing the basics of
Rust. We've intentionally designed it so that most of the complexity is hidden
away until you need it.

Reading the [online documentation][turtle-docs] for the types and methods we
provide may require a bit more knowledge, but you can mostly get by using what
you learn in this guide and guessing along the way. The helpful error messages
provided by the Rust compiler should also come in handy.

## Exercises

Most lessons of this guide will end with some exercises to help you further
reinforce what you learned during that lesson. All exercises are completely
**optional**. If you get stuck on an exercise, it is totally okay to move on and
come back to it later.

All exercises are rated by their difficulty. This is a subjective rating, so
some exercises may end up being easier or harder than they are rated.

* **`(*)` - knowledge check**<br />
  These exercises are usually just slight modifications of the programs seen in
  that lesson. They are designed to reinforce what you learned without trying
  to be very challenging.
* **`(**)` - moderate**<br />
  These exercises give you a little more of a challenge by having you write
  programs that test your understanding of the current lesson you're reading,
  and previous lessons. You'll get to apply what you've learned without
  necessarily stepping too far outside of what you've seen so far.
* **`(***)` - challenging**<br />
  These exercises are designed to allow you to explore the turtle crate. We want
  you to get some experience creating your own drawings using what you've
  learned and your own creativity. Completing an exercise with this rating may
  require looking things up in the turtle documentation. You may even need to
  think about what you have learned in a new way. In general, any exercise that
  takes a non-trivial amount of work on your part will get this rating.

If you need help, see the [Getting Help] section of the guide.

[rust]: https://www.rust-lang.org
[rust-book]: https://doc.rust-lang.org/book/
[python]: https://www.python.org
[python-turtle]: https://docs.python.org/3.3/library/turtle.html
[logo-lang]: https://en.wikipedia.org/wiki/Logo_(programming_language)
[turtle-docs]: https://docs.rs/turtle
[Getting Help]: {% link guide/getting-help.md %}
