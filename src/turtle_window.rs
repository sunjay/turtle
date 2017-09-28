use std::thread;
use std::process;
use std::sync::mpsc;

use piston_window::{
    PistonWindow,
    WindowSettings,
};

use renderer::{Renderer, Command, Response};

pub struct TurtleWindow {
    thread_handle: Option<thread::JoinHandle<()>>,
    transmitter: mpsc::Sender<Command>,
    receiver: mpsc::Receiver<Response>,
}

impl TurtleWindow {
    pub fn new() -> TurtleWindow {
        let (renderer_tx, renderer_rx) = mpsc::channel();
        let (main_tx, main_rx) = mpsc::channel();

        let handle = thread::spawn(move || {
            let mut window: PistonWindow = WindowSettings::new(
                "Turtle", [800, 600]
            ).exit_on_esc(true).build().unwrap();

            Renderer::new().run(&mut window, renderer_rx, main_tx);
        });

        Self {
            thread_handle: Some(handle),
            transmitter: renderer_tx,
            receiver: main_rx,
        }
    }

    pub fn apply(&self, command: Command) {
        let result = self.transmitter.send(command).map_err(|_| ());
        let result = result.and_then(|_| {
            // Wait for the drawing animation to complete
            self.receiver.recv().map_err(|_| ())
        });
        match result {
            Ok(_) => {},
            Err(_) => {
                // The connection has been closed so the window was closed
                // or an error occurred on that thread
                process::exit(0);
            },
        }
    }
}

impl Drop for TurtleWindow {
    fn drop(&mut self) {
        if let Some(handle) = self.thread_handle.take() {
            handle.join().unwrap();
        }
    }
}
