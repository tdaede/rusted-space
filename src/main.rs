extern crate time;
extern crate gl;
extern crate glfw;

// include the OpenGL type aliases
use gl::types::*;
use glfw::{Action, Context, Key};
use state::State;
use renderer::Renderer;

mod renderer;
mod state;

fn main() {
    let glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();
    let height = 800;
    let width = 600;

    let (window, events) = glfw.create_window(height, width, "Rusted Space", glfw::WindowMode::Windowed)
        .expect("Failed to create GLFW window.");

    window.set_key_polling(true);
    window.make_current();

    let mut state = State::new();
    let mut renderer = Renderer::new(glfw, window, events, state);

    while renderer.running() {
        renderer.tick()
    }

    println!("Rusted Space ended not with a bang, but with whimper");
}
