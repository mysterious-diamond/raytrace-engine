use minifb::{Key, Window, WindowOptions};

use crate::engine::Scene;

mod engine;

const WIDTH: usize = 800;
const HEIGHT: usize = 600;
const FOV: usize = 100;
const VIEW_DISTANCE: usize = 100;
const WALK_DISTANCE: f64 = 1.0;
const LIGHT_INTENSITY: f64 = 2.0;

pub fn main() {
    let mut window = match Window::new(
        "raytrace",
        WIDTH,
        HEIGHT,
        WindowOptions {
            resize: false,
            ..WindowOptions::default()
        },
    ) {
        Ok(win) => win,
        Err(err) => panic!("Window was not able to be created, {}", err),
    };

    window.set_target_fps(120);
    let mut scene: Scene = Scene::scene_with_sphere();

    while window.is_open() {
        let buffer: Vec<u32> = scene.render();

        match window.update_with_buffer(&buffer, WIDTH, HEIGHT) {
            Ok(_) => (),
            Err(err) => panic!("Error in updating window buffer, {}", err),
        }

        if window.is_key_down(Key::W) || window.is_key_down(Key::Up) {
            scene.camera.position.x += WALK_DISTANCE;
        }
        if window.is_key_down(Key::S) || window.is_key_down(Key::Down) {
            scene.camera.position.x -= WALK_DISTANCE;
        }
        if window.is_key_down(Key::D) || window.is_key_down(Key::Right) {
            scene.camera.position.y += WALK_DISTANCE;
        }
        if window.is_key_down(Key::A) || window.is_key_down(Key::Left) {
            scene.camera.position.y -= WALK_DISTANCE;
        }
    }
}
