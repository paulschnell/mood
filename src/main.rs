extern crate gl;
extern crate glfw;
extern crate nalgebra_glm;

mod graphics;
mod utils;

use glfw::Context;
use nalgebra_glm as ng;

const INIT_WIDTH: u32 = 1280;
const INIT_HEIGHT: u32 = 720;

const CAMERA_SPEED: f32 = 4.0;
const CAMERA_SENSITIVITY: f32 = 8.0;

type Vertex = [f32; 6];
const VERTICES: [Vertex; 8] = [
    [1.0, 1.0, 1.0, 1.0, 0.0, 0.0],
    [1.0, -1.0, 1.0, 0.0, 1.0, 0.0],
    [-1.0, -1.0, 1.0, 0.0, 0.0, 1.0],
    [-1.0, 1.0, 1.0, 1.0, 1.0, 0.0],
    [1.0, 1.0, -1.0, 1.0, 0.0, 1.0],
    [1.0, -1.0, -1.0, 0.0, 1.0, 1.0],
    [-1.0, -1.0, -1.0, 1.0, 1.0, 1.0],
    [-1.0, 1.0, -1.0, 0.0, 0.0, 0.0],
];

const INDICES: [u32; 36] = [
    0, 1, 2, 2, 3, 0, 4, 5, 1, 1, 0, 4, 7, 6, 5, 5, 4, 7, 3, 2, 6, 6, 7, 3, 4, 0, 3, 3, 7, 4, 1, 5,
    6, 6, 2, 1,
];

fn main() {
    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).expect("Failed on initilizing glfw.");

    let (mut window, events) = glfw
        .create_window(
            INIT_WIDTH,
            INIT_HEIGHT,
            "mood - C++ mag niemand",
            glfw::WindowMode::Windowed,
        )
        .expect("Failed to create window.");

    window.set_key_polling(true);
    window.set_size_polling(true);
    window.set_cursor_pos_polling(true);
    window.set_scroll_polling(true);

    window.set_cursor_mode(glfw::CursorMode::Disabled);

    window.make_current();

    let wnd_last_size = window.get_size();

    // OpenGL
    gl::load_with(|s| window.get_proc_address(s));

    let mut graphics = graphics::Graphics::init();
    graphics.resize(wnd_last_size.0 as u32, wnd_last_size.1 as u32);

    let mut vao = 0;
    let mut vbo = 0;
    let mut ebo = 0;
    unsafe {
        gl::GenVertexArrays(1, &mut vao);
        gl::GenBuffers(1, &mut vbo);
        gl::GenBuffers(1, &mut ebo);

        gl::BindVertexArray(vao);

        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        gl::BufferData(
            gl::ARRAY_BUFFER,
            std::mem::size_of_val(&VERTICES) as isize,
            VERTICES.as_ptr().cast(),
            gl::STATIC_DRAW,
        );

        gl::VertexAttribPointer(
            0,
            3,
            gl::FLOAT,
            gl::FALSE,
            std::mem::size_of::<Vertex>().try_into().unwrap(),
            0 as *const _,
        );
        gl::EnableVertexAttribArray(0);

        gl::VertexAttribPointer(
            1,
            3,
            gl::FLOAT,
            gl::FALSE,
            std::mem::size_of::<Vertex>().try_into().unwrap(),
            (std::mem::size_of::<f32>() * 3) as *const _,
        );
        gl::EnableVertexAttribArray(1);

        gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ebo);
        gl::BufferData(
            gl::ELEMENT_ARRAY_BUFFER,
            std::mem::size_of_val(&INDICES) as isize,
            INDICES.as_ptr().cast(),
            gl::STATIC_DRAW,
        );

        gl::BindBuffer(gl::ARRAY_BUFFER, 0); // unbind
    }

    // Matrices
    let mut model = ng::Mat4::identity();
    model = ng::scale(&model, &ng::Vec3::new(0.5, 0.5, 0.5));

    let mut pre_time = glfw.get_time();
    // Main loop
    while !window.should_close() {
        let current_time = glfw.get_time();

        let delta_time = current_time - pre_time;
        pre_time = current_time;

        for (_, event) in glfw::flush_messages(&events) {
            match event {
                glfw::WindowEvent::Key(glfw::Key::Escape, _, glfw::Action::Press, _) => {
                    window.set_should_close(true);
                }

                glfw::WindowEvent::Size(width, height) => {
                    graphics.resize(width as u32, height as u32);
                }

                glfw::WindowEvent::CursorPos(x, y) => {
                    graphics.camera.cur_mov(
                        x as f32,
                        y as f32,
                        CAMERA_SENSITIVITY * delta_time as f32,
                    );
                }

                _ => {}
            }
        }

        // Update
        graphics
            .camera
            .track_input(&window, CAMERA_SPEED * delta_time as f32);

        model = ng::rotate(
            &model,
            4.0 * delta_time as f32,
            &ng::Vec3::new(1.0, 0.0, 1.0),
        );

        graphics.update();

        // Uniforms
        graphics.shaders.set_mat4("model", &model);

        // Render
        unsafe {
            gl::BindVertexArray(vao);

            gl::DrawElements(
                gl::TRIANGLES,
                INDICES.len() as i32,
                gl::UNSIGNED_INT,
                0 as _,
            );
        }

        window.swap_buffers();
        glfw.poll_events();
    }

    graphics.destroy();
}
