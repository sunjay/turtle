---
layout: docs
title: "Running Examples"
docs_title: "Running Examples"
permalink: /guide/running-examples/
---

Sometimes, it can be very helpful to see examples of full programs using the
turtle crate. While we cover several full examples in this guide, you can find
even more in the [`examples/`] folder of the [turtle GitHub repository][turtle-repo].

To run these examples, clone the repository using [git], and then use cargo to
launch the specific example you want to see. For instance, the following
commands run the `circle` example in `examples/circle.rs`:

```bash
git clone https://github.com/sunjay/turtle
cd turtle
git checkout v{{ site.data.lib.latest.version }}
cargo run --example circle
```

Note that when copying example files you still need to modify the `Cargo.toml`
file as specified in the [Quickstart] guide.

[turtle-repo]: https://github.com/sunjay/turtle
[`examples/`]: https://github.com/sunjay/turtle/tree/master/examples
[git]: https://git-scm.com/book
[Quickstart]: {% link guide/quickstart.md %}
