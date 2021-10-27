extern crate ncurses;

mod camera;
mod canvas;
use std::f32::consts::PI;
use cgmath::Vector3;
use ncurses::*;

use crate::camera::{Buffer, Camera, WIDTH, HEIGHT};

fn main() {
    let mut cam = Camera {
        position: Vector3 { x: 0.0, y: -0.7, z: 0.0 },
        direction: Vector3 { x: 1.0, y: 0.0, z: 0.0 },
        angle: PI / 2.0,
        distance: 1.0,
        aspect_ratio: 2.0 * HEIGHT as f32 / WIDTH as f32,
        brightness: 5.0,
        buffer: Buffer([[' '; WIDTH as usize]; HEIGHT as usize]),
        time: 0.0,
    };

    initscr();

    for _round in 0..20 {
        for i in 0..60 {
            cam.time = (i as f32 / 60.0) * 2.0 * PI;
            cam.render();

            clear();
            addstr(&cam.buffer.to_string());
            refresh();
        }
    }

    endwin();
}
