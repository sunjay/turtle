use std::process;
use std::sync::mpsc;

use app::TurtleApp;
use renderer::Renderer;

pub fn run() {
    let app = TurtleApp::new();
    let (_drawing_tx, drawing_rx) = mpsc::channel();
    let (events_tx, _events_rx) = mpsc::channel();

    let read_only = app.read_only();
    Renderer::new().run(drawing_rx, events_tx, read_only);

    process::exit(0);
}
