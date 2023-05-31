use nalgebra_glm as ng;

pub mod gui;
pub mod map;
pub mod triangle;

pub trait Renderable {
    fn create(&mut self);
    fn render(&self, shaders: &crate::graphics::shader::Shader);
    fn update(&mut self, delta_time: f32);
}

struct Model {
    pub vao: u32,
    pub vbo: u32,
    pub ebo: u32,
    pub transform: ng::Mat4,
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
