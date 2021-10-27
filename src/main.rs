extern crate ncurses;

mod camera;
mod canvas;
use std::{f32::consts::PI, time::Instant};
use cgmath::{Angle, InnerSpace, Matrix3, Rad, Vector3};
use ncurses::*;

use crate::camera::{Buffer, Camera, WIDTH, HEIGHT};

fn main() {
    let mut cam = Camera {
        position: Vector3 { x: 0.0, y: -0.7, z: 0.0 },
        direction: Vector3 { x: 1.0, y: 0.0, z: 0.0 }.normalize(),
        light: Vector3 { x: 1.0, y: 1.0, z: -1.0 }.normalize(),
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
        addstr(&format!("Camera: {:?}\n", cam.position));
        addstr(&format!("Facing: {:?}\n", cam.direction));
        addstr(&format!("Light: {:?}\n", cam.light));
        refresh();

        // Handle input
        let char = getch();
        addstr(&format!("\nPressed: {:?}\n", char));
        refresh();

        let cam_speed = 0.5;
        let cam_turn_rate = 30.0;

        if char == 106 || char == 74 {
            cam.position -= cam.direction * cam_speed;
        } else if char == 107 || char == 75 {
            cam.position += cam.direction * cam_speed;
        } else if char == 104 {
            cam.direction = Matrix3::from_angle_z(-Rad::full_turn() / cam_turn_rate) * cam.direction;
        } else if char == 108 {
            cam.direction = Matrix3::from_angle_z(Rad::full_turn() / cam_turn_rate) * cam.direction;
        } else if char == 72 {
            cam.position -= Matrix3::from_angle_z(Rad::turn_div_4()) * cam.direction * cam_speed;
        } else if char == 76 {
            cam.position += Matrix3::from_angle_z(Rad::turn_div_4()) * cam.direction * cam_speed;
        } else if char == 70 { // F to reverse camera direction
            cam.direction = -cam.direction;
        } else if char == 101 { // e to change lights
            cam.light = Matrix3::from_angle_z(Rad::turn_div_2() / cam_turn_rate) * cam.light;
        } else if char == 69 { // E to change lights vertically
            cam.light = Matrix3::from_angle_y(Rad::turn_div_2() / cam_turn_rate) * cam.light;
        }
    }

    endwin();
}
