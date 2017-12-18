use std::slice;

use graphics::{self, types};

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
    buffer: *mut u8
}

impl RgbaBufferGraphics {
    pub fn new(width: usize, height: usize, buffer: *mut u8) -> RgbaBufferGraphics {
        RgbaBufferGraphics {
            width,
            height,
            buffer
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

        slice[pixel_index] = red;
        slice[pixel_index + 1] = green;
        slice[pixel_index + 2] = blue;
        slice[pixel_index + 3] = alpha;
    }
}

impl graphics::Graphics for RgbaBufferGraphics {
    type Texture = RgbaTexture;

    fn clear_color(&mut self, color: types::Color) {
        let num_pixels = self.width * self.height;

        let mut pixel_index = 0;
        for _ in 0..num_pixels {
            self.write_color(pixel_index, &color);

            pixel_index += 4;
        }
    }

    fn clear_stencil(&mut self, _value: u8) {
        // TODO
    }

    fn tri_list<F>(&mut self, _draw_state: &graphics::DrawState, _color: &[f32; 4], mut _f: F) where F: FnMut(&mut FnMut(&[[f32; 2]])) {
        // TODO
    }

    fn tri_list_uv<F>(&mut self, _draw_state: &graphics::DrawState, _color: &[f32; 4], _texture: &<Self as graphics::Graphics>::Texture, _f: F) where F: FnMut(&mut FnMut(&[[f32; 2]], &[[f32; 2]])) {
        // TODO
    }
}

/// Maps an f32 in [0_f32, 1.0] to [0_u8, 255]
#[inline]
fn piston_color_channel_to_byte(f: f32) -> u8 {
    (f * 255.0) as u8
}


