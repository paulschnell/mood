extern crate image;

use image::io::Reader as ImageReader;

use crate::graphics::renderable::{Model, Renderable};
use crate::graphics::shader::Shader;

type Vertex = [f32; 5];
const VERTICES: [Vertex; 4] = [
    [1.0, 1.0, -3.0, 1.0, 1.0],
    [-1.0, 1.0, -3.0, 0.0, 1.0],
    [-1.0, -1.0, -3.0, 0.0, 0.0],
    [1.0, -1.0, -3.0, 0.0, 1.0],
];
const INDICES: [u8; 6] = [0, 1, 2, 2, 3, 0];

pub struct Triangle {
    model: Model,
    texture: u32,
}

impl Triangle {
    pub fn new() -> Self {
        let mut new = Triangle {
            model: Model::default(),
            texture: 0,
        };

        new.create();

        new
    }
}

impl Renderable for Triangle {
    fn create(&mut self) {
        unsafe {
            gl::GenVertexArrays(1, &mut self.model.vao);
            gl::GenBuffers(1, &mut self.model.vbo);
            gl::GenBuffers(1, &mut self.model.ebo);

            gl::BindVertexArray(self.model.vao);

            gl::BindBuffer(gl::ARRAY_BUFFER, self.model.vbo);
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
                2,
                gl::FLOAT,
                gl::FALSE,
                std::mem::size_of::<Vertex>().try_into().unwrap(),
                (std::mem::size_of::<f32>() * 3) as *const _,
            );
            gl::EnableVertexAttribArray(1);

            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, self.model.ebo);
            gl::BufferData(
                gl::ELEMENT_ARRAY_BUFFER,
                std::mem::size_of_val(&INDICES) as isize,
                INDICES.as_ptr().cast(),
                gl::STATIC_DRAW,
            );

            // Texture stuff
            gl::GenTextures(1, &mut self.texture);
            gl::ActiveTexture(gl::TEXTURE0);
            gl::BindTexture(gl::TEXTURE_2D, self.texture);

            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::NEAREST as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::NEAREST as i32);

            let img = ImageReader::open("assets/textures/floor.png")
                .unwrap()
                .decode()
                .unwrap()
                .into_rgba8();

            gl::TexImage2D(
                gl::TEXTURE_2D,
                0,
                gl::RGB as i32,
                img.width() as i32,
                img.height() as i32,
                0,
                gl::RGBA,
                gl::UNSIGNED_BYTE,
                img.as_raw().as_ptr() as _,
            );
            gl::GenerateMipmap(gl::TEXTURE_2D);

            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
        }
    }

    fn render(&self, shaders: &Shader) {
        unsafe {
            gl::ActiveTexture(gl::TEXTURE0);
            gl::BindTexture(gl::TEXTURE_2D, self.texture);
        }

        shaders.set_i32("texture0", &0);
        shaders.set_mat4("model", &self.model.transform);
        unsafe {
            gl::BindVertexArray(self.model.vao);
            gl::DrawElements(
                gl::TRIANGLES,
                INDICES.len() as i32,
                gl::UNSIGNED_BYTE,
                0 as _,
            );
        }
    }

    fn update(&mut self, delta_time: f32) {}
}
