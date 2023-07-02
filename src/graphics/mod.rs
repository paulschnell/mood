pub mod camera;
pub mod guimanager;
mod guis;
pub mod renderable;
mod shader;

use crate::utils::Rect;
use nalgebra_glm as ng;
use renderable::mapdata::Map;
use renderable::RenderableShader;

use self::guimanager::GuiManager;

pub struct Graphics {
    screen_size: Rect<u32>,
    paused: bool,
    projection: ng::Mat4,

    gui_manager: GuiManager,

    map_shader: shader::Shader,
    map: Map,
}

impl Graphics {
    pub fn init() -> Self {
        unsafe {
            gl::ClearColor(55.0 / 255.0, 96.0 / 255.0, 97.0 / 255.0, 1.0);

            gl::Enable(gl::CULL_FACE);

            gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
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

            gui_manager: GuiManager::new(guimanager::ActiveInterface::TEST),

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
        unsafe {
            gl::Enable(gl::DEPTH_TEST);
        }
        self.map.render(&self.map_shader);

        unsafe {
            gl::Enable(gl::BLEND);
        }
        self.gui_manager.render();
        unsafe {
            gl::Disable(gl::BLEND);
        }
        
        unsafe {
            gl::Disable(gl::DEPTH_TEST);
        }
    }

    pub fn destroy(&self) {}

    pub fn resize(&mut self, width: u32, height: u32) {
        unsafe {
            gl::Viewport(0, 0, width as i32, height as i32);
        }

        // self.gui_manager.active_interface_mut().resize(
        //     width as f32,
        //     height as f32,
        //     self.screen_size.right as f32,
        //     self.screen_size.bottom as f32,
        // );

        self.screen_size.right = width;
        self.screen_size.bottom = height;
    }

    pub fn pause(&mut self) {
        self.map_shader.use_program();
        self.map_shader.set_i32("bPause", &1);
        self.paused = true;
        self.gui_manager.set_gui(guimanager::ActiveInterface::PAUSE);
    }

    pub fn unpause(&mut self) {
        self.map_shader.use_program();
        self.map_shader.set_i32("bPause", &0);
        self.paused = false;
        self.gui_manager.set_gui(guimanager::ActiveInterface::TEST);
    }

    pub fn spawn(&self) -> (f32, f32, f32) {
        self.map.spawn
    }

    pub fn map(&self) -> &Map {
        &self.map
    }

    pub fn gui_manager(&self) -> &guimanager::GuiManager {
        &self.gui_manager
    }
}
