use crate::vector::Vector;
use std::{cmp::{max, min}, f32::consts::PI, f64::MAX_EXP, fmt};

const WIDTH: i32 = 100;
const HEIGHT: i32 = 50;

#[derive(Debug)]
pub struct Buffer (pub [[char; 100]; 50]);

impl fmt::Display for Buffer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for i in 0..50 {
            for j in 0..100 {
                write!(f, "{}", self.0[i][j])?;
            }
            writeln!(f)?;
        }
        write!(f, "")
    }
}

#[derive(Debug)]
pub struct Camera {
    pub position: Vector,
    pub direction: Vector,
    pub angle: f32,
    pub distance: f32,
    pub brightness: f32,
    pub aspect_ratio: f32,
    pub buffer: Buffer,
}

impl Camera {
    pub fn sdf(&self, point: Vector) -> f32 {
        // Sphere
        let center = Vector { x: 3.0, y: 0.0, z: 0.0 };
        let radius = 1.0;

        let sphere_dist = (point - center).magnitude() - radius;

        let floor_dist = point.z + 1.0; // Floor at z = -1

        sphere_dist.min(floor_dist)
    }

    pub fn screen(&self) -> (f32, f32) {
        let width = self.distance * 2.0 * (self.angle / 2.0).tan();
        let height = width * self.aspect_ratio;
        // println!("Screen {}x{} units", width, height);
        (width, height)
    }
    pub fn render(& mut self) {
        let palette = "$@B%8&WM#oahkbdpqwmZO0QLCJUYXzcvunxrjft/\\|()1{}[]?-_+~<>i!lI;:,\"^`'.";
        let (screen_width, screen_height) = self.screen();

        let h_step = self.direction.rotate_z(PI / 2.0).normalized() * screen_width / WIDTH as f32;
        let v_step = Vector { x: 0.0, y: 0.0, z: -screen_height / HEIGHT as f32 };
        // println!("Steps: h{}, v{}", h_step, v_step);

        // Initialize with a corner
        let point = self.position + (self.direction.normalized() * self.distance) - (h_step * (WIDTH / 2) as f32 + v_step * (HEIGHT / 2) as f32);
        // println!("Corner: {}", point);

        let mut ray_dir = point - self.position;

        for i in 0..HEIGHT as usize {
            ray_dir = ray_dir + v_step;
            for j in 0..WIDTH as usize {
                ray_dir = ray_dir + h_step;


                let brightness = self.shoot_ray(ray_dir);
                self.buffer.0[i][j] = palette.chars().nth((brightness * palette.len() as f32) as usize - 1).unwrap();
                // println!("[{}, {}]: {}", i, j, ray_dir);
            }
            ray_dir = ray_dir - h_step * WIDTH as f32;
        }
    }

    pub fn shoot_ray(&self, direction: Vector) -> f32 {
        let threshold = 0.1;

        let ray = direction.normalized();
        let mut point = self.position;
        let mut dist = 0.0;
        let mut count = 0;

        while dist < self.brightness && count < 25 {
            count += 1;
            dist = self.sdf(point);
            if dist < threshold {
                // println!("Dist: {}, point {}", dist, point);
                return (point - self.position).magnitude() / self.brightness
            }
            point = point + ray * dist;
        }

        return 1.0
    }
}
