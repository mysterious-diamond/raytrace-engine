// TODO :
// 1. Optimize the raycasting system by a LOT

use glam::{Quat, Vec3A};
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
    pub orientation: Quat,
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
    orientation: Quat,
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

// traits & implementations
trait Object: Send + Sync {
    fn intersect(&self, ray: &Ray) -> Option<Hit>;
}

impl Scene {
    pub fn scene_with_sphere() -> Scene {
        let sphere = Sphere {
            position: Vec3A::new(50.0, 0.0, 0.0),
            radius: 20.0,
            color: Color::new(4, 2, 115),
        };

        let boxed: Box<dyn Object + 'static> = Box::new(sphere);

        Scene {
            objects: vec![boxed],
            camera: Camera::new(Vec3A::ZERO, Quat::IDENTITY),
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
        let cam_angle: Quat = self.camera.orientation;
        let cam_pos: Vec3A = self.camera.position;

        let column: usize = pix_id % width;
        let row: usize = pix_id / width;

        let ndc_x: f32 = (2.0 * column as f32 + 1.0) / width as f32 - 1.0;
        let ndc_y: f32 = 1.0 - (2.0 * row as f32 + 1.0) / height as f32;

        let aspect_ratio: f32 = width as f32 / height as f32;
        let half_fov_tan: f32 = (FOV as f32 / 2.0).to_radians().tan();

        let screen_x: f32 = ndc_x * half_fov_tan * aspect_ratio;
        let screen_y: f32 = ndc_y * half_fov_tan;

        let forward: Vec3A = cam_angle * Vec3A::X;
        let right: Vec3A = cam_angle * Vec3A::Z;
        let up: Vec3A = cam_angle * Vec3A::Y;

        let ray_dir: Vec3A = (forward + (right * screen_x) + (up * screen_y)).normalize();

        let target: Vec3A = cam_pos + (ray_dir * VIEW_DISTANCE as f32);
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
            0
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
        (color.r as u32) << 16 | (color.g as u32) << 8 | color.b as u32
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
    pub fn new(position: Vec3A, orientation: Quat) -> Camera {
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

impl Color {
    pub fn new(r: u8, g: u8, b: u8) -> Color {
        Color { r, g, b }
    }
}
