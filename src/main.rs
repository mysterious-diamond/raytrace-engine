use minifb::{Window, WindowOptions};

use crate::engine::Scene;

mod engine;

const WIDTH: usize = 800;
const HEIGHT: usize = 600;
const FOV: usize = 90;
const VIEW_DISTANCE: usize = 100;

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
    let mut angle: f64 = 0.0;
    let light_spin_radius: f64 = 10.0;

    while window.is_open() {
        let buffer: Vec<u32> = scene.render();

        match window.update_with_buffer(&buffer, WIDTH, HEIGHT) {
            Ok(_) => (),
            Err(err) => panic!("Error in updating window buffer, {}", err),
        }

        scene.light.position.y = -10.0 * light_spin_radius * angle.cos();
        scene.light.position.z = -10.0 * light_spin_radius * angle.sin();
        angle += 0.12;
    }
}
