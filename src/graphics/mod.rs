pub mod camera;
pub mod renderable;
mod shader;

use crate::utils::Rect;
use nalgebra_glm as ng;
use renderable::mapdata::Map;
use renderable::RenderableShader;

pub struct Graphics {
    screen_size: Rect<u32>,
    paused: bool,
    projection: ng::Mat4,

    map_shader: shader::Shader,
    map: Map,
}

impl Graphics {
    pub fn init() -> Self {
        unsafe {
            gl::ClearColor(55.0 / 255.0, 96.0 / 255.0, 97.0 / 255.0, 1.0);
            gl::Enable(gl::DEPTH_TEST);

            gl::Enable(gl::CULL_FACE);
        }

        let mut graphics = Graphics {
            screen_size: Rect::new(0, 0, crate::INIT_WIDTH, crate::INIT_HEIGHT),
            paused: false,
            projection: ng::perspective(
                crate::INIT_WIDTH as f32 / crate::INIT_HEIGHT as f32,
                45.0 * ng::pi::<f32>() / 180.0,
                0.001,
                100.0,
            ),

            map_shader: shader::Shader::new(
                "assets/shaders/map.glsl.vert",
                "assets/shaders/map.glsl.frag",
            ),
            map: Map::new(),
        };

        graphics
            .map
            .load_from_file("test2.json", &graphics.map_shader);

        graphics
    }

    pub fn update(&mut self, delta_time: f32, view: &ng::Mat4) {
        if !self.paused {
            self.map.update(delta_time);
        }

        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }

        if !self.paused {
            self.map_shader.use_program();
            self.map_shader.set_mat4("projection", &self.projection);
            self.map_shader.set_mat4("view", view);
        }

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

    pub fn pause(&mut self) {
        self.map_shader.use_program();
        self.map_shader.set_i32("bPause", &1);
        self.paused = true;
    }

    pub fn unpause(&mut self) {
        self.map_shader.use_program();
        self.map_shader.set_i32("bPause", &0);
        self.paused = false;
    }

    pub fn spawn(&self) -> (f32, f32, f32) {
        self.map.spawn
    }

    pub fn map(&self) -> &Map {
        &self.map
    }
}
