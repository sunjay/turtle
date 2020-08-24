use std::sync::Arc;

use tokio::sync::{MutexGuard, Mutex, mpsc};

use super::super::{
    app::{TurtleDrawings, TurtleId},
    state::DrawingState,
};

use super::Resources;

pub type TurtleDataGuard<'a> = MutexGuard<'a, TurtleDrawings>;

#[derive(Debug)]
pub struct TurtleData {
    pub(in super) id: TurtleId,
    pub(in super) turtle: Arc<Mutex<TurtleDrawings>>,
    pub(in super) resource_manager: ResourceManager,
}

impl TurtleData {
    pub async fn lock(&self) -> TurtleDataGuard<'_> {
        self.turtle.lock().await
    }
}

impl Drop for TurtleData {
    fn drop(&mut self) {
        self.resource_manager.free_turtle(self.id);
    }
}

pub async fn lock_turtles(turtles: &[TurtleData]) -> Vec<TurtleDataGuard<'_>> {
    //TODO: Replace this with a `tokio` equivalent when tokio-rs/tokio#2478 is resolved:
    //  https://github.com/tokio-rs/tokio/issues/2478
    use futures_util::future::join_all;

    join_all(turtles.iter().map(|turtle| turtle.lock())).await
}

pub type DrawingDataGuard<'a> = MutexGuard<'a, DrawingState>;

#[derive(Debug)]
pub struct DrawingData {
    pub(in super) drawing: Arc<Mutex<DrawingState>>,
    pub(in super) resource_manager: ResourceManager,
}

impl DrawingData {
    pub async fn lock(&self) -> DrawingDataGuard<'_> {
        self.drawing.lock().await
    }
}

impl Drop for DrawingData {
    fn drop(&mut self) {
        self.resource_manager.free_drawing();
    }
}

#[derive(Debug, Clone)]
enum FreeResource {
    /// Report that the drawing is now free for use
    Drawing,

    /// Report that the given turtle is now free for use
    Turtle(TurtleId),
}

/// An asynchronous task that manages freeing resources once they are no longer being used
///
/// This exists because you can't run asynchronous code in Drop, but you can send a message over a
/// channel.
#[derive(Debug, Clone)]
pub struct ResourceManager {
    sender: mpsc::UnboundedSender<FreeResource>,
}

impl ResourceManager {
    pub fn new(res: Arc<Mutex<Resources>>) -> Self {
        let (sender, mut receiver) = mpsc::unbounded_channel();

        tokio::spawn(async move {
            let handle_free = |res: &mut Resources, free_res| {
                match free_res {
                    FreeResource::Drawing => res.drawing().free(),

                    FreeResource::Turtle(id) => {
                        res.turtle(id).free();
                    },
                }
            };

            // The thread will exit once no more messages will ever be received
            while let Some(free_res) = receiver.recv().await {
                let mut res = res.lock().await;
                handle_free(&mut res, free_res);

                // Handle as many other messages as we can while we're still holding on to the lock
                while let Ok(free_res) = receiver.try_recv() {
                    handle_free(&mut res, free_res);
                }
            }
        });

        Self {sender}
    }

    pub fn free_drawing(&self) {
        self.sender.send(FreeResource::Drawing)
            .expect("bug: resource manager task should outlive notifiers");
    }

    pub fn free_turtle(&self, id: TurtleId) {
        self.sender.send(FreeResource::Turtle(id))
            .expect("bug: resource manager task should outlive notifiers");
    }
}
