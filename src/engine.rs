// TODO :
// 1. Implement pitch and yaw shift each loop
// 2. Render result of Scene::render in main.rs

use std::fmt;
use std::{
    f64::consts::PI,
    ops::{Add, Div, Mul, Sub},
};

use crate::{FOV, HEIGHT, VIEW_DISTANCE, WIDTH};

// Scene specific structs
pub struct Scene {
    objects: Vec<Box<dyn Object>>,
    pub camera: Camera,
    pub light: Light,
}

pub struct Camera {
    position: Vec3,
    orientation: EulerAngles,
}

pub struct Light {
    pub position: Vec3,
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

// Angle related structs
#[derive(Clone, Copy)]
pub struct EulerAngles {
    pub roll: Degree,
    pub pitch: Degree,
    pub yaw: Degree,
}

#[derive(Clone, Copy)]
pub struct Degree {
    pub value: f64,
}

// traits & implementations
trait Object {
    fn intersect(&self, ray: &Ray) -> Option<Hit>;
}

impl Scene {
    pub fn scene_with_sphere() -> Scene {
        let sphere = Sphere {
            position: Vec3::new(50.0, 0.0, 0.0),
            radius: 10.0,
            color: Color::new(4, 2, 115),
        };

        let boxed: Box<dyn Object + 'static> = Box::new(sphere);

        Scene {
            objects: vec![boxed],
            camera: Camera::new(
                Vec3::origin(),
                EulerAngles::new(0.0.into(), 0.0.into(), 0.0.into()),
            ),
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
        let black_pix_val = self.generate_pixel_buf_value(Color::new(0, 0, 0));

        let total_up_deg: Degree = (FOV as f64 / 2.0 * HEIGHT as f64 / WIDTH as f64).into();
        let yaw: Degree = angle.yaw - (FOV as f64 / 2.0).into();
        let pitch: Degree = angle.pitch + total_up_deg;

        let pitch_increments: Degree = (2.0 * total_up_deg.value / (HEIGHT - 1) as f64).into();
        let yaw_increments: Degree = (FOV as f64 / (WIDTH - 1) as f64).into();

        angle.pitch = pitch;
        angle.yaw = yaw;

        for (i, pix) in buffer.iter_mut().enumerate() {
            let rad_pitch = angle.pitch.value * PI / 180.0;
            let rad_yaw = angle.yaw.value * PI / 180.0;

            let end_x = cam_pos.x + VIEW_DISTANCE as f64 * rad_pitch.cos() * rad_yaw.cos();
            let end_y = cam_pos.y + VIEW_DISTANCE as f64 * rad_pitch.cos() * rad_yaw.sin();
            let end_z = cam_pos.z + VIEW_DISTANCE as f64 * rad_pitch.sin();

            let target = Vec3::new(end_x, end_y, end_z);
            let res = self.send_ray(cam_pos, target);

            if let Some(val) = res {
                let surface_color = val.surface_color;

                let mut color = Color::new(
                    ((surface_color.r as f64 / 255.0) * (self.light.color.r as f64 / 255.0) * 255.0)
                        as u8,
                    ((surface_color.g as f64 / 255.0) * (self.light.color.g as f64 / 255.0) * 255.0)
                        as u8,
                    ((surface_color.b as f64 / 255.0) * (self.light.color.b as f64 / 255.0) * 255.0)
                        as u8,
                );

                let mut light_vec: Vec3 = self.light.position - val.point;
                light_vec.normalize();

                let intensity = val.normal.dot(light_vec).max(0.0);

                color.r = ((color.r as f64 / 255.0 * intensity) * 255.0) as u8;
                color.g = ((color.g as f64 / 255.0 * intensity) * 255.0) as u8;
                color.b = ((color.b as f64 / 255.0 * intensity) * 255.0) as u8;

                *pix = self.generate_pixel_buf_value(color);
            } else {
                *pix = black_pix_val;
            }

            if (i + 1) % WIDTH == 0 {
                angle.yaw = yaw;
                angle.pitch = angle.pitch - pitch_increments;
            } else {
                angle.yaw = angle.yaw + yaw_increments;
            }
        }

        buffer
    }

    fn send_ray(&self, origin: Vec3, target: Vec3) -> Option<Hit> {
        let mut direction: Vec3 = target - origin;
        direction.normalize();

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
        let t1: f64 = -ca - hc;
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

impl From<f64> for Degree {
    fn from(num: f64) -> Self {
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
