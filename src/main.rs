mod vector;
mod camera;
mod canvas;
use std::f32::consts::PI;

use crate::{camera::{Buffer, Camera}, vector::Vector};

trait Object {
    fn sdf(&self, point: Vector) -> f32;
}

fn main() {
    let mut cam = Camera {
        position: Vector { x: 0.0, y: -0.7, z: 0.0 },
        direction: Vector { x: 1.0, y: 0.0, z: 0.0 },
        angle: PI / 2.0,
        distance: 1.0,
        aspect_ratio: 1.0,
        brightness: 10.0,
        buffer: Buffer([['.'; 120]; 60])
    };

    cam.render();
    println!("{}", cam.buffer);
}
