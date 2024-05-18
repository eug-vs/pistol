use cgmath::prelude::*;
use cgmath::Vector3;
use ncurses::addstr;
use rayon::prelude::*;
use std::f32;
use std::sync::Arc;

use crate::Buffer;
use crate::Camera;
use crate::Object;

type Vector = Vector3<f32>;

pub struct Renderer {
    pub camera: Camera,
    pub buffer: Buffer,
    pub sdf: Arc<dyn Fn(Vector3<f32>, f32) -> f32 + Send + Sync>,
}

impl Renderer {
    pub fn new(buffer: Buffer, camera: Camera, objects: Vec<Box<dyn Object>>) -> Self {
        let sdf = move |point: Vector3<f32>, time: f32| -> f32 {
            let mut dist = f32::MAX;
            for object in objects.iter() {
                dist = dist.min(object.sdf(point, time));
            }
            dist
        };

        Self {
            buffer,
            camera,
            sdf: Arc::new(sdf),
        }
    }

    pub fn render(&self, time: f32) {
        let mut iterator = self.camera.get_screen_iterator();
        iterator.set_buffer_size(&self.buffer);

        let sdf = |point: Vector3<f32>| -> f32 { (self.sdf)(point, time) };

        let ray_dirs: Vec<Vector3<f32>> = iterator.collect();

        // Ray march in parallel
        let chars: Vec<char> = ray_dirs
            .par_iter()
            .map(|ray_dir| {
                let collision = Self::ray_march(self.camera.position, *ray_dir, &sdf);
                match collision {
                    Some(point) => Self::light_point(point, &sdf),
                    None => 0.0,
                }
            })
            .map(|brightness| {
                self.buffer.palette
                    [((1.0 - brightness) * (self.buffer.palette.len() - 1) as f32) as usize]
            })
            .collect();

        for _i in 0..self.buffer.height as usize {
            let mut row = "\n".to_string();
            for _j in 0..self.buffer.width as usize {
                let character = chars[_i * self.buffer.width as usize + _j];
                row.push(character);
            }
            addstr(&row);
        }
    }

    pub fn ray_march(
        origin: Vector,
        direction: Vector,
        sdf: &dyn Fn(Vector) -> f32,
    ) -> Option<Vector> {
        let threshold = 0.01;

        let ray = direction.normalize();
        let mut point = origin;
        let mut dist = 0.0;
        let mut count = 0;

        while dist < 8.0 && count < 30 {
            count += 1;
            dist = sdf(point);
            if dist.abs() < threshold {
                return Some(point);
            }
            point += ray * dist;
        }

        None
    }

    pub fn light_point(point: Vector, sdf: &dyn Fn(Vector) -> f32) -> f32 {
        let light = Vector::new(1.0, 1.0, -1.0);
        let ambient = 0.1;
        ambient
            + (1.0 - ambient)
                * (Self::diffuse_lighting(point, light, sdf) * 0.7
                    + Self::specular_lighting(point, light, sdf) * 0.3)
    }

    pub fn diffuse_lighting(point: Vector, light: Vector, sdf: &dyn Fn(Vector) -> f32) -> f32 {
        let mut res: f32 = 1.0;
        let mut t = 0.1;
        let k = 4.0;

        while t < 1.0 {
            let h = sdf(point - light * t);
            if h < 0.001 {
                return 0.00;
            }
            res = res.min(k * h / t);
            t += h;
        }

        res
    }

    pub fn specular_lighting(point: Vector, light: Vector, sdf: &dyn Fn(Vector) -> f32) -> f32 {
        let normal = Self::normal(point, sdf);
        let dot = -(normal.dot(light));
        dot.min(1.0).max(0.0)
    }

    pub fn normal(point: Vector, sdf: &dyn Fn(Vector) -> f32) -> Vector {
        let d = 0.001;

        let dx = Vector::unit_x() * d;
        let dy = Vector::unit_y() * d;
        let dz = Vector::unit_z() * d;

        let dist = sdf(point);

        (Vector {
            x: (sdf(point + dx) - dist),
            y: (sdf(point + dy) - dist),
            z: (sdf(point + dz) - dist),
        } / d)
            .normalize()
    }
}
