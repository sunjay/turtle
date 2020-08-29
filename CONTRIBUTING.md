# Contributing to turtle

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

This project and everyone participating in it is governed by the [Turtle Code of
Conduct][turtle-coc]. By participating, you are expected to uphold this code.
Please report unacceptable behavior to varma.sunjay+turtlecoc@gmail.com.

[turtle-coc]: https://github.com/sunjay/turtle/blob/master/CODE_OF_CONDUCT.md

## Getting Help
[getting-help]: #getting-help

**Having trouble with some part of the Rust language? Don't give up!**

The Rust community is full of kind and patient people who are ready to help you
with whatever you are dealing with as you learn Rust. Here are some ideas for
what you can do when you get stuck.

* Are you dealing with a specific error message?
  1. Try pasting key parts of the error message into your favorite search engine
    ([Google], [DuckDuckGo], etc.).
  2. Read the results and don't worry if you don't find what you need.
  3. **It is totally okay to ask for help if searching doesn't get you what you
     need or if you don't know what to search for!**
* Ask for help on a Q&A site like [Stack Overflow] or on the [Rust Users Forum].
  * You can use these websites to ask about using the turtle crate too. Don't
    be afraid to ask if you need help doing something with this crate or any
    other Rust crate!

**Is some part of the turtle crate not working as you expect?**

Create a new issue by following the instructions under [Bug Reports][bug-reports].
If you are not sure whether your problem qualifies for an issue, or if you're
having any trouble with any of this, come ask in the [Turtle Zulip] chat room.
We are happy to help you!

**Still stuck?**

If none of the above seems to apply, feel free to come ask in
the [Turtle Zulip] chat room or in the [Rust Users Forum]. Someone will help
you find what you need. :smile:

[Google]: http://google.com/
[DuckDuckGo]: https://duckduckgo.com/
[Stack Overflow]: https://stackoverflow.com/
[Rust Users Forum]: https://users.rust-lang.org/
[Turtle Zulip]: https://turtle.zulipchat.com

## Bug Reports
[bug-reports]: #bug-reports

> **New to GitHub?** You may want to read about [GitHub issues](https://help.github.com/articles/about-issues/).

While [bugs] are unfortunate, they're a reality in software. We can't fix what we
don't know about, so please report liberally. If you're not sure if something is
a bug or not, feel free to file a bug anyway.

**If you believe reporting your bug publicly represents a security risk to Rust
or turtle users, please follow the Rust team's [instructions for reporting
security vulnerabilities](https://www.rust-lang.org/security.html)**.

If you have the chance, before reporting a bug, please [search existing
issues][issues], as it's possible that someone else has already reported your
error. This doesn't always work, and sometimes it's hard to know what to search
for, so consider this extra credit. We won't mind if you accidentally file a
duplicate report.

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
version of the turtle crate you are using and which other crates may be running
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

[issues]: https://github.com/sunjay/turtle/issues
[bugs]: https://en.wikipedia.org/wiki/Software_bug

## Building, Testing & Generating Documentation
[building-testing-generating-documentation]: #building-testing--generating-documentation

The standard cargo commands can all be used with this project. Running tests
requires an extra flag which is documented below.

* To build the project, run `cargo build`
* To run an example, run `cargo run --example filename`
  * Example: run `cargo run --example rust` to run `examples/rust.rs`
  * This will automatically build the project for you
* To test the project, run `cargo test --features "test unstable"`
  * Without the `--features "test"` part, the tests would open several windows
    during testing and the tests will not end until those windows are closed
    (see `Cargo.toml` for more explanation)
  * Without the `--features "unstable"` part, the tests would not all compile
    because many tests depend on unstable features
  * We currently prevent you from running tests without the `test` feature
    enabled. Trying to do so should give you a helpful error message that tells
    you the right thing to do.
  * Animations are **disabled** during tests, so setting the speed will have no
    impact on anything. You should not write unit tests or doctests that depend
    on animations.
* To measure test code coverage, several steps:
  * Before anything, we use [Tarpaulin](https://github.com/xd009642/tarpaulin)
    in order to run the coverage. As of writing this, Tarpaulin only supports the
    **Linux x86_64** platform, so make sure to be using a compatible setup.
  * First, install Tarpaulin using `cargo install cargo-tarpaulin --version 0.13.3`.
    * We are currently using not the most recent version of Tarpaulin because
      one of our indirect dependencies itself uses a Tarpaulin configuration
      attribute refactored in `0.13.4`, which causes a compilation error. So
      until this indirect dependency gets to a newer version either by a direct
      dependency or us requiring it, this is how it will be.
    * This will very probably change in the future, in that case, it will be
      properly documented here and one will be able to omit the `--version 0.13.3`
      in order to download and install the latest version.
  * Then run `cargo +nightly tarpaulin --features "test unstable" --run-types Tests Doctests`
    * `--features "test unstable"` is required for the same reasons as the tests:
      the documentation tests use the `test` feature in order to get configured
      automatically and many tests require `unstable` features.
    * `+nightly` has to be given because Tarpaulin runs documentation tests only
      on the nightly channel. If you don't have it installed yet, please refer to
      the [rustup documentation](https://rust-lang.github.io/rustup/concepts/channels.html)
      in order to get it on your machine.

    * `--run-types Tests Doctests` is needed because Tarpaulin only measures
      coverage from the calls generated by the public tests by default, so we
      have to explicitely specify documentation tests as well: a lot of
      documentation tests have been implemented, so taking them into account
      drastically changes the overall score obtained.
  * Tarpaulin has a lot of other useful functionalities, so be sure to check
    them out by listing its available options using `cargo tarpaulin --help` and
    reading [their README](https://github.com/xd009642/tarpaulin#tarpaulin),
    especially for the `--out` option which lets you export results to many
    different formats.
  * When implementing new features or completing some for this project, try to
    also write as many tests as possible in order to keep your features in
    check and let everyone have a greater trust in them. The higher the score,
    usually the better!
* To generate the project's documentation, run `cargo doc`
  * To open the generated documentation, run `cargo doc --open`
  * When writing documentation, it is often easier to have the documentation
    automatically build itself in the background whenever you change a file.
    To set that up, install `cargo-watch` with `cargo install cargo-watch` and
    then run `cargo watch -x doc`
  * To only build the documentation for turtle and skip building all of its
    dependencies, run `cargo doc --no-deps` or `cargo watch -x 'doc --no-deps'`
  * When building documentation normally, the markers that list the features
    required for various parts of turtle are missing. To build the documentation
    with those markers, use this command:
    ```bash
    RUSTDOCFLAGS="--cfg docsrs" cargo +nightly doc --features "unstable" --open
    ```

## Adding Examples
[adding-examples]: #adding-examples

Adding an example is a great contribution for anyone of any skill level. Writing
examples can help you learn more about Rust and the turtle crate. Your
submissions will help show future learners creative and cool ways to use the
turtle crate. :smile:

See the [`examples/`](https://github.com/sunjay/turtle/tree/master/examples)
directory in this repository for the many examples that have already been
written.

There is an [example-idea](https://github.com/sunjay/turtle/labels/example-idea)
label for issues that contain great ideas for examples that you could create!

> **New to GitHub?** You may want to read about [GitHub Pull Requests](https://help.github.com/articles/about-pull-requests/).

To add an example, complete the following steps:

1. Create a [fork](https://help.github.com/articles/fork-a-repo/) of the
   [turtle repository](https://github.com/sunjay/turtle)
2. [Clone](https://help.github.com/articles/cloning-a-repository/) your fork of
   the repository
3. Add a file called `YOUREXAMPLE.rs` to the `examples/` directory in the cloned
   repository
   * Replace `YOUREXAMPLE` with the name you want your example to have (use
     underscores `_` instead of spaces)
   * For best results, use a short, descriptive name for your example
   * Good name: `snowman.rs`
   * Bad name: `example1.rs`
4. Edit `YOUREXAMPLE.rs` to contain the example code you want to contribute
   * See the other examples in the `examples/` directory to get an idea of what
     the file could look like
5. Run your example using `cargo run --example YOUREXAMPLE` (notice that there
   is no `.rs` this time)
6. Make any other changes and re-run the example as many times as you need to
   until you are satisfied
7. Create a commit with a brief message about your example
   ```bash
   git commit -a -m "This is my really cool example of a butterfly"
   ```
8. [Push the commit](https://guides.github.com/activities/forking/#making-changes)
   to your fork
7. [Create a pull request](https://help.github.com/articles/creating-a-pull-request/)
   and mention in your title and description that you're adding an example
   * Follow [this guide](https://guides.github.com/activities/forking/#making-a-pull-request)
     if you need help
   * (optional) Tell us a bit about your example and include screenshots or
     [gifs](https://github.com/phw/peek) of it if you can!

[**Check out this sample pull request of someone adding an example**](https://github.com/sunjay/turtle/pull/29)

**Some things to keep in mind about adding examples:**

* Try to look through the other examples in the `examples/` directory and make
  sure your example is different from everything else
* If your example is too similar, it may be better to modify the existing
  example to incorporate the improvements that you have made

## Fixing Bugs & Implementing Features
[fixing-bugs-implementing-features]: #fixing-bugs--implementing-features

Writing perfect software is pretty much impossible! We could use your help to
fix the problems that people find in turtle and to add new features that people
request!

> **New to GitHub?** You may want to read about [GitHub Pull Requests](https://help.github.com/articles/about-pull-requests/)
> and about [GitHub Issues](https://guides.github.com/features/issues/).

The turtle project uses the [Issue Tracker][issues] to keep track of the various
bugs that people find and features that people ask for.

### Issue Labels

We have a [good first issue](https://github.com/sunjay/turtle/labels/good%20first%20issue) label that
we use specifically for issues that we think first time contributors should be
able to complete. These issues should all have detailed instructions about what
exactly needs to be fixed and at least some hints about how you should go about
fixing the problem.

The [help wanted](https://github.com/sunjay/turtle/labels/help%20wanted) label
is used for any issues (including good first issues) that can be attempted by
anyone looking to contribute to turtle

Other labels help sort issues into various categories based on what areas of
turtle they are related to. For example, issues that are primarily about
contributing documentation are given the [docs](https://github.com/sunjay/turtle/labels/docs)
label whereas new features are given the [feature](https://github.com/sunjay/turtle/labels/feature)
label.

### Getting Help With Issues

**If you need more details about any issue at all, or if you get stuck trying to
fix an issue, please do not hesitate to leave a comment on the issue and ask
for help.**

We are happy to provide as much guidance as you need **regardless of your skill
level**. Please ask! Especially if an issue doesn't have enough instructions or
tips. We are ready to help you!

### Step-by-step

To start fixing a bug or adding a feature:

1. Find an issue in the [Issue Tracker][issues] that you want to fix, or
   create an issue (See [Bug Reports][bug-reports]) if one doesn't exist already
2. Leave a comment saying that you want to work on that issue so that no one
   else starts working on it while you're attempting it
3. Look at the description and any instructions that may have been posted
4. Ask for help as you need it (See [Getting Help][getting-help])
5. [Create a pull request](https://help.github.com/articles/creating-a-pull-request/)
   and mention in the body [which issue you are fixing](https://help.github.com/articles/closing-issues-using-keywords/)
   * Follow [this guide](https://guides.github.com/activities/forking/#making-a-pull-request)
     if you need help

[**Check out this sample pull request of someone fixing a bug**](https://github.com/sunjay/turtle/pull/40)

**Some things to keep in mind about pull requests:**

* Provide a description of what you changed and why you changed it (or how your
  changes fixed the issue you were working on)
* Be ready to accept feedback and make changes if necessary

### How we think about contributing

**This is important!** We want to work together to make this project as great as
possible! That means that when you submit some code, we'll look at it
**carefully** and give you detailed feedback about possible improvements.
Sometimes we'll have a lot to say, and other times we'll approve your changes
right away. Regardless, please remember that when we provide feedback about your
code, **it isn't a reflection of you or how skilled you are**. Even the most
advanced programmers solicit feedback reguarly and improve their work. The code
you're submitting is being **given to the community** and thus the community
will come in and take a look at it to make sure we're all doing the best work
possible.

See also: [Moving from Editorial to Engineering](https://truss.works/blog/2018/1/5/of-tracked-changes-and-diffs-moving-from-editorial-to-engineering)
