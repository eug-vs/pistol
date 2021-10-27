use cgmath::Matrix3;
use cgmath::Rad;
use cgmath::Vector3;
use cgmath::prelude::*;
use std::{cmp::{max, min}, f32::consts::PI, fmt};

type Vector = Vector3<f32>;

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
    pub light: Vector,
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

fn sphere(point: Vector, center: Vector, radius: f32) -> f32 {
    (point - center).magnitude() - radius
}

fn r#box(point: Vector, center: Vector, size: Vector) -> f32 {
    let diff = center - point;
    let q = diff.map(|n| n.abs()) - size / 2.0;
    return q.map(|n| n.max(0.0)).magnitude() + (q.y.max(q.z).max(q.x)).min(0.0)
}


impl Camera {
    pub fn sdf(&self, point: Vector) -> f32 {
        let mut dist: f32;
         // Floor at z = -2
        let floor_dist = point.z + 1.0;

        dist = floor_dist;

        // Sphere
        {
            let center = Vector { x: 4.0, y: 0.0, z: 0.0 };
            let radius = 1.5;
            dist = softmin(dist, sphere(point, center, radius), 1.2);
        }

        // Hole
        {
            let center = Vector { x: 4.0, y: 0.0, z: 0.0 };
            let size = Vector::new(5.0, 2.0, 2.0);
            dist = dist.max(-r#box(point, center, size));
        }

        // Windows
        {
            let center = Vector { x: 4.0, y: 0.0, z: 0.0 };
            let size = Vector::new(1.0, 5.0, 1.0);
            dist = dist.max(-(r#box(point, center, size)));
        }

        return dist
    }

    pub fn screen(&self) -> (f32, f32) {
        let width = self.distance * 2.0 * (self.angle / 2.0).tan();
        let height = width * self.aspect_ratio;
        // println!("Screen {}x{} units", width, height);
        (width, height)
    }

    pub fn render(& mut self) {
        let palette = "$@B%8&WM#oahkbdpqwmZO0QLCJUYXzcvunxrjft/\\|()1{}[]?-_+~<>i!lI;:,\"^`'. ";
        let (screen_width, screen_height) = self.screen();

        let h_step = Matrix3::from_angle_z(Rad::turn_div_4()) * self.direction * screen_width / WIDTH as f32;
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
                    let h = self.sdf(point - self.light * t);
                    if h < 0.001 {
                        return 0.97
                    }
                    res = res.min(k * h / t);
                    t += h;

                }

                let normal = self.normal(point);
                let dot = -(normal.dot(self.light));

                return 1.0 - 0.5 * dot.max(0.01).min(0.98) - 0.5 * res.min(0.98)
            }
            point = point + ray * dist;
        }

        return 1.0
    }
}
