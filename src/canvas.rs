use std::thread;
use std::sync::mpsc;

use app::TurtleApp;
use renderer::Renderer;
use server;

pub fn run() {
    let app = TurtleApp::new();
    let read_only = app.read_only();
    let (drawing_tx, drawing_rx) = mpsc::channel();

    thread::spawn(move || {
        server::run(app, drawing_tx);
    });

    // Renderer MUST run on the main thread or else it will panic on MacOS
    Renderer::new().run(drawing_rx, read_only);
}
