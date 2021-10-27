extern crate ncurses;

mod camera;
mod canvas;
use std::{f32::consts::PI, time::Instant};
use cgmath::Vector3;
use ncurses::*;

use crate::camera::{Buffer, Camera, WIDTH, HEIGHT, rotate_z};

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

    let time = 0;

    while true {
        clear();
        flushinp();

        // Render
        cam.time = (time as f32 / 60.0) * 2.0 * PI;
        let timestamp = Instant::now();
        cam.render();
        addstr(&cam.buffer.to_string());
        addstr(&format!("\nRendered in {:?}\n", timestamp.elapsed()).to_string());
        refresh();

        // Handle input
        let char = getch();

        let cam_speed = 0.5;
        let cam_turn_rate = 30.0;

        if char == 106 || char == 74 {
            cam.position -= cam.direction * cam_speed;
        } else if char == 107 || char == 75 {
            cam.position += cam.direction * cam_speed;
        } else if char == 104 {
            cam.direction = rotate_z(cam.direction, -2.0 * PI / cam_turn_rate);
        } else if char == 108 {
            cam.direction = rotate_z(cam.direction, 2.0 * PI / cam_turn_rate);
        } else if char == 72 {
            cam.position -= rotate_z(cam.direction, PI / 2.0) * cam_speed;
        } else if char == 76 {
            cam.position += rotate_z(cam.direction, PI / 2.0) * cam_speed;
        } else if char == 70 { // F
            cam.direction = -cam.direction;
        }
    }

    endwin();
}
