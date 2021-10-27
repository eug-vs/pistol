use cgmath::Vector3;
use cgmath::prelude::*;
use std::{cmp::{max, min}, f32::consts::PI, fmt};

type Vector = Vector3<f32>;

pub fn rotate_z(vec: Vector, angle: f32) -> Vector {
    return Vector {
        x: vec.x * angle.cos() + vec.y * angle.sin(),
        y: vec.y * angle.cos() - vec.x * angle.sin(),
        z: vec.z,
    }
}

pub const HEIGHT: i32 = 40;
pub const WIDTH: i32 = HEIGHT * 3;

#[derive(Debug)]
pub struct Buffer (pub [[char; WIDTH as usize]; HEIGHT as usize]);

impl fmt::Display for Buffer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for i in 0..HEIGHT as usize {
            for j in 0..WIDTH as usize {
                write!(f, "{}", self.0[i][j])?;
            }
            writeln!(f)?;
        }
        write!(f, "")
    }
}

#[derive(Debug)]
pub struct Camera {
    pub time: f32,
    pub position: Vector,
    pub direction: Vector,
    pub angle: f32,
    pub distance: f32,
    pub brightness: f32,
    pub aspect_ratio: f32,
    pub buffer: Buffer,
}

fn softmin(left: f32, right: f32, k: f32) -> f32 {
    // return left.min(right);
    let h = (k-(left-right).abs()).max(0.0) / k;
    return left.min(right) - h*h*k*(1.0/4.0);
}

impl Camera {
    pub fn sdf(&self, point: Vector) -> f32 {
         // Floor at z = -2
        let floor_dist = point.z + 1.0;

        // Sphere
        let center = Vector { x: 4.0, y: 0.0, z: 0.0 };
        let radius = 1.0 + 0.5 * self.time.sin();
        let sphere_dist = (point - center).magnitude() - radius;

        // Small sphere
        let center2 = Vector { x: 3.5, y: 0.5, z: 0.0 };
        let radius2 = 0.7;
        let sphere2_dist = (point - center2).magnitude() - radius2;

        // Second sphere
        let center3 = Vector { x: 4.0 + self.time.sin() * 1.6, y: -2.5, z: 0.0 - self.time.sin() * 0.8 };
        let radius3 = 1.0;
        let sphere3_dist = (point - center3).magnitude() - radius3;

        softmin(
            softmin(
                sphere_dist.max(-sphere2_dist),
                sphere3_dist,
                1.5
            ),
            floor_dist,
            0.8
        )
    }

    pub fn rorate_around_point(& mut self, point: Vector) {
        let rotations_per_round = 2.0;
        self.position = rotate_z(self.position - point, rotations_per_round * 2.0 * PI / 60.0) + point;
        self.direction = (point - self.position).normalize();
    }

    pub fn screen(&self) -> (f32, f32) {
        let width = self.distance * 2.0 * (self.angle / 2.0).tan();
        let height = width * self.aspect_ratio;
        // println!("Screen {}x{} units", width, height);
        (width, height)
    }
    pub fn render(& mut self) {
        self.rorate_around_point(Vector { x: 4.0, y: 0.0, z: 0.0 });

        let palette = "$@B%8&WM#oahkbdpqwmZO0QLCJUYXzcvunxrjft/\\|()1{}[]?-_+~<>i!lI;:,\"^`'. ";
        let (screen_width, screen_height) = self.screen();

        let h_step = rotate_z(self.direction, PI / 2.0).normalize() * screen_width / WIDTH as f32;
        let v_step = Vector { x: 0.0, y: 0.0, z: -screen_height / HEIGHT as f32 };
        // println!("Steps: h{}, v{}", h_step, v_step);

        // Initialize with a corner
        let point = self.position + (self.direction.normalize() * self.distance) - (h_step * (WIDTH / 2) as f32 + v_step * (HEIGHT / 2) as f32);
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

        Vector { x: dfdx, y: dfdy, z: dfdz }.normalize()
    }

    pub fn shoot_ray(&self, direction: Vector) -> f32 {
        let light = Vector { x: 1.0, y: 1.0, z: -1.0 }.normalize();
        let threshold = 0.1;

        let ray = direction.normalize();
        let mut point = self.position;
        let mut dist = 0.0;
        let mut count = 0;

        while dist < self.brightness && count < 50 {
            count += 1;
            dist = self.sdf(point);
            if dist < threshold {
                // Collision in point! Let's calculate lights now:
                let mut res: f32 = 1.0;
                let mut t = 0.001;
                let k = 2.0;

                while t < 7.0 {
                    let h = self.sdf(point - light * t);
                    if h < 0.001 {
                        return 0.97
                    }
                    res = res.min(k * h / t);
                    t += h;

                }

                let normal = self.normal(point);
                let dot = -(normal.dot(light));

                return 1.0 - 0.5 * dot.max(0.01).min(0.98) - 0.5 * res.min(0.98)
            }
            point = point + ray * dist;
        }

        return 1.0
    }
}
