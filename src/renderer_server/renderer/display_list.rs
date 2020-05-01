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
