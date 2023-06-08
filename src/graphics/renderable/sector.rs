use crate::graphics::renderable::{ModelTxs, Renderable};
use crate::graphics::shader::Shader;

pub type Vertex = [f32; 3];

#[derive(Clone)]
pub struct Sector {
    pub model: ModelTxs,
    pub vertices: Vec<Vertex>,
    pub indices: Vec<u32>,
}

impl Sector {
    pub fn new(vertices: Vec<Vertex>, indices: Vec<u32>) -> Self {
        Sector {
            model: ModelTxs::default(),
            vertices,
            indices,
        }
    }
}

impl Renderable for Sector {
    fn create(&mut self) {
        unsafe {
            gl::GenVertexArrays(1, &mut self.model.vao);
            gl::GenBuffers(1, &mut self.model.vbo);
            gl::GenBuffers(1, &mut self.model.ebo);

            gl::BindVertexArray(self.model.vao);

            gl::BindBuffer(gl::ARRAY_BUFFER, self.model.vbo);
            gl::BufferData(
                gl::ARRAY_BUFFER,
                self.vertices
                    .as_ptr_range()
                    .end
                    .offset_from(self.vertices.as_ptr_range().start)
                    * std::mem::size_of::<Vertex>() as isize,
                self.vertices.as_ptr().cast(),
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

            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, self.model.ebo);
            gl::BufferData(
                gl::ELEMENT_ARRAY_BUFFER,
                self.indices
                    .as_ptr_range()
                    .end
                    .offset_from(self.indices.as_ptr_range().start)
                    * std::mem::size_of::<u32>() as isize,
                self.indices.as_ptr().cast(),
                gl::STATIC_DRAW,
            );

            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
        }
    }

    fn render(&self, shaders: &Shader) {
        shaders.set_mat4("model", &self.model.transform);

        unsafe {
            gl::BindVertexArray(self.model.vao);
            gl::DrawElements(
                gl::TRIANGLES,
                self.indices.len() as i32,
                gl::UNSIGNED_INT,
                0 as _,
            );
        }
    }

    fn update(&mut self, _delta_time: f32) {}
}
