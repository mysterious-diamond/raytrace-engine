// TODO : 
// 1. Implement pitch and yaw shift each loop
// 2. Render result of Scene::render in main.rs

use std::{
    f64::consts::PI,
    ops::{Add, Sub},
};

use crate::{FOV, HEIGHT, VIEW_DISTANCE, WIDTH};

// Scene specific structs
pub struct Scene {
    objects: Vec<Box<dyn Object>>,
    camera: Camera,
    light: Light,
}

pub struct Camera {
    position: Vec3,
    orientation: EulerAngles,
}

pub struct Light {
    position: Vec3,
    color: Color,
}

// Object structs
pub struct Sphere {
    position: Vec3,
    radius: f64,
    color: Color,
}

pub struct Cube {
    position: Vec3,
    length: f64,
    orientation: EulerAngles,
    color: Color,
}

// Raycast specific structs
pub struct Ray {
    origin: Vec3,
    target: Vec3,
    direction: Vec3,
}

pub struct Hit {
    distance: f32,
    point: Vec3,
    normal: Vec3,
    surface_color: Color,
}

// Custom data structs
#[derive(Clone, Copy)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

#[derive(Clone, Copy)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

#[derive(Clone, Copy)]
pub struct EulerAngles {
    pub roll: f64,
    pub pitch: f64,
    pub yaw: f64,
}

// traits & implementations
trait Object {
    fn intersect(&self, ray: &Ray) -> Option<Hit>;
}

impl Scene {
    pub fn scene_with_sphere() -> Scene {
        let sphere = Sphere {
            position: Vec3::new(30.0, 0.0, 0.0),
            radius: 10.0,
            color: Color::new(4, 2, 115),
        };

        let boxed: Box<dyn Object + 'static> = Box::new(sphere);

        Scene {
            objects: vec![boxed],
            camera: Camera::new(Vec3::origin(), EulerAngles::new(0.0, 0.0, 0.0)),
            light: Light {
                position: Vec3::new(-10.0, -10.0, 20.0),
                color: Color::new(255, 255, 255),
            },
        }
    }

    pub fn render(&self) -> Vec<u32> {
        let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];
        let mut angle: EulerAngles = self.camera.orientation;
        let cam_pos = self.camera.position;

        let yaw = angle.yaw - FOV as f64 / 2.0;
        let pitch = angle.pitch + FOV as f64 / 2.0 * HEIGHT as f64 / WIDTH as f64;

        angle.yaw = yaw;
        angle.pitch = pitch;

        for i in 0..buffer.len() {
            let rad_z = cam_pos.z * PI / 180.0;
            let rad_y = cam_pos.y * PI / 180.0;

            let end_x = cam_pos.x + VIEW_DISTANCE as f64 * rad_z.cos() * rad_y.cos();
            let end_y = cam_pos.y + VIEW_DISTANCE as f64 * rad_z.sin() * rad_y.cos();
            let end_z = cam_pos.z + VIEW_DISTANCE as f64 * rad_z.sin();

            let target = Vec3::new(end_x, end_y, end_z);
            let res = self.send_ray(cam_pos, target);

            if let Some(val) = res {
                let mut color = Color::new(
                    val.surface_color.r * self.light.color.r,
                    val.surface_color.g * self.light.color.g,
                    val.surface_color.b * self.light.color.b,
                );

                let mut light_vec = Vec3::new(
                    val.point.x - self.light.position.x,
                    val.point.y - self.light.position.y,
                    val.point.z - self.light.position.z,
                );

                light_vec.normalize();
                let intensity = val.normal.dot(light_vec).max(0.0);

                color.r = ((color.r as f64 / 255.0 * intensity) * 255.0) as u8;
                color.g = ((color.g as f64 / 255.0 * intensity) * 255.0) as u8;
                color.b = ((color.b as f64 / 255.0 * intensity) * 255.0) as u8;

                buffer[i] = self.generate_pixel_buf_value(color);

                if (i + 1) % WIDTH == 0 {
                    angle.yaw = yaw;
                    pitch -= 
                }
            } else {
                buffer[i] = self.generate_pixel_buf_value(Color::new(0, 0, 0));
            }
        }

        buffer
    }

    fn send_ray(&self, origin: Vec3, target: Vec3) -> Option<Hit> {
        let ray = Ray {
            origin,
            target,
            direction: target - origin,
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
        // The vector from the origin of the ray to the center of the circle
        let l: Vec3 = ray.origin - self.position;
        let ca: f64 = l.dot(ray.direction);

        // The distance squared from the center to the closest point on ray
        let dist: f64 = l.dot(l) - ca * ca;
        if dist > self.radius * self.radius {
            return None;
        }

        // Half chord distance of ray in sphere
        let hc: f64 = f64::sqrt(self.radius * self.radius - dist);

        // Supposed first intersect point
        let t1: f64 = ca - hc;
        if t1 < 0.0 || t1 > (ray.target - ray.origin).magnitude() {
            None
        } else {
            let am = Vec3 {
                x: ray.direction.x * t1,
                y: ray.direction.y * t1,
                z: ray.direction.z * t1,
            };
            let point: Vec3 = ray.origin + am;
            let mut normal: Vec3 = point - self.position;
            normal.normalize();

            Some(Hit::new(ray.origin, normal, point, self.color))
        }
    }
}

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Vec3 {
        Vec3 { x, y, z }
    }

    pub fn origin() -> Vec3 {
        Vec3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }

    pub fn dot(&self, other: Vec3) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn normalize(&mut self) {
        let magnitude = self.magnitude();

        self.x /= magnitude;
        self.y /= magnitude;
        self.z /= magnitude;
    }

    pub fn magnitude(&self) -> f64 {
        f64::sqrt(self.x * self.x + self.y * self.y + self.z * self.z)
    }
}

impl Sub for Vec3 {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Vec3 {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl Add for Vec3 {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Vec3 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z - other.z,
        }
    }
}

impl Camera {
    pub fn new(position: Vec3, orientation: EulerAngles) -> Camera {
        Camera {
            position,
            orientation,
        }
    }
}

impl Hit {
    pub fn new(origin: Vec3, normal: Vec3, intersect: Vec3, surface_color: Color) -> Hit {
        Hit {
            distance: (intersect - origin).magnitude() as f32,
            point: intersect,
            normal,
            surface_color,
        }
    }
}

impl EulerAngles {
    pub fn new(roll: f64, pitch: f64, yaw: f64) -> EulerAngles {
        EulerAngles { roll, pitch, yaw }
    }
}

impl Color {
    pub fn new(r: u8, g: u8, b: u8) -> Color {
        Color { r, g, b }
    }
}
