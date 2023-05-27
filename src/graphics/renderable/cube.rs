use crate::graphics::renderable::{Model, Renderable};
use crate::graphics::shader::Shader;
use nalgebra_glm as ng;

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

pub struct Cube {
    model: Model,
}

impl Cube {
    pub fn new() -> Self {
        let mut new = Cube {
            model: Model::default(),
        };

        new.create();

        new.model.transform = ng::scale(&new.model.transform, &ng::Vec3::new(0.5, 0.5, 0.5));

        new
    }
}

impl Renderable for Cube {
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
                3,
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

            gl::BindBuffer(gl::ARRAY_BUFFER, 0); // unbind
        }
    }

    fn render(&self, shaders: &Shader) {
        shaders.set_mat4("model", &self.model.transform);
        unsafe {
            gl::BindVertexArray(self.model.vao);
            gl::DrawElements(
                gl::TRIANGLES,
                INDICES.len() as i32,
                gl::UNSIGNED_INT,
                0 as _,
            );
        }
    }

    fn update(&mut self, delta_time: f32) {
        self.model.transform = ng::rotate(
            &self.model.transform,
            4.0 * delta_time,
            &ng::Vec3::new(1.0, 0.0, 1.0),
        );
    }
}
