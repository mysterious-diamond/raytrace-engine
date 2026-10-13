use minifb::{Window, WindowOptions};

use crate::engine::Scene;

mod engine;

const WIDTH: usize = 800;
const HEIGHT: usize = 600;

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

    let scene = Scene::scene_with_sphere();

    while window.is_open() {
        window.update();
    }
}
