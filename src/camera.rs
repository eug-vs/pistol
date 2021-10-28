use cgmath::Matrix3;
use cgmath::Vector3;
use cgmath::prelude::*;
use ncurses::addch;
use ncurses::addstr;
use std::f32;
use std::f32::consts::PI;
use std::time::Instant;

type Vector = Vector3<f32>;

pub const HEIGHT: i32 = 50;
pub const WIDTH: i32 = HEIGHT * 3;

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
    pub speed: f32,
    pub turn_rate: f32,
    pub width: f32,
    pub height: f32,
    pub palette: Vec<char>,
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
    pub fn sd_gear(&self, point: Vector, center: Vector, radius: f32, thickness: f32, turn_rate: f32) -> f32 {
        let mut dist: f32;

        let thickness_over_2 = thickness / 2.0;
        let thickness_over_4 = thickness / 4.0;

        // Ring
        {
            let cylinder_dist = (Vector::new(0.0, point.y, point.z) - center).magnitude() - (radius - thickness_over_4);
            dist = cylinder_dist.abs() - thickness_over_2; // Make cylinder hollow
        }

        // Teeth
        {
            let sector_angle: f32 = 2.0 * PI / 12.0;

            // Account for rotation with time
            let angle = sector_angle * self.time / turn_rate;
            let rotated_point = Vector::new(
                point.x,
                point.y * angle.cos() - point.z * angle.sin(),
                point.y * angle.sin() + point.z * angle.cos()
            );

            // Map all space to the first sector
            let point_angle = (rotated_point.z / rotated_point.y).atan();
            let angle2 = -sector_angle * (point_angle / sector_angle).round();

            let mapped_point = Vector::new(
                rotated_point.x,
                (rotated_point.y * angle2.cos() - rotated_point.z * angle2.sin()).abs(),
                rotated_point.y * angle2.sin() + rotated_point.z * angle2.cos()
            );

            let center = Vector { x: 0.0, y: radius + thickness_over_2, z: 0.0 };
            let size = Vector::new(thickness, thickness * 2.0, thickness);
            // Make teeth smooth by subtracting some amount
            dist = dist.min(sd_box(mapped_point, center, size) - thickness_over_4);
        }

        // Take a slice
        dist = dist.max(point.x.abs() - thickness_over_2);

        return dist;
    }
    pub fn sdf(&self, point: Vector) -> f32 {
        self.sd_gear(point, Vector::zero(), 3.0, 0.6, 10.0)
    }

    pub fn render(& mut self) {
        // Linear transormation operator for calculating screen position
        // Assumes "initial" screen is perpendicular to OX
        // and it's bottom edge is parallel to OY
        let operator = Matrix3::from_cols(
            self.direction * self.distance,
            self.direction.cross(self.up) * self.width,
            -self.up * self.height,
        );

        let mut ray_dir = operator * Vector::new(1.0, -0.5, -0.5); // Corner
        let step_v = operator * Vector3::unit_z() / HEIGHT as f32;
        let step_h = operator * Vector3::unit_y() / WIDTH as f32;

        for _i in 0..HEIGHT as usize {
            ray_dir += step_v;
            let mut row = "\n".to_string();
            for _j in 0..WIDTH as usize {
                ray_dir += step_h;

                let collision = self.ray_marching(self.position, ray_dir);

                let brightness = match collision {
                    Some(point) => self.light_point(point),
                    None => 0.0
                };

                row.push(self.palette[((1.0 - brightness) * (self.palette.len() - 1) as f32) as usize]);
            }
            ray_dir -= step_h * WIDTH as f32;
            addstr(&row);
        }
    }

    pub fn normal(&self, point: Vector) -> Vector {
        let d = 0.001;

        let dx = Vector::unit_x() * d;
        let dy = Vector::unit_y() * d;
        let dz = Vector::unit_z() * d;

        let sdf = self.sdf(point);

        return (Vector {
            x: (self.sdf(point + dx) - sdf),
            y: (self.sdf(point + dy) - sdf),
            z: (self.sdf(point + dz) - sdf),
        } / d).normalize()
    }

    pub fn ray_marching(&self, origin: Vector, direction: Vector) -> Option<Vector> {
        let threshold = 0.1;

        let ray = direction.normalize();
        let mut point = origin;
        let mut dist = 0.0;
        let mut count = 0;

        while dist < self.brightness && count < 10 {
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
        let mut t = 0.1;
        let k = 4.0;

        while t < 1.0 {
            let h = self.sdf(point - self.light * t);
            if h < 0.001 {
                return 0.00
            }
            res = res.min(k * h / t);
            t += h;
        }

        return res
    }

    pub fn specular_lighting(&self, point: Vector) -> f32 {
        let normal = self.normal(point);
        let dot = -(normal.dot(self.light));
        return dot.min(1.0).max(0.0)
    }
}
