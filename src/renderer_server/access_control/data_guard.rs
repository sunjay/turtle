use std::sync::Arc;

use tokio::sync::{Mutex, MutexGuard};

use super::super::{
    app::{TurtleDrawings, TurtleId},
    state::DrawingState,
};

use super::SharedResources;

pub type TurtleDataGuard<'a> = MutexGuard<'a, TurtleDrawings>;

#[derive(Debug)]
pub struct TurtleData {
    pub(in super) id: TurtleId,
    pub(in super) turtle: Arc<Mutex<TurtleDrawings>>,
    pub(in super) resources: SharedResources,
}

impl TurtleData {
    pub async fn lock(&self) -> TurtleDataGuard<'_> {
        self.turtle.lock().await
    }
}

impl Drop for TurtleData {
    fn drop(&mut self) {
        self.resources.lock().turtle(self.id).free();
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
    pub(in super) resources: SharedResources,
}

impl DrawingData {
    pub async fn lock(&self) -> DrawingDataGuard<'_> {
        self.drawing.lock().await
    }
}

impl Drop for DrawingData {
    fn drop(&mut self) {
        self.resources.lock().drawing().free();
    }
}
