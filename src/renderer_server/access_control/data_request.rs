use std::{future::Future, sync::Arc, pin::Pin};

use futures_util::future;

use super::{DrawingData, TurtleData, Resources, PendingDataRequest, SharedResources};

use super::super::app::{App, TurtleId};

pub trait DataRequest {
    /// The type returned when the request is fulfilled
    type Output: Send;

    /// Attempt to reserve all the resources needed to fulfill this request
    ///
    /// `generate_data_request` should only be called if one of the required resources is not
    /// available. Every call to that function will increment the number of resources being waited
    /// for. If this method is called, the request will wait for all resources to become available
    /// before proceeding.
    fn poll_resources<F>(&self, resources: &mut Resources, generate_data_request: F)
        where F: FnMut() -> Arc<PendingDataRequest>;

    /// Assuming that the requested resources are now available, this method retrieves them
    //TODO: This can become an async trait method once those are supported by the compiler
    fn fetch_resources<'a>(&'a self, app: &'a App, resources: &'a SharedResources) -> Pin<Box<dyn Future<Output=Self::Output> + Send + 'a>>;
}

// This is used for the multiple turtles feature (for Turtle::clone())
impl<T: DataRequest, U: DataRequest> DataRequest for (T, U) {
    type Output = (<T as DataRequest>::Output, <U as DataRequest>::Output);

    fn poll_resources<F>(&self, resources: &mut Resources, mut generate_data_request: F)
        where F: FnMut() -> Arc<PendingDataRequest>
    {
        let (req1, req2) = self;
        req1.poll_resources(resources, &mut generate_data_request);
        req2.poll_resources(resources, &mut generate_data_request);
    }

    fn fetch_resources<'a>(&'a self, app: &'a App, resources: &'a SharedResources) -> Pin<Box<dyn Future<Output=Self::Output> + Send + 'a>> {
        let (req1, req2) = self;
        Box::pin(future::join(
            req1.fetch_resources(app, resources),
            req2.fetch_resources(app, resources),
        ))
    }
}

impl<T: DataRequest> DataRequest for Vec<T> {
    type Output = Vec<<T as DataRequest>::Output>;

    fn poll_resources<F>(&self, resources: &mut Resources, mut generate_data_request: F)
        where F: FnMut() -> Arc<PendingDataRequest>
    {
        for req in self {
            req.poll_resources(resources, &mut generate_data_request);
        }
    }

    fn fetch_resources<'a>(&'a self, app: &'a App, resources: &'a SharedResources) -> Pin<Box<dyn Future<Output=Self::Output> + Send + 'a>> {
        Box::pin(future::join_all(self.iter().map(|req| req.fetch_resources(app, resources))))
    }
}

impl DataRequest for TurtleId {
    type Output = TurtleData;

    fn poll_resources<F>(&self, resources: &mut Resources, mut generate_data_request: F)
        where F: FnMut() -> Arc<PendingDataRequest>
    {
        let turtle_res = resources.turtle(*self);
        if turtle_res.is_available() {
            turtle_res.reserve();
        } else {
            turtle_res.push_data_request(generate_data_request());
        }
    }

    fn fetch_resources<'a>(&'a self, app: &'a App, resources: &'a SharedResources) -> Pin<Box<dyn Future<Output=Self::Output> + Send + 'a>> {
        use futures_util::FutureExt;

        let id = *self;

        Box::pin(app.turtle(id).map(move |turtle| TurtleData {
            id,
            turtle,
            resources: resources.clone(),
        }))
    }
}

/// A placeholder struct that makes it possible to request the drawing
///
/// Makes it possible to request the drawing
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FetchDrawing;

impl DataRequest for FetchDrawing {
    type Output = DrawingData;

    fn poll_resources<F>(&self, resources: &mut Resources, mut generate_data_request: F)
        where F: FnMut() -> Arc<PendingDataRequest>
    {
        let drawing_res = resources.drawing();
        if drawing_res.is_available() {
            drawing_res.reserve();
        } else {
            drawing_res.push_data_request(generate_data_request());
        }
    }

    fn fetch_resources<'a>(&'a self, app: &'a App, resources: &'a SharedResources) -> Pin<Box<dyn Future<Output=Self::Output> + Send + 'a>> {
        Box::pin(future::ready(DrawingData {
            drawing: app.drawing().clone(),
            resources: resources.clone(),
        }))
    }
}
