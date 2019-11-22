---
layout: docs
title: Turtle Documentation
nav_title: Guide
docs_title: Welcome!
permalink: /guide/
---

## Welcome to Turtle!

The turtle crate allows you to make animated drawings using the [Rust
programming language][rust].

![TODO: Interesting image that demos the crate (maybe a grid of images/gifs?)]

This crate is inspired by the [Logo educational programming language][logo-lang].
Many languages contain implementations of Logo's "turtle graphics". For example,
the [Python programming language][python] comes with a
[built-in turtle module][python-turtle]. This crate largely takes after the
Python implementation, but uses Rust conventions and best practices to provide
the best possible platform for learning Rust.

We work hard to make sure the crate is well-documented and full of examples. Our
goal is to make sure we provide an approachable, friendly, and fun way to create
drawings and learn the Rust language.

## Audience

The first several sections of this guide only require that you know the basics
of the Rust programming language. We'll use concepts like functions, variables,
loops, and if conditions, but not much more than that.

**If you're brand new to Rust, or if you run into a concept you don't understand,
we recommend that you read [The Rust Book][rust-book].**

You can create a great deal of drawings with turtle just knowing the basics of
Rust. We've intentionally designed it so that most of the complexity is hidden
away until you need it.

Reading the [online documentation][turtle-docs] for the types and methods we
provide may require a bit more knowledge, but you can mostly get by using what
you learn in this guide and guessing along the way. The helpful error messages
provided by the Rust compiler should also come in handy.

[rust]: https://www.rust-lang.org
[rust-book]: https://doc.rust-lang.org/book/
[python]: https://www.python.org
[python-turtle]: https://docs.python.org/3.3/library/turtle.html
[logo-lang]: https://en.wikipedia.org/wiki/Logo_(programming_language)
[turtle-docs]: https://docs.rs/turtle
