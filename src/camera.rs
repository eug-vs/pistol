use cgmath::Matrix3;
use cgmath::Rad;
use cgmath::Vector3;
use cgmath::prelude::*;
use std::fmt;

type Vector = Vector3<f32>;

pub const HEIGHT: i32 = 30;
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
    pub up: Vector,
    pub light: Vector,
    pub angle: f32,
    pub distance: f32,
    pub brightness: f32,
    pub aspect_ratio: f32,
    pub buffer: Buffer,
    pub speed: f32,
    pub turn_rate: f32,
}

fn softmin(left: f32, right: f32, k: f32) -> f32 {
    // return left.min(right);
    let h = (k-(left-right).abs()).max(0.0) / k;
    return left.min(right) - h*h*k*(1.0/4.0);
}

fn sd_sphere(point: Vector, center: Vector, radius: f32) -> f32 {
    (point - center).magnitude() - radius
}

fn sd_box(point: Vector, center: Vector, size: Vector) -> f32 {
    let diff = center - point;
    let q = diff.map(|n| n.abs()) - size / 2.0;
    return q.map(|n| n.max(0.0)).magnitude() + (q.y.max(q.z).max(q.x)).min(0.0)
}


impl Camera {
    pub fn sd_gear(&self, point: Vector, center: Vector, radius: f32, thickness: f32) -> f32 {
        let mut dist: f32;

        // Ring
        {
            let cylinder_dist = (Vector::new(0.0, point.y, point.z) - center).magnitude() - (radius - thickness / 4.0);
            dist = cylinder_dist.abs() - thickness / 2.0; // Make cylinder hollow
        }

        // Teeth
        {
            let center = Vector { x: 0.0, y: radius + thickness / 2.0, z: 0.0 };
            let size = Vector::new(thickness, thickness * 2.0, thickness);

            let sector_angle = Rad::full_turn() / 12.0;
            let point_angle = (point.z / point.y).atan();
            let sector = (point_angle / sector_angle.0).round();
            let mut mapped_point = Matrix3::from_angle_x(-sector_angle * sector) * point;
            mapped_point.y = mapped_point.y.abs();


            // Make teeth smooth by subtracting some amount
            dist = dist.min(sd_box(mapped_point, center, size) - thickness / 4.0);
        }

        // Tak a slice
        dist = dist.max(point.x.abs() - thickness / 2.0);

        return dist;
    }
    pub fn sdf(&self, point: Vector) -> f32 {
        self.sd_gear(point, Vector::zero(), 3.0, 0.6)
    }

    pub fn screen(&self) -> (f32, f32) {
        let width = self.distance * 2.0 * (self.angle / 2.0).tan();
        let height = width * self.aspect_ratio;
        // println!("Screen {}x{} units", width, height);
        (width, height)
    }

    pub fn render(& mut self) {
        let palette = "$@B%8&WM#oahkbdpqwmZO0QLCJUYXzcvunxrjft/\\|()1{}[]?-_+~<>i!lI;:,\"^`'. ".to_string();
        let (screen_width, screen_height) = self.screen();

        let cross = self.up.cross(self.direction);

        // Linear transormation operator for calculating screen position
        // Assumes "initial" screen is perpendicular to OX
        // and it's bottom edge is parallel to OY
        let operator = Matrix3::from_cols(
            self.direction * self.distance,
            cross * screen_width,
            self.up * screen_height,
        );

        for i in 0..HEIGHT as usize {
            let ix = i as f32 / HEIGHT as f32;
            for j in 0..WIDTH as usize {
                let jx = j as f32 / WIDTH as f32;
                // Apply transform to unit square centered at (1, 0, 0)
                let ray_dir = operator * Vector { x: 1.0, y: 0.5 - jx, z: 0.5 - ix };

                let collision = self.ray_marching(self.position, ray_dir);

                let brightness = match collision {
                    Some(point) => self.light_point(point),
                    None => 0.0
                };

                self.buffer.0[i][j] = palette
                    .chars()
                    .nth(((1.0 - brightness) * (palette.len() - 1) as f32) as usize)
                    .unwrap();
            }
        }
    }

    pub fn normal(&self, point: Vector) -> Vector {
        let d = 0.001;

        let dx = Vector::unit_x() * d;
        let dy = Vector::unit_y() * d;
        let dz = Vector::unit_z() * d;

        return (Vector {
            x: (self.sdf(point + dx) - self.sdf(point - dx)),
            y: (self.sdf(point + dy) - self.sdf(point - dy)),
            z: (self.sdf(point + dz) - self.sdf(point - dz)),
        } / (2.0 * d)).normalize()
    }

    pub fn ray_marching(&self, origin: Vector, direction: Vector) -> Option<Vector> {
        let threshold = 0.1;

        let ray = direction.normalize();
        let mut point = origin;
        let mut dist = 0.0;
        let mut count = 0;

        while dist < 10.0 && count < 30 {
            count += 1;
            dist = self.sdf(point);
            if dist.abs() < threshold {
                return Some(point);
            }
            point += ray * dist;
        }

        return None
    }

    pub fn light_point(&self, point: Vector) -> f32 {
        let ambient = 0.1;
        return ambient + (1.0 - ambient) * (self.diffuse_lighting(point) * 0.7 + self.specular_lighting(point) * 0.3)
    }

    pub fn diffuse_lighting(&self, point: Vector) -> f32 {
        let mut res: f32 = 1.0;
        let mut ph = 1e20;
        let mut t = 0.001;
        let k = 4.0;

        while t < 7.0 {
            let h = self.sdf(point - self.light * t);
            if h < 0.001 {
                return 0.00
            }
            let y = h * h / (2.0 * ph);
            let d = (h * h - y * y).sqrt();
            res = res.min(k * d / (t - y).max(0.0));
            t += h;
            ph = h;
        }

        return res
    }

    pub fn specular_lighting(&self, point: Vector) -> f32 {
        let normal = self.normal(point);
        let dot = -(normal.dot(self.light));
        return dot.min(1.0).max(0.0)
    }
}
