mod camera;
mod canvas;
use std::f32::consts::PI;
use cgmath::Vector3;

use crate::camera::{Buffer, Camera};

fn main() {
    let mut cam = Camera {
        position: Vector3 { x: 0.0, y: -0.7, z: 0.0 },
        direction: Vector3 { x: 1.0, y: 0.0, z: 0.0 },
        angle: PI / 2.0,
        distance: 1.0,
        aspect_ratio: 1.0,
        brightness: 10.0,
        buffer: Buffer([['.'; 120]; 60])
    };

    cam.render();
    println!("{}", cam.buffer);
}
