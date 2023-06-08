use nalgebra_glm as ng;

pub mod gui;
pub mod triangle;
pub mod sector;
pub mod mapdata;

pub trait Renderable {
    fn create(&mut self);
    fn render(&self, shaders: &crate::graphics::shader::Shader);
    fn update(&mut self, delta_time: f32);
}

#[derive(Clone)]
struct Model {
    pub vao: u32,
    pub vbo: u32,
    pub ebo: u32,
    pub transform: ng::Mat4,
}

#[derive(Clone)]
pub struct ModelTxs {
    pub vao: u32,
    pub vbo: u32,
    pub ebo: u32,
    pub transform: ng::Mat4,
    pub textures: Vec<u32>,
}

impl Default for Model {
    fn default() -> Self {
        Model {
            vao: 0,
            vbo: 0,
            ebo: 0,
            transform: ng::Mat4::identity(),
        }
    }
}

impl Default for ModelTxs {
    fn default() -> Self {
        ModelTxs {
            vao: 0,
            vbo: 0,
            ebo: 0,
            transform: ng::Mat4::identity(),
            textures: Vec::new(),
        }
    }
}
