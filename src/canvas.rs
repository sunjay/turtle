use std::thread;
use std::sync::mpsc;

use app::TurtleApp;
use renderer::Renderer;
use server;

pub fn run() {
    let app = TurtleApp::new();
    let read_only = app.read_only();
    let (query_tx, query_rx) = mpsc::channel();

    thread::spawn(move || {
        server::run(app, query_tx);
    });

    // Renderer MUST run on the main thread or else it will panic on MacOS
    Renderer::new().run(query_rx, read_only);
}
