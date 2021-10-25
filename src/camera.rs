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
         // Floor at z = -2
        let floor_dist = point.z + 1.0;

        // Sphere
        let center = Vector { x: 3.0, y: 0.0, z: 0.0 };
        let radius = 1.0;
        let sphere_dist = (point - center).magnitude() - radius;


        // Small sphere
        let center2 = Vector { x: 2.5, y: 0.5, z: 0.0 };
        let radius2 = 0.7;
        let sphere2_dist = (point - center2).magnitude() - radius2;

        sphere_dist.max(-sphere2_dist).min(floor_dist)
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

    pub fn normal(&self, point: Vector) -> Vector {
        let d = 0.01;

        let dx = Vector { x: d, y: 0.0, z: 0.0 };
        let dfdx = (self.sdf(point + dx) - self.sdf(point - dx)) / (2.0 * d);

        let dy = Vector { x: 0.0, y: d, z: 0.0 };
        let dfdy = (self.sdf(point + dy) - self.sdf(point - dy)) / (2.0 * d);

        let dz = Vector { x: 0.0, y: 0.0, z: d };
        let dfdz = (self.sdf(point + dz) - self.sdf(point - dz)) / (2.0 * d);

        Vector { x: dfdx, y: dfdy, z: dfdz }.normalized()
    }

    pub fn shoot_ray(&self, direction: Vector) -> f32 {
        let light = Vector { x: 1.0, y: 1.0, z: -1.0 }.normalized();
        let threshold = 0.1;

        let ray = direction.normalized();
        let mut point = self.position;
        let mut dist = 0.0;
        let mut count = 0;

        while dist < self.brightness && count < 30 {
            count += 1;
            dist = self.sdf(point);
            if dist < threshold {
                // Collision in point! Let's calculate lights now:
                let normal = self.normal(point);
                let dot = -(normal * light);
                return 1.0 - dot.max(0.01).min(0.98);
            }
            point = point + ray * dist;
        }

        return 1.0
    }
}
