use cgmath::Vector3;
use cgmath::prelude::*;
use ncurses::addstr;
use std::f32;

use crate::Buffer;
use crate::Camera;

type Vector = Vector3<f32>;

pub struct Renderer {
    pub camera: Camera,
    pub buffer: Buffer,
}

impl Renderer {
    pub fn render(&self, sdf: &dyn Fn(Vector) -> f32) {
        let (mut ray_dir, mut step_h, mut step_v) = self.camera.get_screen_iterator();

        step_v /= self.buffer.height;
        step_h /= self.buffer.width;

        for _i in 0..self.buffer.height as usize {
            ray_dir += step_v;
            let mut row = "\n".to_string();
            for _j in 0..self.buffer.width as usize {
                ray_dir += step_h;

                let collision = Self::ray_march(self.camera.position, ray_dir, sdf);

                let brightness = match collision {
                    Some(point) => Self::light_point(point, sdf),
                    None => 0.0
                };

                row.push(self.buffer.palette[((1.0 - brightness) * (self.buffer.palette.len() - 1) as f32) as usize]);
            }
            ray_dir -= step_h * self.buffer.width;
            addstr(&row);
        }
    }

    pub fn ray_march(origin: Vector, direction: Vector, sdf: &dyn Fn(Vector) -> f32) -> Option<Vector> {
        let threshold = 0.1;

        let ray = direction.normalize();
        let mut point = origin;
        let mut dist = 0.0;
        let mut count = 0;

        while dist < 8.0 && count < 10 {
            count += 1;
            dist = sdf(point);
            if dist.abs() < threshold {
                return Some(point);
            }
            point += ray * dist;
        }

        return None
    }

    pub fn light_point(point: Vector, sdf: &dyn Fn(Vector) -> f32) -> f32 {
        let light = Vector::new(1.0, 1.0, -1.0);
        let ambient = 0.1;
        return ambient + (1.0 - ambient) * (
            Self::diffuse_lighting(point, light, sdf) * 0.7 +
            Self::specular_lighting(point, light, sdf) * 0.3
        )
    }

    pub fn diffuse_lighting(point: Vector, light: Vector, sdf: &dyn Fn(Vector) -> f32) -> f32 {
        let mut res: f32 = 1.0;
        let mut t = 0.1;
        let k = 4.0;

        while t < 1.0 {
            let h = sdf(point - light * t);
            if h < 0.001 {
                return 0.00
            }
            res = res.min(k * h / t);
            t += h;
        }

        return res
    }

    pub fn specular_lighting(point: Vector, light: Vector, sdf: &dyn Fn(Vector) -> f32) -> f32 {
        let normal = Self::normal(point, sdf);
        let dot = -(normal.dot(light));
        return dot.min(1.0).max(0.0)
     }

     pub fn normal(point: Vector, sdf: &dyn Fn(Vector) -> f32) -> Vector {
        let d = 0.001;

        let dx = Vector::unit_x() * d;
        let dy = Vector::unit_y() * d;
        let dz = Vector::unit_z() * d;

        let dist = sdf(point);

        return (Vector {
            x: (sdf(point + dx) - dist),
            y: (sdf(point + dy) - dist),
            z: (sdf(point + dz) - dist),
        } / d).normalize()
    }
}
