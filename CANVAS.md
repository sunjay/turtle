Turtle for the web!

# Building

To build a wasm file, uncomment `crate-type = ["cdylib"]` in Cargo.toml and run:

```
find . -name '*.wasm' -delete && cargo +nightly build --no-default-features --features=canvas --target=wasm32-unknown-unknown --release
```

Cargo doesn't seem very smart about updating the `.wasm` file, so we always delete it.

Once the `.wasm` file is built, run a basic web server (like `python -m SimpleHTTPServer 9000`) and open [http://localhost:9000/canvas.html](http://localhost:9000/canvas.html).

# Architecture

On a desktop OS, Turtle uses a separate process to handle rendering and turtle state. This allows the turtle control logic to take over the main thread, which in turn allows a convenient programming model: the user provides `main()`, and as long as Turtle initialization is done right, everything proceeds fine. Rendering commands, etc, are done via stdin/stdout between the control process and render process.

On the web, we don't have the built-in start point of `main()`, so control flow will have to be different. Rendering can still be done with Piston's abstraction, but we need to render to a `<canvas>`, so we need a `Graphics` implementation that writes to an RGBA pixel buffer.

We also don't have proper threads. WebWorkers may end up being useful for handling keyboard events, but for the moment, we just take over the main thread for the host web page and do not handle input events.
