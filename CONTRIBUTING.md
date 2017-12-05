# Contributing to Turtle

:+1::tada: First off, thanks for taking the time to contribute! :tada::+1:

Anyone of any skill level or background is welcome to contribute to this crate!
We will help you out as much as we can by providing instructions and pointing
you in the direction of useful resources as you need them.

Please be kind and courteous to us as we work together to make this crate as
great as possible!

## Table of Contents
[table-of-contents]: #table-of-contents

1. [Code of Conduct][code-of-conduct]
2. [Getting Help][getting-help]
3. [Bug Reports][bug-reports]
4. [Building, Testing & Generating Documentation][building-testing-generating-documentation]
5. [Adding Examples][adding-examples] - :tada::tada: **Great for first time contributors!** :tada::tada:
6. [Fixing Bugs & Implementing Features][fixing-bugs-implementing-features]

## Code of Conduct
[code-of-conduct]: #code-of-conduct

**TODO:** We will have one very soon! If you know about adding a code of conduct
to a project, please reach out!

## Getting Help
[getting-help]: #getting-help

> **Note:** Please don't file an issue to ask a question. You'll get faster
> results by using the resources below.

The following are some resources you can use to find help when you run into a
problem. The links are listed in the order you should try each one, but feel
free to come to the [Turtle Gitter] anytime if you are lost.

* **Help with the Rust Language** - [Google], [Stack Overflow], [Rust Users Forum], [Turtle Gitter]
* **Help with Turtle** - [Stack Overflow], [Turtle Gitter], [Google], [Rust Users Forum]
* **Found a bug?** - [Open an issue][issues] (feel free to ask about your bug in the [Turtle Gitter] if you are not sure)

[Google]: http://google.com/
[Stack Overflow]: https://stackoverflow.com/
[Rust Users Forum]: https://users.rust-lang.org/
[Turtle Gitter]: https://gitter.im/rust-turtle/discuss
[issues]: https://github.com/sunjay/turtle/issues

## Bug Reports
[bug-reports]: #bug-reports

> **New to GitHub?** You may want to read about [GitHub issues](https://help.github.com/articles/about-issues/).

While bugs are unfortunate, they're a reality in software. We can't fix what we
don't know about, so please report liberally. If you're not sure if something is
a bug or not, feel free to file a bug anyway.

**If you believe reporting your bug publicly represents a security risk to Rust
or Turtle users, please follow the Rust team's [instructions for reporting
security vulnerabilities](https://www.rust-lang.org/security.html)**.

If you have the chance, before reporting a bug, please [search existing
issues](https://github.com/sunjay/turtle/search?q=&type=Issues), as it's
possible that someone else has already reported your error. This doesn't always
work, and sometimes it's hard to know what to search for, so consider this extra
credit. We won't mind if you accidentally file a duplicate report.

Opening an issue is as easy as following
[this link](https://github.com/sunjay/turtle/issues/new) and filling out the
fields. Here's a template that you can use to file a bug, though it's not
necessary to use it exactly:

    <short summary of the bug>

    I tried this code:

    <code sample that causes the bug>

    I expected to see this happen: <explanation>

    Instead, this happened: <explanation>

    ## Meta

    Run `rustc --version --verbose` and paste the output:
    ```
    rustc version here...
    ```

    <details><summary>Cargo.lock</summary>
    Paste the contents of your Cargo.lock file here
    </details>

    #### Backtrace

All three components are important: what you did, what you expected, what
happened instead. Please include the output of `rustc --version --verbose`,
which includes important information about what platform you're on, what version
of Rust you're using, etc. Providing your `Cargo.lock` file will tell us which
version of the Turtle crate you are using and which other crates may be running
as well.

Sometimes, a backtrace is helpful, and so including that is nice. To get a
backtrace, set the `RUST_BACKTRACE` environment variable to a value other than
`0`. The easiest way to do this is to invoke `rustc` like this:

```bash
$ RUST_BACKTRACE=1 cargo run ...
```

On Windows, you will need to run the `set` command before running `cargo run`.

```cmd
set RUST_BACKTRACE=1
cargo run ...
```

## Building, Testing & Generating Documentation
[building-testing-generating-documentation]: #building-testing--generating-documentation

The standard cargo commands can all be used with this project. Running tests
requires an extra flag which is documented below.

* To build the project, run `cargo build`
* To run an example, run `cargo run --example filename`
  * Example: run `cargo run --example rust` to run `examples/rust.rs`
  * This will automatically build the project for you
* To test the project, run `cargo test --features test`
  * If you do not include the `--features test` part, the tests will open
    several windows during testing and the tests will not end until those
    windows are closed (see `Cargo.toml` for more explanation)
* To generate the project's documentation, run `cargo doc`
  * When writing documentation, it is often easier to have the documentation
    automatically build itself in the background whenever you change a file.
    To set that up, install `cargo-watch` with `cargo install cargo-watch` and
    then run `cargo watch -x doc`

## Adding Examples
[adding-examples]: #adding-examples

> **New to GitHub?** You may want to read about [GitHub Pull Requests](https://help.github.com/articles/about-pull-requests/).

[Example Pull Request](https://github.com/sunjay/turtle/pull/29)

Adding an example is a great contribution for anyone of any skill level.
Writing examples can help you learn more about Rust and the Turtle crate. Your
submissions can help future learners understand a concept.

See the [`examples/`](https://github.com/sunjay/turtle/tree/master/examples)
directory in this repository for the many examples that have already been
written.

TODO: More documentation to come! (maybe written by you?)
* TODO: Instructions about running examples
* TODO: Improve the structure of this section
* TODO: Details about where to add files, naming, adding to examples/README.md, etc.

**Some things to keep in mind about adding examples:**

* Try to look through the other examples in the `examples/` directory and make
  sure your example is different from everything else
* If your example is too similar, it may be better to modify the existing
  example to incorporate the improvements that you have made

**Some things to keep in mind about pull requests:**

* Reference the issue you are trying to address by its number or URL
  (optional, only necessary if an issue exists for what you are submitting)
* Provide a description of what you changed and why you changed it (or why your
  changes fixed the issue you were working on)
* Be ready to accept feedback and make changes if necessary
  * We want to work together to make this project as great as possible! That
    means that when you submit some code, we'll look at it carefully and give
    you feedback about possible improvements. Sometimes we'll have a lot to say,
    and other times we'll approve your changes right away. Either way, please
    remember that when we provide feedback about your code, it isn't a reflection
    of you. Even the most advanced programmers solicit feedback reguarly and
    improve their work. The code you're submitting is being given to the
    community and thus the community will come in and take a look at it.

## Fixing Bugs & Implementing Features
[fixing-bugs-implementing-features]: #fixing-bugs--implementing-features

> **New to GitHub?** You may want to read about [GitHub Pull Requests](https://help.github.com/articles/about-pull-requests/).

[Example Pull Request (PR)](https://github.com/sunjay/turtle/pull/29)

See the notes above about things to keep in mind about pull requests.

Feel free to ask in the issue you are trying to fix if you would like to learn
how to fix something or if you need any help.

TODO: More documentation to come! (maybe written by you?)
* TODO: Instructions about submitting a PR
* TODO: Improvements to this section and how it is structured
* TODO: Details
