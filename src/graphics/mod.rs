mod camera;
mod renderable;
mod shader;

use crate::graphics::renderable::Renderable;
use crate::utils::Rect;
use nalgebra_glm as ng;

pub struct Graphics {
    screen_size: Rect<u32>,
    projection: ng::Mat4,
    pub shaders: shader::Shader,
    pub camera: camera::Camera,

    cube: renderable::cube::Cube,
}

impl Graphics {
    pub fn init() -> Self {
        unsafe {
            gl::ClearColor(18.0 / 255.0, 18.0 / 255.0, 18.0 / 255.0, 1.0);
            gl::Enable(gl::DEPTH_TEST);
        }

        Graphics {
            screen_size: Rect::new(0, 0, crate::INIT_WIDTH, crate::INIT_HEIGHT),
            projection: ng::perspective(
                crate::INIT_WIDTH as f32 / crate::INIT_HEIGHT as f32,
                45.0 * ng::pi::<f32>() / 180.0,
                0.1,
                100.0,
            ),
            shaders: shader::Shader::new(
                "assets/shaders/shader.glsl.vert",
                "assets/shaders/shader.glsl.frag",
            ),
            camera: camera::Camera::new(),

            cube: renderable::cube::Cube::new(),
        }
    }

    pub fn update(&mut self, delta_time: f32) {
        self.cube.update(delta_time);

        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }
        self.shaders.use_program();
        self.shaders.set_mat4("projection", &self.projection);
        self.shaders.set_mat4("view", &self.camera.view());

        self.cube.render(&self.shaders);
    }

    pub fn destroy(&self) {}

    pub fn resize(&mut self, width: u32, height: u32) {
        unsafe {
            gl::Viewport(0, 0, width as i32, height as i32);
        }
        self.screen_size.right = width;
        self.screen_size.bottom = height;
    }

    pub fn handle_input(&mut self, delta_time: f32, window: &glfw::Window) {
        self.camera
            .track_input(&window, crate::CAMERA_SPEED * delta_time as f32);
    }
}
