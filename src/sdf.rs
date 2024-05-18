use cgmath::prelude::*;
use cgmath::Vector3;
use std::f32::consts::PI;

type Vector = Vector3<f32>;

pub trait Object: Send + Sync {
    fn sdf(&self, point: Vector, time: f32) -> f32;
}

pub struct Sphere {
    pub center: Vector,
    pub radius: f32,
}

impl Object for Sphere {
    fn sdf(&self, point: Vector, _time: f32) -> f32 {
        (point - self.center).magnitude() - self.radius
    }
}

pub struct SDBox {
    pub center: Vector,
    pub size: Vector,
}

impl Object for SDBox {
    fn sdf(&self, point: Vector, _time: f32) -> f32 {
        let diff = self.center - point;
        let q = diff.map(|n| n.abs()) - self.size / 2.0;
        q.map(|n| n.max(0.0)).magnitude() + (q.y.max(q.z).max(q.x)).min(0.0)
    }
}

pub struct Gear {
    pub center: Vector,
    pub radius: f32,
    pub thickness: f32,
    pub turn_rate: f32,
}

impl Object for Gear {
    fn sdf(&self, point: Vector, time: f32) -> f32 {
        let mut dist: f32;

        let thickness_over_2 = self.thickness / 2.0;
        let thickness_over_4 = self.thickness / 4.0;

        // Ring
        {
            let cylinder_dist = (Vector::new(0.0, point.y, point.z) - self.center).magnitude()
                - (self.radius - thickness_over_4);
            dist = cylinder_dist.abs() - thickness_over_2; // Make cylinder hollow
        }
        // Teeth
        {
            let sector_angle: f32 = 2.0 * PI / 12.0;

            // Account for rotation with time
            let angle = sector_angle * time / self.turn_rate;
            let rotated_point = Vector::new(
                point.x,
                point.y * angle.cos() - point.z * angle.sin(),
                point.y * angle.sin() + point.z * angle.cos(),
            );

            // Map all space to the first sector
            let point_angle = (rotated_point.z / rotated_point.y).atan();
            let angle2 = -sector_angle * (point_angle / sector_angle).round();

            let mapped_point = Vector::new(
                rotated_point.x,
                (rotated_point.y * angle2.cos() - rotated_point.z * angle2.sin()).abs(),
                rotated_point.y * angle2.sin() + rotated_point.z * angle2.cos(),
            );

            // Make teeth smooth by subtracting some amount
            dist = dist.min(
                SDBox {
                    center: Vector {
                        x: 0.0,
                        y: self.radius + thickness_over_2,
                        z: 0.0,
                    },
                    size: Vector::new(self.thickness, self.thickness * 2.0, self.thickness),
                }
                .sdf(mapped_point, time)
                    - thickness_over_4,
            );
        }

        // Take a slice
        dist = dist.max(point.x.abs() - thickness_over_2);

        dist
    }
}
