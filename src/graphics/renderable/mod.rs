use image::io::Reader as ImageReader;
use nalgebra_glm as ng;

pub mod gui;
pub mod mapdata;
pub mod sector;

pub trait Renderable {
    fn create(&mut self);
    fn render(&self, shaders: &crate::graphics::shader::Shader);
    fn update(&mut self, delta_time: f32);
}

pub trait RenderableShader {
    fn create(&mut self, shaders: &crate::graphics::shader::Shader);
    fn render(&self, shaders: &crate::graphics::shader::Shader);
    fn update(&mut self, delta_time: f32);
}

#[derive(Clone)]
pub struct Model {
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

pub fn create_texture(id: u32, path: &str) {
    unsafe {
        gl::BindTexture(gl::TEXTURE_2D, id);

        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as i32);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as i32);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::NEAREST as i32);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::NEAREST as i32);

        let img = ImageReader::open(path)
            .unwrap()
            .decode()
            .unwrap()
            .into_rgba8();

        gl::TexImage2D(
            gl::TEXTURE_2D,
            0,
            gl::RGBA as i32,
            img.width() as i32,
            img.height() as i32,
            0,
            gl::RGBA,
            gl::UNSIGNED_BYTE,
            img.as_raw().as_ptr() as _,
        );
        gl::GenerateMipmap(gl::TEXTURE_2D);
    }
}
