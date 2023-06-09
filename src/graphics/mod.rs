mod camera;
mod renderable;
mod shader;

use crate::utils::Rect;
use nalgebra_glm as ng;
use renderable::mapdata::Map;
use renderable::RenderableShader;

pub struct Graphics {
    screen_size: Rect<u32>,
    projection: ng::Mat4,
    pub camera: camera::Camera,

    map_shader: shader::Shader,
    map: Map,
}

impl Graphics {
    pub fn init() -> Self {
        unsafe {
            gl::ClearColor(55.0 / 255.0, 96.0 / 255.0, 97.0 / 255.0, 1.0);
            gl::Enable(gl::DEPTH_TEST);

            gl::Enable(gl::CULL_FACE);
            gl::CullFace(gl::BACK);
        }

        let mut graphics = Graphics {
            screen_size: Rect::new(0, 0, crate::INIT_WIDTH, crate::INIT_HEIGHT),
            projection: ng::perspective(
                crate::INIT_WIDTH as f32 / crate::INIT_HEIGHT as f32,
                45.0 * ng::pi::<f32>() / 180.0,
                0.001,
                100.0,
            ),
            camera: camera::Camera::new(),

            map_shader: shader::Shader::new(
                "assets/shaders/map.glsl.vert",
                "assets/shaders/map.glsl.frag",
            ),
            map: Map::new(),
        };

        graphics
            .map
            .load_from_file("test.json", &graphics.map_shader);

        graphics.camera.put(
            graphics.map.spawn.0,
            graphics.map.spawn.1,
            -1.0 * graphics.map.spawn.2,
            0.0,
            0.0,
        );

        graphics
    }

    pub fn update(&mut self, delta_time: f32) {
        self.map.update(delta_time);

        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }
        self.map_shader.use_program();
        self.map_shader.set_mat4("projection", &self.projection);
        self.map_shader.set_mat4("view", &self.camera.view());

        self.map.render(&self.map_shader);
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
