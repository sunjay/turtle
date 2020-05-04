use crate::{Point, Color};

use super::super::state::Pen;

/// A drawing primitive
#[derive(Debug, Clone)]
pub enum DrawPrim {
}

/// A handle to one of the items in a display list
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PrimHandle(usize);

/// A list of drawing primitives in the order that they are rendered
#[derive(Default, Debug)]
pub struct DisplayList {
    items: Vec<DrawPrim>,
}

impl DisplayList {
    pub fn push_line(&mut self, start: Point, end: Point, pen: &Pen) -> PrimHandle {
        //TODO: A `Point` is in logical coordiantes, whereas Vector2F
        todo!()
    }

    pub fn replace_line(&mut self, handle: PrimHandle, start: Point, end: Point, pen: &Pen) {
        todo!()
    }

    /// Creates a polygon with one point, and pushes it into the display list
    pub fn push_polygon_start(&mut self, start: Point, color: Color) -> PrimHandle {
        todo!()
    }

    /// Pushes a point into a polygon with the given handle
    ///
    /// Returns the index of that point in the polygon (guaranteed to be valid because points
    /// cannot be removed from polygons).
    pub fn polygon_push(&mut self, handle: PrimHandle, point: Point) -> usize {
        todo!()
    }

    /// Modifies a point in a polygon at the given index to be the given point
    ///
    /// Note that the provided index should only ever be one that was previously returned from
    /// the `polygon_push` method.
    pub fn polygon_update(&mut self, handle: PrimHandle, index: usize, point: Point) {
        todo!()
    }

    /// Sets the fill color of a polygon to the given color
    pub fn polygon_set_fill_color(&mut self, handle: PrimHandle, color: Color) {
        todo!()
    }

    /// Removes the given items from the display list
    pub fn remove<I: Iterator<Item=PrimHandle>>(&mut self, items: I) {
        todo!()
    }

    /// Removes all items from the display list
    pub fn clear(&mut self) {
        todo!()
    }
}
