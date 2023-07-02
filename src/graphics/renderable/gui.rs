use super::{Model, Renderable};
use crate::graphics::{guimanager::Component, shader::Shader};

#[derive(Clone)]
pub struct Vertex {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub r: f32,
    pub s: f32,
}

pub struct Gui {
    model: Model,
    vertices: Vec<Vertex>,
    indices: Vec<u32>,

    components: Vec<Component>,
}

impl Gui {
    pub fn new() -> Self {
        Gui {
            model: Model::default(),
            vertices: Vec::new(),
            indices: Vec::new(),

            components: Vec::new(),
        }
    }

    pub fn init(&mut self) {
        for i in 0..self.components.len() {
            self.components[i].create_model(&mut self.vertices, &mut self.indices);
        }

        self.create();
    }

    pub fn add(&mut self, component: Component) {
        self.components.push(component);
    }

    pub fn update_vertex(&self, offset: isize, data: &Vec<Vertex>) {
        unsafe {
            gl::BindBuffer(gl::ARRAY_BUFFER, self.model.vbo);
            let ptr = gl::MapBuffer(gl::ARRAY_BUFFER, gl::WRITE_ONLY);
            std::ptr::copy_nonoverlapping(
                data.as_ptr().cast(),
                ptr.add(offset as usize),
                data.len() * std::mem::size_of::<Vertex>(),
            );
            gl::UnmapBuffer(gl::ARRAY_BUFFER);
        }
    }

    pub fn vertices_clone(&self) -> Vec<Vertex> {
        self.vertices.clone()
    }
}

impl Renderable for Gui {
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
                gl::DYNAMIC_DRAW,
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
        shaders.use_program();
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
