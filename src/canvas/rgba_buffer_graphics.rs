use std::{slice, fmt};
use std::cmp::Ordering;

use graphics::{self, types};
use ::runtime::Runtime;

// TODO textures
pub struct RgbaTexture {}

impl graphics::ImageSize for RgbaTexture {
    fn get_size(&self) -> (u32, u32) {
        (0, 0)
    }
}

/// Graphics implementation that draws into memory for use by an `ImageData` backing a `<canvas>`.
pub struct RgbaBufferGraphics {
    width: usize,
    height: usize,
    // TODO find a more appropriate way to handle ownership of the buffer
    buffer: *mut u8,
}

impl RgbaBufferGraphics {
    pub fn new(width: usize, height: usize, buffer: *mut u8) -> RgbaBufferGraphics {
        RgbaBufferGraphics {
            width,
            height,
            buffer,
        }
    }

    #[inline]
    fn coords_to_pixel_index(&self, p: &Point) -> usize {
        assert!(p.x < self.width);
        assert!(p.y < self.height);
        p.x + p.y * self.width
    }

    #[inline]
    fn write_color(&mut self, pixel_index: usize, color: &types::Color) {
        let red = piston_color_channel_to_byte(color[0]);
        let green = piston_color_channel_to_byte(color[1]);
        let blue = piston_color_channel_to_byte(color[2]);
        let alpha = piston_color_channel_to_byte(color[3]);

        // pixels are stored in RGBA, so each pixel is 4 bytes
        let slice = unsafe { slice::from_raw_parts_mut(self.buffer, self.width * self.height * 4) };

        let byte_index = pixel_index * 4;

        slice[byte_index] = red;
        slice[byte_index + 1] = green;
        slice[byte_index + 2] = blue;
        slice[byte_index + 3] = alpha;
    }

    fn vertex_to_pixel_coords(&self, v: [f32; 2]) -> Point {
        let vx = v[0];
        let vy = v[1];

        // it seems that the vertices are in a space where 0,0 is the center of the screen and
        // negative y is up.
        // translate into pixel where 0,0 is top left

        let x = if vx < -(self.width as f32) / 2.0 {
            0
        } else if vx > self.width as f32 / 2.0 {
            self.width - 1
        } else {
            (vx + self.width as f32 / 2.0) as usize
        };

        let y = if vy < -(self.height as f32) / 2.0 {
            0
        } else if vy > self.height as f32 / 2.0 {
            self.height - 1
        } else {
            (vy + self.height as f32 / 2.0) as usize
        };

        assert!(x < self.width);
        assert!(y < self.height);

        Point::new(x, y)
    }
}

impl graphics::Graphics for RgbaBufferGraphics {
    type Texture = RgbaTexture;

    fn clear_color(&mut self, color: types::Color) {
        let num_pixels = self.width * self.height;

        for i in 0..num_pixels {
            self.write_color(i, &color);
        }
    }

    fn clear_stencil(&mut self, _value: u8) {
        // TODO
    }

    fn tri_list<F>(&mut self, _draw_state: &graphics::DrawState, color: &[f32; 4], mut f: F) where F: FnMut(&mut FnMut(&[[f32; 2]])) {
        f(&mut |verts: &[[f32; 2]]| {
            for t in 0..verts.len() / 3 {
                let v1 = verts[t * 3];
                let v2 = verts[t * 3 + 1];
                let v3 = verts[t * 3 + 2];

                let tri = Triangle::new(self.vertex_to_pixel_coords(v1),
                                        self.vertex_to_pixel_coords(v2),
                                        self.vertex_to_pixel_coords(v3));

                tri.render(self, color);
            }
        })
    }

    fn tri_list_uv<F>(&mut self, _draw_state: &graphics::DrawState, _color: &[f32; 4], _texture: &<Self as graphics::Graphics>::Texture, _f: F) where F: FnMut(&mut FnMut(&[[f32; 2]], &[[f32; 2]])) {
        super::CanvasRuntime::debug_log("unimplemented: tri_list_uv");
    }
}

/// Maps an f32 in [0_f32, 1.0] to [0_u8, 255]
#[inline]
fn piston_color_channel_to_byte(f: f32) -> u8 {
    (f * 255.0) as u8
}

/// A point in 2d space
#[derive(Clone, Copy)]
struct Point {
    x: usize,
    y: usize,
}

impl Point {
    fn new(x: usize, y: usize) -> Point {
        Point {
            x,
            y,
        }
    }
}

impl fmt::Debug for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

/// A triangle in pixel coordinates
struct Triangle {
    // vertices ordered by increasing y (top to bottom) then x coordinate (left to right)
    vertices: [Point; 3]
}

impl Triangle {
    fn new(v1: Point, v2: Point, v3: Point) -> Triangle {
        let mut buf = [v1, v2, v3];
        buf.sort_unstable_by(|p1, p2| {
            match p1.y.cmp(&p2.y) {
                // if u's are equal, use x
                Ordering::Equal => p1.y.cmp(&p2.y),
                x => x
            }
        });

        Triangle {
            vertices: buf
        }
    }

    fn render(&self, graphics: &mut RgbaBufferGraphics, color: &types::Color) {
        // We want to write in descending scan lines because that's how the memory is laid out.
        // For each scan line between the top and bottom of the triangle, we want to find out which
        // parts of the line fall between 2 edges of the triangle.

        // each edge in increasing y order
        let edges = [
            (self.vertices[0], self.vertices[1]),
            (self.vertices[0], self.vertices[2]),
            (self.vertices[1], self.vertices[2])];

        // TODO this rendering isn't quite right; the way rounding etc is handled skips a few pixels

        // For each edge, if it's horizontal, just draw it as a line.
        for i in 0..edges.len() {
            let edge = edges[i];
            if edge.0.y == edge.1.y {
                // the edge is horizontal so just draw it as a line
                for x in (edge.0.x)..(edge.1.x + 1) {
                    let pixel_index = graphics.coords_to_pixel_index(&Point::new(x, edge.0.y));
                    graphics.write_color(pixel_index, color);
                }

                continue
            }

            // it's not horizontal, so look for any edges after this one to find out if they
            // vertically overlap. (Only look forward to avoid drawing the same areas twice.)
            for j in (i + 1)..edges.len() {
                let other_edge = edges[j];

                if other_edge.0.y == other_edge.1.y {
                    // it's horizontal; it will get drawn later
                    continue
                }

                // because other_edge is later in the edge list, we know that its starting
                // vertex has a y no less than the first edge's y. So, the vertical overlap
                // will be from the second edge's min y to the lesser of the two max y's, which
                // might be the empty set if the smaller of the edge ends is before the other edge's
                // min y.

                let overlap_y_start = other_edge.0.y;
                let overlap_y_end = edge.1.y.min(other_edge.1.y);

                // Inverse slope: how many x units should we move for a unit of y.
                // Can't divide by zero because neither of these is horizontal.
                let edge_1_inv_slope = (edge.1.x as f64 - edge.0.x as f64) / (edge.1.y as f64 - edge.0.y as f64);
                let edge_2_inv_slope = (other_edge.1.x as f64 - other_edge.0.x as f64) / (other_edge.1.y as f64 - other_edge.0.y as f64);

                for y in overlap_y_start..(overlap_y_end + 1) {
                    let edge_1_x = (edge.0.x as f64 + ((y - edge.0.y) as f64 * edge_1_inv_slope)).round() as usize;
                    let edge_2_x = (other_edge.0.x as f64 + ((y - other_edge.0.y) as f64 * edge_2_inv_slope)).round() as usize;

                    let start_x = edge_1_x.min(edge_2_x);
                    let end_x = edge_1_x.max(edge_2_x);

                    for x in start_x..(end_x + 1) {
                        let pixel_index = graphics.coords_to_pixel_index(&Point::new(x, y));
                        graphics.write_color(pixel_index, color);
                    }
                }
            }
        }
    }
}
