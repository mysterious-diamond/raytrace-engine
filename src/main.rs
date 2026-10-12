use minifb::{Window, WindowOptions};

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

    while window.is_open() {
        window.update();
    }
}
