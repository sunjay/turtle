use super::app::App;

/// Ensures that request handling is sequentially consistent
///
/// Requests come in to the server in a certain order. Each request has a certain set of
/// "dependencies". These dependencies are the parts of the state that the request wants to
/// modify. We want to avoid partially updating state or ending up in an inconsistent state, so it
/// is important that each request waits until **all** of the data it needs is available. Another
/// part of this is that requests that do not dependent on any of the same data should be able to
/// run concurrently. For example, if two turtles are drawing separate lines, they should be able
/// to draw those lines at the same time.
///
/// More precisely:
/// * This type enforces the property that requests are executed in the order in which they arrive.
/// * Requests that arrive later are only allowed to execute before prior requests if they do not
///   share any data dependencies with the prior requests.
///
/// Example: Suppose there are N = 4 turtles and you have the following requests:
/// - Request R1 depends on turtles: 1, 2, 3
/// - Request R2 depends on turtles: 4
/// - Request R3 depends on turtles: 3, 4
/// - Request R4 depends on turtles: 1, 2, 3, 4
/// - Request R5 depends on turtles: 1
///
/// Expected behaviour:
/// 1. R1 and R2 execute concurrently, no shared dependencies
/// 2. R3 waits on both R1 and R2
/// 3. R4 waits on R3 (and implicitly R1 and R2)
/// 4. R5 waits on R4
///
/// Conceptually, you can imagine that there is a queue for each turtle's data. The requests can
/// be sorted into those queues like so:
///
/// 1: R1, R4, R5
/// 2: R1, R4
/// 3: R1, R3, R4
/// 4: R2, R3, R4
///
/// The key here is that each command is listed in the order that it was in the original queue. A
/// command cannot execute until the command before it is done. A command can't be done until it's
/// at the front of all the queues it is in.
///
/// Note that commands can take a non-instant amount of time to execute. (That is, a command can
/// `await` during its execution.) That means that any locks need to be held across await points so
/// that a command completely finishes executing before the next command is notified that the lock
/// is available.
#[derive(Debug)]
pub struct RequestSequencer<'a> {
    app: &'a App,
}

impl<'a> RequestSequencer<'a> {
    pub fn new(app: &'a App) -> Self {
        Self {
            app,
        }
    }

    /// Requests the opportunity to potentially read or modify all turtles
    ///
    /// This request is guaranteed to be fulfilled in FIFO order as soon as all of the turtles are
    /// available.
    pub async fn request_all_turtles(&self) -> Vec<Arc<Mutex<TurtleDrawings>>> {
        todo!()
    }

    /// Requests the opportunity to potentially read or modify all turtles
    pub async fn request_all_turtles(&self) -> Vec<Arc<Mutex<TurtleDrawings>>> {
        todo!()
    }
}
