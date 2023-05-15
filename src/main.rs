extern crate gl;
extern crate glfw;
extern crate nalgebra_glm;

use glfw::Context;
// use nalgebra_glm as ng;

struct Vertex {
    pos: [f32; 3],
    color: [f32; 3],
}

const VERTICES: [Vertex; 3] = [
    Vertex {
        pos: [0.0, 0.5, 0.0],
        color: [1.0, 0.0, 0.0],
    },
    Vertex {
        pos: [0.5, -0.5, 0.0],
        color: [0.0, 1.0, 0.0],
    },
    Vertex {
        pos: [-0.5, -0.5, 0.0],
        color: [0.0, 0.0, 1.0],
    },
];

fn main() {
    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).expect("Failed on initilizing glfw.");

    let (mut window, events) = glfw
        .create_window(
            1280,
            720,
            "mood - C++ mag niemand",
            glfw::WindowMode::Windowed,
        )
        .expect("Failed to create window.");

    window.set_key_polling(true);
    window.set_size_polling(true);
    window.set_cursor_pos_polling(true);
    window.set_scroll_polling(true);
    window.make_current();

    let wnd_last_size = window.get_size();

    // OpenGL
    gl::load_with(|s| window.get_proc_address(s));

    unsafe {
        gl::Viewport(0, 0, wnd_last_size.0, wnd_last_size.1);
        gl::ClearColor(18.0 / 255.0, 18.0 / 255.0, 18.0 / 255.0, 1.0);
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

    let shader_program;
    unsafe {
        shader_program = gl::CreateProgram();
        gl::AttachShader(shader_program, vert_shader);
        gl::AttachShader(shader_program, frag_shader);
        gl::LinkProgram(shader_program);
    }

    let mut vao = 0;
    let mut vbo = 0;
    unsafe {
        gl::GenVertexArrays(1, &mut vao);
        gl::GenBuffers(1, &mut vbo);

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

        gl::BindBuffer(gl::ARRAY_BUFFER, 0); // unbind
    }

    // Main loop
    while !window.should_close() {
        for (_, event) in glfw::flush_messages(&events) {
            handle_event(&event, &mut window);
        }

        // Update
        // ...

        // Render
        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT);

            gl::UseProgram(shader_program);

            gl::BindVertexArray(vao);

            gl::DrawArrays(gl::TRIANGLES, 0, VERTICES.len() as i32);
        }

        window.swap_buffers();
        glfw.poll_events();
    }
}

fn handle_event(event: &glfw::WindowEvent, window: &mut glfw::Window) {
    match event {
        glfw::WindowEvent::Key(glfw::Key::Escape, _, glfw::Action::Press, _) => {
            window.set_should_close(true);
        }

        glfw::WindowEvent::Size(width, height) => unsafe {
            gl::Viewport(0, 0, *width, *height);
        },

        _ => {}
    }
}
