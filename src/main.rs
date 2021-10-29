extern crate ncurses;

mod camera;
mod buffer;
mod renderer;
mod sdf;

use std::{f32::consts::PI, time::Instant};
use cgmath::{Angle, Matrix3, Rad, Vector3, Zero};
use ncurses::*;

use camera::Camera;
use buffer::Buffer;
use renderer::Renderer;
use sdf::sd_gear;


fn main() {
    let mut renderer = Renderer {
        buffer:Buffer::from_height(50.0, 3.0),
        camera: Camera::new(
            Vector3::new(-4.0, 0.0, 0.0),
            Vector3::zero(),
            PI / 2.0,
            1.0
        )
    };

    // This closure will later be built
    // by parsing a JSON scene
    let sdf_global = |point: Vector3<f32>, time: f32| -> f32 {
        sd_gear(point, time, Vector3::zero(), 2.0, 0.4, 30.0)
    };

    initscr();

    let mut time = 0.0;

    while true {
        clear();
        flushinp();

        time += 1.0;

        let sdf = |point: Vector3<f32>| -> f32 {
            sdf_global(point, time)
        };

        // Render
        let timestamp = Instant::now();
        renderer.render(&sdf);
        addstr(&format!("\nRendered in {:?} ({:.1} FPS)\n", timestamp.elapsed(), 1.0 / timestamp.elapsed().as_secs_f64()));
        addstr(&format!("Camera: {:?}\n", renderer.camera.position));
        addstr(&format!("Facing: {:?}, Up: {:?}\n", renderer.camera.direction, renderer.camera.up));

        refresh();

        // Handle input
        // TODO: move all bullshit below to a separate file
        let char = getch();
        addstr(&format!("\nPressed: {:?}\n", char));
        refresh();

        if char == 107 { // k to move forward
            renderer.camera.position += renderer.camera.direction * renderer.camera.speed;
        } else if char == 106 { // j to move backward
            renderer.camera.position -= renderer.camera.direction * renderer.camera.speed;
        } else if char == 72 { // H to move left
            renderer.camera.position += Matrix3::from_axis_angle(renderer.camera.up, Rad::turn_div_4()) * renderer.camera.direction * renderer.camera.speed;
        } else if char == 76 { // L to move right
            renderer.camera.position -= Matrix3::from_axis_angle(renderer.camera.up, Rad::turn_div_4()) * renderer.camera.direction * renderer.camera.speed;
        } else if char == 104 { // h to rotate left
            let rotation = Matrix3::from_angle_z(Rad::full_turn() / renderer.camera.turn_rate);
            renderer.camera.direction = rotation * renderer.camera.direction;
            renderer.camera.up = rotation * renderer.camera.up;
        } else if char == 108 { // l to rotate right
            let rotation = Matrix3::from_angle_z(-Rad::full_turn() / renderer.camera.turn_rate);
            renderer.camera.direction = rotation * renderer.camera.direction;
            renderer.camera.up = rotation * renderer.camera.up;
        } else if char == 75 { // K to rotate up
            let axis = renderer.camera.up.cross(renderer.camera.direction);
            let angle = -Rad::full_turn() / renderer.camera.turn_rate;
            let rotation = Matrix3::from_axis_angle(axis, angle);
            renderer.camera.up = rotation * renderer.camera.up;
            renderer.camera.direction = rotation * renderer.camera.direction;
        } else if char == 74 { // J to rotate down
            let axis = renderer.camera.up.cross(renderer.camera.direction);
            let angle = Rad::full_turn() / renderer.camera.turn_rate;
            let rotation = Matrix3::from_axis_angle(axis, angle);
            renderer.camera.up = rotation * renderer.camera.up;
            renderer.camera.direction = rotation * renderer.camera.direction;
        } else if char == 117 { // u to move up along Z
            renderer.camera.position += Vector3::unit_z() * renderer.camera.speed;
        } else if char == 100 { // d to move down along Z
            renderer.camera.position -= Vector3::unit_z() * renderer.camera.speed;
        } else if char == 70 { // F to reverse camera direction
            renderer.camera.direction = -renderer.camera.direction;
        }
    }

    endwin();
}
