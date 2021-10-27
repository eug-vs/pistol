extern crate ncurses;

mod camera;
mod canvas;
use std::{f32::consts::PI, time::Instant};
use cgmath::{Angle, InnerSpace, Matrix3, Rad, Vector3, Zero};
use ncurses::*;

use crate::camera::{Buffer, Camera, WIDTH, HEIGHT};

fn main() {
    let mut cam = Camera {
        position: Vector3::zero(),
        direction: Vector3::unit_x(),
        up: Vector3::unit_z(),
        light: Vector3 { x: 1.0, y: 1.0, z: -1.0 }.normalize(),
        angle: PI / 2.0,
        distance: 1.0,
        aspect_ratio: 2.0 * HEIGHT as f32 / WIDTH as f32,
        brightness: 5.0,
        buffer: Buffer([[' '; WIDTH as usize]; HEIGHT as usize]),
        time: 0.0,
        speed: 0.5,
        turn_rate: 30.0,
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
        addstr(&format!("\nRendered in {:?} ({:.0} FPS)\n", timestamp.elapsed(), 1.0 / timestamp.elapsed().as_secs_f64()));
        addstr(&format!("Camera: {:?}\n", cam.position));
        addstr(&format!("Facing: {:?}, Up: {:?}\n", cam.direction, cam.up));
        addstr(&format!("Light: {:?}\n", cam.light));
        refresh();

        // Handle input
        let char = getch();
        addstr(&format!("\nPressed: {:?}\n", char));
        refresh();

        if char == 107 { // k to move forward
            cam.position += cam.direction * cam.speed;
        } else if char == 106 { // j to move backward
            cam.position -= cam.direction * cam.speed;
        } else if char == 72 { // H to move left
            cam.position += Matrix3::from_axis_angle(cam.up, Rad::turn_div_4()) * cam.direction * cam.speed;
        } else if char == 76 { // L to move right
            cam.position -= Matrix3::from_axis_angle(cam.up, Rad::turn_div_4()) * cam.direction * cam.speed;
        } else if char == 104 { // h to rotate left
            let rotation = Matrix3::from_angle_z(Rad::full_turn() / cam.turn_rate);
            cam.direction = rotation * cam.direction;
            cam.up = rotation * cam.up;
        } else if char == 108 { // l to rotate right
            let rotation = Matrix3::from_angle_z(-Rad::full_turn() / cam.turn_rate);
            cam.direction = rotation * cam.direction;
            cam.up = rotation * cam.up;
        } else if char == 75 { // K to rotate up
            let axis = cam.up.cross(cam.direction);
            let angle = -Rad::full_turn() / cam.turn_rate;
            let rotation = Matrix3::from_axis_angle(axis, angle);
            cam.up = rotation * cam.up;
            cam.direction = rotation * cam.direction;
        } else if char == 74 { // J to rotate down
            let axis = cam.up.cross(cam.direction);
            let angle = Rad::full_turn() / cam.turn_rate;
            let rotation = Matrix3::from_axis_angle(axis, angle);
            cam.up = rotation * cam.up;
            cam.direction = rotation * cam.direction;
        } else if char == 117 { // u to move up along Z
            cam.position += Vector3::unit_z() * cam.speed;
        } else if char == 100 { // d to move down along Z
            cam.position -= Vector3::unit_z() * cam.speed;
        } else if char == 70 { // F to reverse camera direction
            cam.direction = -cam.direction;
        } else if char == 101 { // e to change lights
            cam.light = Matrix3::from_angle_z(Rad::turn_div_2() / cam.turn_rate) * cam.light;
        } else if char == 69 { // E to change lights vertically
            cam.light = Matrix3::from_angle_y(Rad::turn_div_2() / cam.turn_rate) * cam.light;
        }
    }

    endwin();
}
