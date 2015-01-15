use time::precise_time_ns;
use state::State;

use gl::types::*;
use glfw;
use glfw::{Action, Context, Key, Window, Glfw};
use std::sync::mpsc::Receiver;

pub struct Renderer {
    glfw: Glfw,
    window: Window,
    state: State,
    events: Receiver<(f64, glfw::WindowEvent)>,
    delta_time: u64,
    last_time: u64,
}

impl Renderer {
    pub fn new(glfw: Glfw, window: Window, events: Receiver<(f64, glfw::WindowEvent)>, state: State) -> Renderer {
        Renderer {
            glfw: glfw,
            window: window,
            events: events,
            state: state,
            delta_time: 0,
            last_time: precise_time_ns(),
        }
    }

    pub fn running(&self) -> bool {
        !self.window.should_close()
    }

    fn handle_event(&self, window: &glfw::Window, event: glfw::WindowEvent) {
        match event {
            glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
                window.set_should_close(true)
            }
            _ => {}
        }
    }

    pub fn tick(&mut self) {
         let events = self.get_events();
         for event in events.iter() {
             self.handle_event(&self.window, *event);
         }

         let time = precise_time_ns();
         self.delta_time = time - self.last_time;
         self.last_time = time;
         println!("tick: {}", self.delta_time);
    }

    fn get_events(&self) -> Vec<glfw::WindowEvent> {
        self.glfw.poll_events();
        let mut events = Vec::new();
        for (_, event) in glfw::flush_messages(&self.events) {
            events.push(event);
        }
        events
    }
}
