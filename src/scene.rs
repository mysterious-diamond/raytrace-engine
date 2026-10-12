// Scene specific structs
pub struct Scene {
    objects: Vec<Box<dyn Object>>,
    camera: Camera,
}

pub struct Camera {
    position: Vec3<f64>,
}

// Object structs
pub struct Sphere {
    position: Vec3<f64>,
    radius: f64,
    orientation: EulerAngles,
    color: Color,
}

pub struct Cube {
    position: Vec3<f64>,
    length: f64,
    orientation: EulerAngles,
    color: Color,
}

// Raycast specific structs
pub struct Ray {
    origin: Vec3<f64>,
    target: Vec3<f64>,
}

pub struct Hit {
    distance: f32,
    point: Vec3<f64>,
    normal: Vec3<f64>,
    color: Color,
}

// Custom data structs
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

pub struct Vec3<T> {
    pub x: T,
    pub y: T,
    pub z: T,
}

pub struct EulerAngles {
    pub roll: f64,
    pub pitch: f64,
    pub yaw: f64,
}

// Object traits & implementations
trait Object {
    fn intersect(&self, ray: Ray) -> Option<Hit>;
}

impl Object for Sphere {
    fn intersect(&self, ray: Ray) -> Option<Hit> {
        None
    }
}
