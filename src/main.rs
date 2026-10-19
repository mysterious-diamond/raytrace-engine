use std::f32::consts::TAU;

use glam::Vec3A;
use minifb::{Key, MouseButton, MouseMode, Window, WindowOptions};

use crate::engine::Scene;

mod engine;

const START_WIDTH: usize = 800;
const START_HEIGHT: usize = 600;
const FOV: usize = 90;
const VIEW_DISTANCE: usize = 100;
const LIGHT_INTENSITY: f32 = 2.0;
const WALK_DISTANCE: f32 = 1.1;
const MOUSE_SENS: f32 = 0.005;

pub struct App {
    pub window: Window,
    pub scene: Scene,
}

pub fn main() {
    let window = match Window::new(
        "raytrace",
        START_WIDTH,
        START_HEIGHT,
        WindowOptions {
            resize: true,
            borderless: true,
            ..WindowOptions::default()
        },
    ) {
        Ok(win) => win,
        Err(err) => panic!("app.window was not able to be created, {}", err),
    };

    let scene: Scene = Scene::scene_with_2_spheres();

    let mut app: App = App { window, scene };
    let mut is_tracking_mouse: bool = false;
    let mut last_mouse_pos: (f32, f32) = (0.0, 0.0);
    let mut yaw: f32 = 0.0;
    let mut pitch: f32 = 0.0;

    while app.window.is_open() {
        let (width, height) = app.window.get_size();
        let buffer: Vec<u32> = app.scene.render(width, height);

        match app.window.update_with_buffer(&buffer, width, height) {
            Ok(_) => (),
            Err(err) => panic!("Error in updating app.window buffer, {}", err),
        }

        let cam_quat = app.scene.camera.orientation;
        let raw_forward = cam_quat * Vec3A::X;
        let raw_right = cam_quat * Vec3A::Z;

        let mut move_forward = Vec3A::new(raw_forward.x, 0.0, raw_forward.z);
        if move_forward.length_squared() > 0.0 {
            move_forward = move_forward.normalize();
        }

        let mut move_right = Vec3A::new(raw_right.x, 0.0, raw_right.z);
        if move_right.length_squared() > 0.0 {
            move_right = move_right.normalize();
        }

        let mut input_direction = Vec3A::ZERO;

        if app.window.is_key_down(Key::W) || app.window.is_key_down(Key::Up) {
            input_direction += move_forward;
        }
        if app.window.is_key_down(Key::S) || app.window.is_key_down(Key::Down) {
            input_direction -= move_forward;
        }
        if app.window.is_key_down(Key::D) || app.window.is_key_down(Key::Right) {
            input_direction += move_right;
        }
        if app.window.is_key_down(Key::A) || app.window.is_key_down(Key::Left) {
            input_direction -= move_right;
        }

        if input_direction.length_squared() > 0.0 {
            app.scene.camera.position += input_direction.normalize() * WALK_DISTANCE;
        }

        if app.window.get_mouse_down(MouseButton::Right) {
            let pos: Option<(f32, f32)> = app.window.get_unscaled_mouse_pos(MouseMode::Clamp);
            if let Some(val) = pos {
                if !is_tracking_mouse {
                    last_mouse_pos = val;
                    is_tracking_mouse = true;
                } else {
                    let diff_x: f32 = val.0 - last_mouse_pos.0;
                    let diff_y: f32 = val.1 - last_mouse_pos.1;

                    yaw -= diff_x * MOUSE_SENS;
                    pitch -= diff_y * MOUSE_SENS;

                    let max_pitch: f32 = 89.0f32.to_radians();
                    pitch = pitch.clamp(-max_pitch, max_pitch);

                    yaw = yaw.rem_euclid(TAU);

                    app.scene.camera.orientation =
                        glam::Quat::from_euler(glam::EulerRot::YZX, yaw, pitch, 0.0);

                    last_mouse_pos = val;
                }
            }
        } else if is_tracking_mouse {
            is_tracking_mouse = false
        }
    }
}
