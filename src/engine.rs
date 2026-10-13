use std::ops::{Add, Sub};

// Scene specific structs
pub struct Scene {
    objects: Vec<Box<dyn Object>>,
    camera: Camera,
}

pub struct Camera {
    position: Vec3,
    orientation: EulerAngles,
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
}

// Custom data structs
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

pub struct EulerAngles {
    pub roll: f64,
    pub pitch: f64,
    pub yaw: f64,
}

// traits & implementations
trait Object {
    fn intersect(&self, ray: Ray) -> Option<Hit>;
}

impl Scene {
    pub fn scene_with_sphere() -> Scene {
        let sphere = Sphere {
            position: Vec3::new(0.0, 0.0, 30.0),
            radius: 10.0,
            color: Color::new(4, 2, 115),
        };

        let boxed: Box<dyn Object + 'static> = Box::new(sphere);

        Scene {
            objects: vec![boxed],
            camera: Camera::new(Vec3::origin(), EulerAngles::new(0.0, 0.0, 0.0)),
        }
    }
}

impl Object for Sphere {
    fn intersect(&self, ray: Ray) -> Option<Hit> {
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

            Some(Hit::new(ray.origin, normal, point))
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
    pub fn new(origin: Vec3, normal: Vec3, intersect: Vec3) -> Hit {
        Hit {
            distance: (intersect - origin).magnitude() as f32,
            point: intersect,
            normal,
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
