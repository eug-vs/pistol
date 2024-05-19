use cgmath::prelude::*;
use cgmath::{Matrix3, Vector3};

use crate::screen::ScreenIterator;
type Vector = Vector3<f32>;

// The physical screen that camera projects onto
#[derive(Debug, Copy, Clone)]
pub struct Screen {
    pub width: f32,
    pub height: f32,
}

#[derive(Debug, Copy, Clone)]
pub struct Camera {
    pub position: Vector,
    pub direction: Vector,
    pub up: Vector,
    pub distance: f32,
    pub speed: f32,
    pub turn_rate: f32,
    pub screen: Screen,
}

const ASPECT_RATIO: f32 = 2.0 / 3.0;

impl Camera {
    pub fn new(position: Vector, look_at: Vector, angle: f32, distance: f32) -> Self {
        let width = distance * 2.0 * (angle / 2.0).tan();
        let height = width * ASPECT_RATIO;

        Self {
            position,
            direction: (look_at - position).normalize(),
            up: Vector::unit_z(),
            distance,
            screen: Screen { width, height },
            speed: 0.15,
            turn_rate: 90.0,
        }
    }
    pub fn get_screen_iterator(self) -> ScreenIterator {
        // Linear transormation operator for calculating screen position
        // Assumes "initial" screen is perpendicular to OX
        // and it's bottom edge is parallel to OY
        let operator = Matrix3::from_cols(
            self.direction * self.distance,
            self.direction.cross(self.up) * self.screen.width,
            -self.up * self.screen.height,
        );

        ScreenIterator::from_screen_position(
            operator * Vector::new(1.0, -0.5, -0.5),
            operator * Vector::unit_z(),
            operator * Vector::unit_y(),
        )
    }
}
