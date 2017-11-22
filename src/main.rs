#[macro_use]
extern crate serde_derive;

extern crate serde;
extern crate serde_json;

extern crate piston_window;
extern crate interpolation;
extern crate rand;

mod app;
mod color;
mod event;
mod extensions;
mod query;
mod radians;
mod renderer;
mod speed;
mod state;
mod types;

use std::sync::mpsc;

use app::TurtleApp;
use renderer::Renderer;

fn main() {
    let app = TurtleApp::new();
    let (_drawing_tx, drawing_rx) = mpsc::channel();
    let (events_tx, _events_rx) = mpsc::channel();

    let read_only = app.read_only();
    Renderer::new().run(drawing_rx, events_tx, read_only);
}
