extern crate glfw;

use glfw::{Action, Context, Key};

mod global_constants;

use crate::global_constants::VERSION_STRING;

fn handle_window_event(window: &mut glfw::Window, event : glfw::WindowEvent) {
    match event {
        glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
            window.set_should_close(true)
        }
        _ => {}
    }
}


fn main() {
    println!("Initializing rust_sorting_algorithm_visualiser {}", VERSION_STRING);

    let mut glfw = glfw::init(glfw::fail_on_errors).unwrap();

    let(mut window, events) = glfw.create_window(300, 300, 
        "rust_sorting_algorithm_visualiser", 
        glfw::WindowMode::Windowed).expect("Failed to create GLFW window.");

    window.set_key_polling(true);
    window.make_current();

    while !window.should_close() {
        glfw.poll_events();
        for(_, event) in glfw::flush_messages(&events) {
            handle_window_event(&mut window, event);
        }
        window.swap_buffers();
    }
}
