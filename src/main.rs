extern crate gl;
extern crate glfw;
extern crate nalgebra_glm;

use glfw::Context;
use nalgebra_glm as ng;

const INIT_WIDTH: u32 = 1280;
const INIT_HEIGHT: u32 = 720;

const CAMERA_SPEED: f32 = 4.0;
const CAMERA_SENSITIVITY: f32 = 2.0;

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
    let mut cur_last_pos: (f32, f32) = (wnd_last_size.0 as f32 / 2.0, wnd_last_size.1 as f32 / 2.0);

    // OpenGL
    gl::load_with(|s| window.get_proc_address(s));

    unsafe {
        gl::Viewport(0, 0, wnd_last_size.0, wnd_last_size.1);
        gl::ClearColor(18.0 / 255.0, 18.0 / 255.0, 18.0 / 255.0, 1.0);
        gl::Enable(gl::DEPTH_TEST);
    }

    // Shader
    let vert_shader: u32;
    let frag_shader: u32;
    unsafe {
        vert_shader = gl::CreateShader(gl::VERTEX_SHADER);
        frag_shader = gl::CreateShader(gl::FRAGMENT_SHADER);
    }
    // load shader from string
    unsafe {
        let shader_source = std::fs::read_to_string("assets/shaders/shader.glsl.vert")
            .expect("Cannot read assets/shaders/shader.glsl.vert");

        gl::ShaderSource(
            vert_shader,
            1,
            &(shader_source.as_str().as_bytes().as_ptr().cast()),
            &(shader_source.len().try_into().unwrap()),
        );
        gl::CompileShader(vert_shader);
    }
    check_for_error(&vert_shader, "VERTEX");

    unsafe {
        let shader_source = std::fs::read_to_string("assets/shaders/shader.glsl.frag")
            .expect("Cannot read assets/shaders/shader.glsl.frag");
        gl::ShaderSource(
            frag_shader,
            1,
            &(shader_source.as_str().as_bytes().as_ptr().cast()),
            &(shader_source.len().try_into().unwrap()),
        );
        gl::CompileShader(frag_shader);
    }
    check_for_error(&frag_shader, "FRAGMENT");

    let shader_program;
    unsafe {
        shader_program = gl::CreateProgram();
        gl::AttachShader(shader_program, vert_shader);
        gl::AttachShader(shader_program, frag_shader);
        gl::LinkProgram(shader_program);
    }
    check_for_error(&shader_program, "PROGRAM");

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
    let projection = ng::perspective(1280.0 / 720.0, 45.0 * ng::pi::<f32>() / 180.0, 0.1, 100.0);

    let mut view = ng::Mat4::identity();

    let mut model = ng::Mat4::identity();
    model = ng::scale(&view, &ng::Vec3::new(0.5, 0.5, 0.5));

    // Camera
    let mut camera_pos = ng::Vec3::new(0.0, 0.0, 3.0);
    let mut camera_front = ng::Vec3::new(0.0, 0.0, -1.0);
    let camera_up = ng::Vec3::new(0.0, 1.0, 0.0);
    let mut yaw: f32 = -90.0; //
    let mut pitch: f32 = 0.0;

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

                glfw::WindowEvent::Size(width, height) => unsafe {
                    gl::Viewport(0, 0, width, height);
                },

                glfw::WindowEvent::CursorPos(pos_x, pos_y) => {
                    let offset = (
                        (pos_x as f32 - cur_last_pos.0) * CAMERA_SENSITIVITY * delta_time as f32,
                        (cur_last_pos.1 - pos_y as f32) * CAMERA_SENSITIVITY * delta_time as f32,
                    );
                    yaw += offset.0;
                    pitch += offset.1;
                    cur_last_pos = (pos_x as f32, pos_y as f32);

                    let direction = ng::Vec3::new(
                        f32::cos(deg_to_rad(yaw)) * f32::cos(deg_to_rad(pitch)),
                        f32::sin(deg_to_rad(pitch)),
                        f32::sin(deg_to_rad(yaw)) * f32::cos(deg_to_rad(pitch)),
                    );
                    camera_front = ng::normalize(&direction);
                }

                _ => {}
            }
        }

        // Update
        if window.get_key(glfw::Key::W) == glfw::Action::Press {
            camera_pos += CAMERA_SPEED * delta_time as f32 * camera_front;
        }
        if window.get_key(glfw::Key::S) == glfw::Action::Press {
            camera_pos -= CAMERA_SPEED * delta_time as f32 * camera_front;
        }
        if window.get_key(glfw::Key::A) == glfw::Action::Press {
            camera_pos -= ng::normalize(&ng::cross(&camera_front, &camera_up))
                * CAMERA_SPEED
                * delta_time as f32;
        }
        if window.get_key(glfw::Key::D) == glfw::Action::Press {
            camera_pos += ng::normalize(&ng::cross(&camera_front, &camera_up))
                * CAMERA_SPEED
                * delta_time as f32;
        }
        if window.get_key(glfw::Key::Space) == glfw::Action::Press {
            camera_pos += CAMERA_SPEED * delta_time as f32 * camera_up;
        }
        if window.get_key(glfw::Key::C) == glfw::Action::Press {
            camera_pos -= CAMERA_SPEED * delta_time as f32 * camera_up;
        }
        view = ng::look_at(&camera_pos, &(camera_pos + camera_front), &camera_up);
        model = ng::rotate(
            &model,
            4.0 * delta_time as f32,
            &ng::Vec3::new(1.0, 0.0, 1.0),
        );

        // Uniforms
        let proj_mat_name = std::ffi::CString::new("projection").expect("CString::new failed.");
        unsafe {
            gl::UniformMatrix4fv(
                gl::GetUniformLocation(shader_program, proj_mat_name.as_ptr().cast()),
                1,
                gl::FALSE,
                projection.as_ptr(),
            );
        }
        let view_mat_name = std::ffi::CString::new("view").expect("CString::new failed.");
        unsafe {
            gl::UniformMatrix4fv(
                gl::GetUniformLocation(shader_program, view_mat_name.as_ptr().cast()),
                1,
                gl::FALSE,
                view.as_ptr(),
            );
        }
        let model_mat_name = std::ffi::CString::new("model").expect("CString::new failed.");
        unsafe {
            gl::UniformMatrix4fv(
                gl::GetUniformLocation(shader_program, model_mat_name.as_ptr()),
                1,
                gl::FALSE,
                model.as_ptr(),
            );
        }

        // Render
        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);

            gl::UseProgram(shader_program);

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
}

fn check_for_error(shader: &u32, shader_type: &str) {
    let mut success = 0;
    if shader_type.eq_ignore_ascii_case("PROGRAM") {
        unsafe {
            gl::GetProgramiv(*shader, gl::LINK_STATUS, &mut success);
        }
        if success == 0 {
            let mut v: Vec<u8> = Vec::with_capacity(1024);
            let mut log_len: i32 = 0;
            unsafe {
                gl::GetProgramInfoLog(*shader, 1024, &mut log_len, v.as_mut_ptr().cast());
                v.set_len(log_len.try_into().unwrap());
            }
            panic!(
                "Shader Program Linking Error: {}",
                String::from_utf8_lossy(&v)
            );
        }
    } else {
        unsafe {
            gl::GetShaderiv(*shader, gl::COMPILE_STATUS, &mut success);
        }
        if success == 0 {
            let mut v: Vec<u8> = Vec::with_capacity(1024);
            let mut log_len: i32 = 0;
            unsafe {
                gl::GetShaderInfoLog(*shader, 1024, &mut log_len, v.as_mut_ptr().cast());
                v.set_len(log_len.try_into().unwrap());
            }
            panic!(
                "Shader Compiling Error By {shader_type}: {}",
                String::from_utf8_lossy(&v)
            );
        }
    }
}

pub fn deg_to_rad(deg: f32) -> f32 {
    deg * ng::pi::<f32>() / 180.0
}
