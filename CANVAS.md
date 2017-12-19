Turtle for the web!

# Building

To build wasm files for all examples, run:

```
find . -name '*.wasm' -delete && cargo +nightly build \
    --no-default-features --features=canvas \
    --target=wasm32-unknown-unknown --release \
    --examples

```

Cargo doesn't seem very smart about updating the `.wasm` file, so we always delete it.

Once the `.wasm` file is built, run a basic web server (like `python -m SimpleHTTPServer 9000`) and open [http://localhost:9000/canvas.html](http://localhost:9000/canvas.html).

# Architecture

On a desktop OS, Turtle uses a separate process to handle rendering and turtle state. This allows the turtle control logic to take over the main thread, which in turn allows a convenient programming model: the user provides `main()`, and as long as Turtle initialization is done right, everything proceeds fine. Rendering commands, etc, are done via stdin/stdout between the control process and render process.

On the web, we don't have the built-in start point of `main()`, so control flow will have to be different. Rendering can still be done with Piston's abstraction, but we need to render to a `<canvas>`, so we need a `Graphics` implementation that writes to an RGBA pixel buffer.

The Rust logic runs in a Worker and periodically sends copies of the pixel buffer to the main thread to be displayed, which is inefficient but does work. However, there isn't a synchronous way to communicate between the Worker and the main thread, so even if we buffered up input events in the main thread, there wouldn't be a way to get them to the Rust logic since that logic never yields control of the worker thread. There are asynchronous ways to hand off data, but that would require the Rust logic halting and resuming later, which the current structure does not allow.
