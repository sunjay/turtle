use std::cell::RefCell;
use std::path;

use crate::animation::{Animation, AnimationStatus, MoveAnimation, RotateAnimation};
use crate::query::{DrawingCommand, Query, Request, Response, StateUpdate};
use crate::radians::Radians;
use crate::renderer_process::RendererProcess;
use crate::state::{DrawingState, Path, TurtleState};
use crate::timer::Timer;
use crate::{start, Distance, Event, Point};

use DrawingCommand::*;

pub struct TurtleWindow {
    renderer: RefCell<RendererProcess>,
}

impl TurtleWindow {
    pub fn new() -> TurtleWindow {
        // This needs to be called as close to the start of the program as possible
        // Since Turtle::new() is called at the beginning of many turtle programs, we do so here
        // to make sure this call occurs when it should.
        start();

        Self {
            renderer: RefCell::new(RendererProcess::new()),
        }
    }

    pub fn fetch_turtle(&self) -> TurtleState {
        match self.renderer.borrow_mut().send_query(Query::Request(Request::TurtleState)) {
            Some(Response::TurtleState(state)) => state,
            _ => panic!("bug: the renderer process did not sent back TurtleState"),
        }
    }

    /// Fetch and update the turtle with no way of holding on to the reference or forgetting to
    /// update it after
    pub fn with_turtle_mut<F, T>(&mut self, update: F) -> T
    where
        F: FnOnce(&mut TurtleState) -> T,
    {
        let mut turtle = self.fetch_turtle();
        let result = update(&mut turtle);
        self.renderer
            .borrow_mut()
            .send_query(Query::Update(StateUpdate::TurtleState(turtle)));
        result
    }

    pub fn fetch_drawing(&self) -> DrawingState {
        match self.renderer.borrow_mut().send_query(Query::Request(Request::DrawingState)) {
            Some(Response::DrawingState(state)) => state,
            _ => panic!("bug: the renderer process did not sent back DrawingState"),
        }
    }

    /// Fetch and update the drawing with no way of holding on to the reference or forgetting to
    /// update it after
    pub fn with_drawing_mut<F, T>(&mut self, update: F) -> T
    where
        F: FnOnce(&mut DrawingState) -> T,
    {
        let mut drawing = self.fetch_drawing();
        let result = update(&mut drawing);
        self.renderer
            .borrow_mut()
            .send_query(Query::Update(StateUpdate::DrawingState(drawing)));
        result
    }

    fn set_temporary_path(&mut self, path: Option<Path>) {
        self.renderer
            .borrow_mut()
            .send_query(Query::Update(StateUpdate::TemporaryPath(path)));
    }

    /// See [`Drawing::poll_event()`](struct.Drawing.html#method.poll_event).
    pub fn poll_event(&mut self) -> Option<Event> {
        match self.renderer.borrow_mut().send_query(Query::Request(Request::Event)) {
            Some(Response::Event(event)) => event,
            _ => panic!("bug: the renderer process did not sent back an Event"),
        }
    }

    /// Begin filling the shape drawn by the turtle's movements.
    pub fn begin_fill(&mut self) {
        let fill_color = self.fetch_turtle().fill_color;
        self.send_drawing_command(BeginFill(fill_color));
    }

    /// Stop filling the current shape
    pub fn end_fill(&mut self) {
        self.send_drawing_command(EndFill);
    }

    /// Clear the turtle's drawings
    pub fn clear(&mut self) {
        self.send_drawing_command(Clear);
    }

    /// Save drawings as SVG
    pub fn save_svg<P: AsRef<path::Path>>(&self, path: P) {
        self.send_drawing_command(SaveSVG((*path.as_ref()).to_path_buf()));
    }

    /// Move the turtle to the given position without changing its heading.
    pub fn go_to(&mut self, end: Point) {
        let TurtleState {
            position: start,
            speed,
            pen,
            ..
        } = self.fetch_turtle();

        let distance = (end - start).len();
        if !distance.is_normal() {
            return;
        }
        let speed = speed.to_movement(); // px per second
                                         // We take the absolute value because the time is always positive, even if distance is negative
        let total_millis = (distance / speed * 1000.).abs();

        let animation = MoveAnimation {
            path: Path { start, end, pen },
            timer: Timer::start(),
            total_millis,
        };

        self.play_animation(animation);
    }

    /// Move the turtle forward by the given distance. To move backwards, use a negative distance.
    ///
    /// The turtle's motion will be animated based on the speed
    pub fn forward(&mut self, distance: Distance) {
        if !distance.is_normal() {
            return;
        }

        let TurtleState {
            position: start,
            speed,
            heading,
            pen,
            ..
        } = self.fetch_turtle();
        let movement = Point {
            x: distance * heading.cos(),
            y: distance * heading.sin(),
        };
        let end = start + movement;

        let speed = speed.to_movement(); // px per second
                                         // We take the absolute value because the time is always positive, even if distance is negative
        let total_millis = (distance / speed * 1000.).abs();

        let animation = MoveAnimation {
            path: Path { start, end, pen },
            timer: Timer::start(),
            total_millis,
        };

        self.play_animation(animation);
    }

    /// Rotate the turtle in place by the given angle in the given direction of rotation
    pub fn rotate(&mut self, angle: Radians, clockwise: bool) {
        if !angle.is_normal() {
            return;
        }

        let TurtleState { heading, speed, .. } = self.fetch_turtle();
        let speed = speed.to_rotation(); // radians per second
        let total_millis = angle / speed * 1000.;
        // We take the absolute value because the time is always positive, even if angle is negative
        let total_millis = total_millis.to_radians().abs();

        let animation = RotateAnimation {
            start: heading,
            delta_angle: angle,
            clockwise,
            timer: Timer::start(),
            total_millis,
        };

        self.play_animation(animation);
    }

    fn play_animation<A: Animation>(&mut self, animation: A) {
        loop {
            // We want to keep the lock for as little time as possible
            let status = self.with_turtle_mut(|turtle| animation.advance(turtle));
            match status {
                AnimationStatus::Running(path) => self.set_temporary_path(path),
                AnimationStatus::Complete(path) => {
                    if let Some(path) = path {
                        self.set_temporary_path(None);
                        self.send_drawing_command(StorePath(path));
                    }

                    break;
                }
            }
        }
    }

    fn send_drawing_command(&self, command: DrawingCommand) {
        self.renderer.borrow_mut().send_query(Query::Drawing(command));
    }
}
