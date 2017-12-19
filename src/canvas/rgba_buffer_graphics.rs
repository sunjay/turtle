use std::slice;

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

    fn vertex_to_pixel_index(&self, v: [f32; 2]) -> usize {
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

        y * self.width + x
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
        f(&mut |verts| {
            for t in 0..verts.len() / 3 {
                let v1 = verts[t];
                let v2 = verts[t + 1];
                let v3 = verts[t + 2];

                // take a stab at what the pixels for the corners of the triangle are
                let p1 = self.vertex_to_pixel_index(v1);
                let p2 = self.vertex_to_pixel_index(v2);
                let p3 = self.vertex_to_pixel_index(v3);

                self.write_color(p1, color);
                self.write_color(p2, color);
                self.write_color(p3, color);
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
