// TODO :
// 1. Optimize the raycasting system by a LOT

use std::fmt;
use std::{
    f32::consts::PI,
    ops::{Add, Div, Mul, Sub},
};

use glam::Vec3A;
use rayon::prelude::*;

use crate::{FOV, LIGHT_INTENSITY, VIEW_DISTANCE};

// Scene specific structs
pub struct Scene {
    objects: Vec<Box<dyn Object>>,
    pub camera: Camera,
    pub light: Light,
}

pub struct Camera {
    pub position: Vec3A,
    pub orientation: EulerAngles,
}

pub struct Light {
    pub position: Vec3A,
    color: Color,
}

// Object structs
pub struct Sphere {
    position: Vec3A,
    radius: f32,
    color: Color,
}

pub struct Cube {
    position: Vec3A,
    length: f32,
    orientation: EulerAngles,
    color: Color,
}

// Raycast specific structs
pub struct Ray {
    origin: Vec3A,
    target: Vec3A,
    direction: Vec3A,
}

pub struct Hit {
    distance: f32,
    point: Vec3A,
    normal: Vec3A,
    surface_color: Color,
}

// Custom data structs
#[derive(Clone, Copy)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

// Angle related structs
#[derive(Clone, Copy)]
pub struct EulerAngles {
    pub roll: Degree,
    pub pitch: Degree,
    pub yaw: Degree,
}

#[derive(Clone, Copy)]
pub struct Degree {
    pub value: f32,
}

// traits & implementations
trait Object: Send + Sync {
    fn intersect(&self, ray: &Ray) -> Option<Hit>;
}

impl Scene {
    pub fn scene_with_sphere() -> Scene {
        let sphere = Sphere {
            position: Vec3A::new(50.0, 0.0, 0.0),
            radius: 10.0,
            color: Color::new(4, 2, 115),
        };

        let boxed: Box<dyn Object + 'static> = Box::new(sphere);

        Scene {
            objects: vec![boxed],
            camera: Camera::new(
                Vec3A::ZERO,
                EulerAngles::new(0.0.into(), 0.0.into(), 0.0.into()),
            ),
            light: Light {
                position: Vec3A::new(-10.0, -10.0, 20.0),
                color: Color::new(255, 255, 255),
            },
        }
    }

    pub fn render(&self, width: usize, height: usize) -> Vec<u32> {
        (0..width * height)
            .into_par_iter()
            .map(|i| self.get_pix_color_val(i, width, height))
            .collect()
    }

    fn get_pix_color_val(&self, pix_id: usize, width: usize, height: usize) -> u32 {
        let cam_angle: EulerAngles = self.camera.orientation;
        let cam_pos: Vec3A = self.camera.position;

        let column = pix_id % width;
        let row = pix_id / width;

        let horizontal_fov = FOV as f32;
        let vertical_fov = horizontal_fov * height as f32 / width as f32;

        let yaw_offset =
            -horizontal_fov / 2.0 + (column as f32 + 0.5) * horizontal_fov / width as f32;

        let pitch_offset = vertical_fov / 2.0 - (row as f32 + 0.5) * vertical_fov / height as f32;

        let yaw: Degree = cam_angle.yaw + yaw_offset.into();
        let pitch: Degree = cam_angle.pitch + pitch_offset.into();

        let rad_pitch: f32 = pitch.value * PI / 180.0;
        let rad_yaw: f32 = yaw.value * PI / 180.0;

        let end_x: f32 = cam_pos.x + VIEW_DISTANCE as f32 * rad_pitch.cos() * rad_yaw.cos();
        let end_y: f32 = cam_pos.y + VIEW_DISTANCE as f32 * rad_pitch.cos() * rad_yaw.sin();
        let end_z: f32 = cam_pos.z + VIEW_DISTANCE as f32 * rad_pitch.sin();

        let target: Vec3A = Vec3A::new(end_x, end_y, end_z);
        let res: Option<Hit> = self.send_ray(cam_pos, target);

        if let Some(val) = res {
            let surface_color = val.surface_color;

            let light_vec = (self.light.position - val.point).normalize();

            let intensity: f32 = (LIGHT_INTENSITY * val.normal.dot(light_vec)).clamp(0.1, 1.0);
            let factor: f32 = intensity / 255.0;

            let color = Color::new(
                (surface_color.r as f32 * self.light.color.r as f32 * factor) as u8,
                (surface_color.g as f32 * self.light.color.g as f32 * factor) as u8,
                (surface_color.b as f32 * self.light.color.b as f32 * factor) as u8,
            );

            self.generate_pixel_buf_value(color)
        } else {
            self.generate_pixel_buf_value(Color::new(0, 0, 0))
        }
    }

    fn send_ray(&self, origin: Vec3A, target: Vec3A) -> Option<Hit> {
        let direction: Vec3A = (target - origin).normalize();

        let ray = Ray {
            origin,
            target,
            direction,
        };

        for object in &self.objects {
            let res = object.intersect(&ray);
            if let Some(val) = res {
                return Some(val);
            }
        }

        None
    }

    fn generate_pixel_buf_value(&self, color: Color) -> u32 {
        color.r as u32 | (color.g as u32) << 8 | (color.b as u32) << 16
    }
}

impl Object for Sphere {
    fn intersect(&self, ray: &Ray) -> Option<Hit> {
        let oc = ray.origin - self.position;
        let d = ray.direction;

        let a = d.dot(d);
        let half_b = oc.dot(d);
        let c = oc.dot(oc) - self.radius * self.radius;

        let discriminant = half_b * half_b - a * c;
        if discriminant < 0.0 {
            return None;
        }

        let sqrt_discriminant = discriminant.sqrt();
        let mut t = (-half_b - sqrt_discriminant) / a;

        // Edge case when camera is inside the sphere
        if t < 0.0 {
            t = (-half_b + sqrt_discriminant) / a;
        }

        if t < 0.0 {
            return None;
        }

        let am = Vec3A::new(
            ray.direction.x * t,
            ray.direction.y * t,
            ray.direction.z * t,
        );

        let point: Vec3A = ray.origin + am;
        let normal: Vec3A = (point - self.position).normalize();

        Some(Hit::new(ray.origin, normal, point, self.color))
    }
}

impl Camera {
    pub fn new(position: Vec3A, orientation: EulerAngles) -> Camera {
        Camera {
            position,
            orientation,
        }
    }
}

impl Hit {
    pub fn new(origin: Vec3A, normal: Vec3A, intersect: Vec3A, surface_color: Color) -> Hit {
        Hit {
            distance: (intersect - origin).length(),
            point: intersect,
            normal,
            surface_color,
        }
    }
}

impl EulerAngles {
    pub fn new(roll: Degree, pitch: Degree, yaw: Degree) -> EulerAngles {
        EulerAngles { roll, pitch, yaw }
    }
}

impl Add for Degree {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Degree {
            value: (self.value + other.value).rem_euclid(360.0),
        }
    }
}

impl Sub for Degree {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Degree {
            value: (self.value - other.value).rem_euclid(360.0),
        }
    }
}

impl Mul for Degree {
    type Output = Self;

    fn mul(self, other: Self) -> Self::Output {
        Degree {
            value: (self.value * other.value).rem_euclid(360.0),
        }
    }
}

impl Div for Degree {
    type Output = Self;

    fn div(self, other: Self) -> Self::Output {
        Degree {
            value: (self.value / other.value).rem_euclid(360.0),
        }
    }
}

impl From<f32> for Degree {
    fn from(num: f32) -> Self {
        Degree { value: num }
    }
}

impl fmt::Display for Degree {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl Color {
    pub fn new(r: u8, g: u8, b: u8) -> Color {
        Color { r, g, b }
    }
}
