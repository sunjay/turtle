use std::collections::BTreeMap;

use crate::{Point, Color};

use super::super::state::Pen;

#[derive(Debug, Clone)]
pub struct Line {
    /// The point where the line will begin when drawn
    pub start: Point,
    /// The point where the line will end when drawn
    pub end: Point,

    /// The thickness of the line in (logical) pixels
    pub thickness: f64,
    /// The stroke color of the line
    pub color: Color,
}

#[derive(Debug, Clone)]
pub struct Polygon {
    /// The points on the polygon, guaranteed to be non-empty
    ///
    /// A 1-point or 2-point polygon is trivially degenerate, so it is not drawn.
    pub points: Vec<Point>,

    /// The fill color of the polygon
    pub fill_color: Color,
}

/// A drawing primitive
#[derive(Debug, Clone)]
pub enum DrawPrim {
    Line(Line),
    Polygon(Polygon),
}

impl DrawPrim {
    pub fn as_line_mut(&mut self) -> Option<&mut Line> {
        use DrawPrim::*;
        match self {
            Line(line) => Some(line),
            _ => None,
        }
    }

    pub fn as_polygon_mut(&mut self) -> Option<&mut Polygon> {
        use DrawPrim::*;
        match self {
            Polygon(polygon) => Some(polygon),
            _ => None,
        }
    }
}

/// A unique handle to one of the items in a display list
///
/// Handles are guaranteed to be unique, even across removals from the display list.
#[derive(Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub struct PrimHandle(usize);

/// A list of drawing primitives in the order that they are rendered
#[derive(Default, Debug)]
pub struct DisplayList {
    /// Using a b-tree because it provides sorted access/iteration *and* removal of individual
    /// elements. The ID within `PrimHandle` is always monotonic, so this should provide the
    /// correct draw order.
    items: BTreeMap<PrimHandle, DrawPrim>,
    /// The next ID inside `PrimHandle`, must be monotonic (even across removals/deletions)
    next_id: usize,
}

impl DisplayList {
    /// Pushes a new line into the display list
    ///
    /// If a new line would not need to be drawn based on the pen configuration, `None` is
    /// returned. Otherwise, a handle to the line that will be drawn is returned.
    pub fn push_line(&mut self, start: Point, end: Point, pen: &Pen) -> Option<PrimHandle> {
        let &Pen {is_enabled, thickness, color} = pen;

        // Do not draw lines for which the pen is disabled
        if !is_enabled {
            return None;
        }

        let handle = self.insert(DrawPrim::Line(Line {start, end, thickness, color}));
        Some(handle)
    }

    /// Updates the `end` point of a line
    ///
    /// Panics if the given handle does not refer to a line primitive.
    pub fn line_update_end(&mut self, handle: PrimHandle, end: Point) {
        let prim = self.items.get_mut(&handle).expect("bug: invalid handle");
        let line = prim.as_line_mut()
            .expect("bug: attempt to update the end of a draw primitive that was not a line");
        line.end = end;
    }

    /// Creates a polygon with one point, and pushes it into the display list
    pub fn push_polygon_start(&mut self, start: Point, fill_color: Color) -> PrimHandle {
        self.insert(DrawPrim::Polygon(Polygon {points: vec![start], fill_color}))
    }

    /// Pushes a point into a polygon with the given handle
    ///
    /// Returns the index of that point in the polygon (guaranteed to be valid because points
    /// cannot be removed from polygons).
    ///
    /// Panics if the given handle does not refer to a polygon primitive.
    pub fn polygon_push(&mut self, handle: PrimHandle, point: Point) -> usize {
        let prim = self.items.get_mut(&handle).expect("bug: invalid handle");
        let polygon = prim.as_polygon_mut()
            .expect("bug: attempt to push into a draw primitive that was not a polygon");

        let index = polygon.points.len();
        polygon.points.push(point);
        index
    }

    /// Modifies a point in a polygon at the given index to be the given point
    ///
    /// Note that the provided index should only ever be one that was previously returned from
    /// the `polygon_push` method.
    ///
    /// Panics if the given handle does not refer to a polygon primitive or if the given index is
    /// out of bounds.
    pub fn polygon_update(&mut self, handle: PrimHandle, index: usize, point: Point) {
        let prim = self.items.get_mut(&handle).expect("bug: invalid handle");
        let polygon = prim.as_polygon_mut()
            .expect("bug: attempt to update a point in a draw primitive that was not a polygon");

        // This will panic if the index is out of bounds
        polygon.points[index] = point;
    }

    /// Sets the fill color of a polygon to the given color
    ///
    /// Panics if the given handle does not refer to a polygon primitive.
    pub fn polygon_set_fill_color(&mut self, handle: PrimHandle, fill_color: Color) {
        let prim = self.items.get_mut(&handle).expect("bug: invalid handle");
        let polygon = prim.as_polygon_mut()
            .expect("bug: attempt to set the fill color of a draw primitive that was not a polygon");
        polygon.fill_color = fill_color;
    }

    /// Removes the given items from the display list
    pub fn remove<I: Iterator<Item=PrimHandle>>(&mut self, items: I) {
        for handle in items {
            self.items.remove(&handle);
        }
    }

    /// Removes all items from the display list
    ///
    /// This invalidates all handles that have been returned so far. Continuing to use them will
    /// cause a panic. New handles created after this will still be unique.
    pub fn clear(&mut self) {
        self.items.clear();
    }

    /// Iterates over the items in the display list in the order in which they should be rendered
    pub fn iter(&self) -> impl Iterator<Item=&DrawPrim> {
        self.items.values()
    }

    /// Inserts a new primitive into the display list, returning its handle
    fn insert(&mut self, prim: DrawPrim) -> PrimHandle {
        let handle = PrimHandle(self.next_id);
        self.next_id += 1;
        assert!(self.items.insert(handle, prim).is_none(), "bug: handles should be unique");
        handle
    }
}
