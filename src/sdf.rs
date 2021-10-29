use std::f32::consts::PI;
use cgmath::Vector3;
use cgmath::prelude::*;

type Vector = Vector3<f32>;

pub fn sd_sphere(point: Vector, center: Vector, radius: f32) -> f32 {
    (point - center).magnitude() - radius
}

pub fn sd_box(point: Vector, center: Vector, size: Vector) -> f32 {
    let diff = center - point;
    let q = diff.map(|n| n.abs()) - size / 2.0;
    return q.map(|n| n.max(0.0)).magnitude() + (q.y.max(q.z).max(q.x)).min(0.0)
}

pub fn sd_gear(point: Vector, time: f32, center: Vector, radius: f32, thickness: f32, turn_rate: f32) -> f32 {
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
        let angle = sector_angle * time / turn_rate;
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
