use cgmath::{Vector3, Zero};

use crate::buffer::Buffer;
type Vector = Vector3<f32>;

#[derive(Debug)]
pub struct ScreenIterator {
    pub corner_position: Vector,
    pub vertical_edge: Vector,
    pub horizontal_edge: Vector,
    pub horizontal_step: Vector,
    pub vertical_step: Vector,
    buffer_width: usize,
    buffer_height: usize,
    current_index: usize,
}

impl ScreenIterator {
    pub fn from_screen_position(
        corner_position: Vector,
        vertical_edge: Vector,
        horizontal_edge: Vector,
    ) -> Self {
        Self {
            corner_position,
            vertical_edge,
            horizontal_edge,
            vertical_step: Vector::zero(),
            horizontal_step: Vector::zero(),
            current_index: 0,
            buffer_width: 0,
            buffer_height: 0,
        }
    }
    pub fn set_buffer_size(&mut self, buffer: &Buffer) {
        self.buffer_width = buffer.width as usize;
        self.buffer_height = buffer.height as usize;

        self.horizontal_step = self.horizontal_edge / buffer.width;
        self.vertical_step = self.vertical_edge / buffer.height;
    }
}

impl Iterator for ScreenIterator {
    type Item = Vector;
    fn next(&mut self) -> Option<Self::Item> {
        let pixel_x = self.current_index % self.buffer_width;
        let pixel_y = self.current_index / self.buffer_width;

        if self.current_index < self.buffer_width * self.buffer_height {
            self.current_index += 1;
            Some(
                self.corner_position
                    + self.horizontal_step * pixel_x as f32
                    + self.vertical_step * pixel_y as f32,
            )
        } else {
            None
        }
    }
}
