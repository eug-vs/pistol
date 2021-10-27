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
        brightness: 5.0,
        buffer: Buffer([['.'; 120]; 60]),
        time: 0.0,
    };

    for _round in 0..1 {
        for i in 0..60 {
            // 1 sin round
            cam.time = (i as f32 / 60.0) * 2.0 * PI;
            cam.render();
            println!("{}", cam.buffer);
        }
    }
}
