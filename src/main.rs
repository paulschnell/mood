extern crate gl;
extern crate glfw;
extern crate image;
extern crate nalgebra_glm;

pub mod graphics;
mod utils;

use glfw::Context;

const TITLE: &str = "mood - C++ mag niemand";

const INIT_WIDTH: u32 = 1280;
const INIT_HEIGHT: u32 = 720;

const CAMERA_SPEED: f32 = 2.5;
const CAMERA_SENSITIVITY: f32 = 7.0;

fn main() {
    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).expect("Failed on initilizing glfw.");

    let (mut window, events) = glfw
        .create_window(INIT_WIDTH, INIT_HEIGHT, TITLE, glfw::WindowMode::Windowed)
        .expect("Failed to create window.");

    window.set_key_polling(true);
    window.set_size_polling(true);
    window.set_cursor_pos_polling(true);
    window.set_scroll_polling(true);

    window.set_cursor_mode(glfw::CursorMode::Disabled);

    window.make_current();

    let wnd_last_size = window.get_size();

    let mut fill_mode = true;
    let mut paused = false;
    #[cfg(debug_assertions)]
    let mut show_fps = true;
    #[cfg(not(debug_assertions))]
    let mut show_fps = false;

    // OpenGL
    gl::load_with(|s| window.get_proc_address(s));

    let mut graphics = graphics::Graphics::init();
    graphics.resize(wnd_last_size.0 as u32, wnd_last_size.1 as u32);

    let mut pre_time = glfw.get_time();
    // Main loop
    while !window.should_close() {
        let current_time = glfw.get_time();

        let delta_time = current_time - pre_time;
        pre_time = current_time;

        for (_, event) in glfw::flush_messages(&events) {
            match event {
                glfw::WindowEvent::Key(glfw::Key::Escape, _, glfw::Action::Press, _) => {
                    // give mouse back
                    if !paused {
                        paused = true;
                        graphics.pause();
                        window.set_cursor_mode(glfw::CursorMode::Normal);
                    } else {
                        paused = false;
                        graphics.unpause();
                        window.set_cursor_mode(glfw::CursorMode::Disabled);
                        graphics.camera.set_first_move(true);
                    }
                }

                glfw::WindowEvent::Size(width, height) => {
                    graphics.resize(width as u32, height as u32);
                }

                glfw::WindowEvent::CursorPos(x, y) => {
                    if !paused {
                        graphics.camera.cur_mov(
                            x as f32,
                            y as f32,
                            CAMERA_SENSITIVITY * delta_time as f32,
                        );
                    }
                }

                glfw::WindowEvent::Key(
                    glfw::Key::F,
                    _,
                    glfw::Action::Press,
                    glfw::Modifiers::Control,
                ) => {
                    fill_mode = if fill_mode {
                        unsafe {
                            gl::PolygonMode(gl::FRONT_AND_BACK, gl::LINE);
                        }
                        false
                    } else {
                        unsafe {
                            gl::PolygonMode(gl::FRONT_AND_BACK, gl::FILL);
                        }
                        true
                    };
                }

                glfw::WindowEvent::Key(
                    glfw::Key::P,
                    _,
                    glfw::Action::Press,
                    glfw::Modifiers::Control,
                ) => {
                    show_fps = if show_fps {
                        window.set_title(TITLE);
                        false
                    } else {
                        true
                    };
                }

                _ => {}
            }
        }

        // Update
        graphics.handle_input(delta_time as f32, &window);
        graphics.update(delta_time as f32);

        if show_fps {
            window.set_title(
                format!(
                    "{} | FPS: {:.5} | ms per Frame: {:.7}",
                    TITLE,
                    1.0 / delta_time,
                    delta_time
                )
                .as_str(),
            );
        }

        window.swap_buffers();
        glfw.poll_events();
    }

    graphics.destroy();
}
