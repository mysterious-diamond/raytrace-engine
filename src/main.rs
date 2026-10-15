use minifb::{Key, Window, WindowOptions};

use crate::engine::Scene;

mod engine;

const START_WIDTH: usize = 800;
const START_HEIGHT: usize = 600;
const FOV: usize = 100;
const VIEW_DISTANCE: usize = 100;
const WALK_DISTANCE: f32 = 1.0;
const LIGHT_INTENSITY: f32 = 2.0;

pub struct App {
    pub window: Window,
    pub scene: Scene,
}

pub fn main() {
    let mut window = match Window::new(
        "raytrace",
        START_WIDTH,
        START_HEIGHT,
        WindowOptions {
            resize: true,
            ..WindowOptions::default()
        },
    ) {
        Ok(win) => win,
        Err(err) => panic!("app.window was not able to be created, {}", err),
    };

    window.set_target_fps(120);
    let scene: Scene = Scene::scene_with_sphere();

    let mut app: App = App { window, scene };

    while app.window.is_open() {
        let (width, height) = app.window.get_size();
        let buffer: Vec<u32> = app.scene.render(width, height);

        match app.window.update_with_buffer(&buffer, width, height) {
            Ok(_) => (),
            Err(err) => panic!("Error in updating app.window buffer, {}", err),
        }

        if app.window.is_key_down(Key::W) || app.window.is_key_down(Key::Up) {
            app.scene.camera.position.x += WALK_DISTANCE;
        }
        if app.window.is_key_down(Key::S) || app.window.is_key_down(Key::Down) {
            app.scene.camera.position.x -= WALK_DISTANCE;
        }
        if app.window.is_key_down(Key::D) || app.window.is_key_down(Key::Right) {
            app.scene.camera.position.y += WALK_DISTANCE;
        }
        if app.window.is_key_down(Key::A) || app.window.is_key_down(Key::Left) {
            app.scene.camera.position.y -= WALK_DISTANCE;
        }
    }
}
