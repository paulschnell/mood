use super::{create_texture, Model, RenderableShader};
use crate::graphics::shader::Shader;

pub const _UNDEFINED: u32 = 0;
pub const FLOOR: u32 = 1;
pub const CEILING: u32 = 2;
pub const WALL: u32 = 3;
pub const GATE: u32 = 4;

#[derive(Clone)]
pub struct Vertex {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub s: f32,
    pub t: f32,
    pub vtype: u32,
}

#[derive(Clone)]
pub struct Sector {
    wall_model: Model,
    pub wall_vertices: Vec<Vertex>,
    pub wall_indices: Vec<u32>,

    planes_model: Model,
    pub planes_vertices: Vec<Vertex>,
    pub planes_indices: Vec<u32>,

    texture: TextureData,
}

#[derive(Clone)]
pub struct TextureData {
    pub wall: (String, u32),
    pub floor: (String, u32),
    pub ceiling: (String, u32),
    pub gate: (String, u32),
}

impl Sector {
    pub fn new(
        wall_vertices: Vec<Vertex>,
        wall_indices: Vec<u32>,
        planes_vertices: Vec<Vertex>,
        planes_indices: Vec<u32>,
        texture: TextureData,
    ) -> Self {
        Sector {
            wall_model: Model::default(),
            wall_vertices,
            wall_indices,

            planes_model: Model::default(),
            planes_vertices,
            planes_indices,

            texture,
        }
    }

    fn draw(&self, shaders: &Shader, model: &Model, indices_len: i32) {
        shaders.set_mat4("model", &model.transform);

        unsafe {
            gl::BindVertexArray(model.vao);
            gl::DrawElements(gl::TRIANGLES, indices_len, gl::UNSIGNED_INT, 0 as _);
        }
    }
}

impl RenderableShader for Sector {
    fn create(&mut self, shaders: &Shader) {
        unsafe {
            // WALLS
            gl::GenVertexArrays(1, &mut self.wall_model.vao);
            gl::GenBuffers(1, &mut self.wall_model.vbo);
            gl::GenBuffers(1, &mut self.wall_model.ebo);

            gl::BindVertexArray(self.wall_model.vao);

            gl::BindBuffer(gl::ARRAY_BUFFER, self.wall_model.vbo);
            gl::BufferData(
                gl::ARRAY_BUFFER,
                self.wall_vertices
                    .as_ptr_range()
                    .end
                    .offset_from(self.wall_vertices.as_ptr_range().start)
                    * std::mem::size_of::<Vertex>() as isize,
                self.wall_vertices.as_ptr().cast(),
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

            gl::VertexAttribIPointer(
                3,
                1,
                gl::UNSIGNED_INT,
                std::mem::size_of::<Vertex>().try_into().unwrap(),
                (std::mem::size_of::<f32>() * 5) as *const _,
            );
            gl::EnableVertexAttribArray(3);

            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, self.wall_model.ebo);
            gl::BufferData(
                gl::ELEMENT_ARRAY_BUFFER,
                self.wall_indices
                    .as_ptr_range()
                    .end
                    .offset_from(self.wall_indices.as_ptr_range().start)
                    * std::mem::size_of::<u32>() as isize,
                self.wall_indices.as_ptr().cast(),
                gl::STATIC_DRAW,
            );

            gl::BindBuffer(gl::ARRAY_BUFFER, 0);

            // PLANES
            gl::GenVertexArrays(1, &mut self.planes_model.vao);
            gl::GenBuffers(1, &mut self.planes_model.vbo);
            gl::GenBuffers(1, &mut self.planes_model.ebo);

            gl::BindVertexArray(self.planes_model.vao);

            gl::BindBuffer(gl::ARRAY_BUFFER, self.planes_model.vbo);
            gl::BufferData(
                gl::ARRAY_BUFFER,
                self.planes_vertices
                    .as_ptr_range()
                    .end
                    .offset_from(self.planes_vertices.as_ptr_range().start)
                    * std::mem::size_of::<Vertex>() as isize,
                self.planes_vertices.as_ptr().cast(),
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

            gl::VertexAttribIPointer(
                3,
                1,
                gl::UNSIGNED_INT,
                std::mem::size_of::<Vertex>().try_into().unwrap(),
                (std::mem::size_of::<f32>() * 5) as *const _,
            );
            gl::EnableVertexAttribArray(3);

            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, self.planes_model.ebo);
            gl::BufferData(
                gl::ELEMENT_ARRAY_BUFFER,
                self.planes_indices
                    .as_ptr_range()
                    .end
                    .offset_from(self.planes_indices.as_ptr_range().start)
                    * std::mem::size_of::<u32>() as isize,
                self.planes_indices.as_ptr().cast(),
                gl::STATIC_DRAW,
            );

            gl::BindBuffer(gl::ARRAY_BUFFER, 0);

            shaders.use_program();

            gl::GenTextures(1, &mut self.texture.floor.1);
            create_texture(self.texture.floor.1, &self.texture.floor.0);
            shaders.set_i32("tx_floor", &0);

            gl::GenTextures(1, &mut self.texture.ceiling.1);
            create_texture(self.texture.ceiling.1, &self.texture.ceiling.0);
            shaders.set_i32("tx_ceiling", &1);

            gl::GenTextures(1, &mut self.texture.wall.1);
            create_texture(self.texture.wall.1, &self.texture.wall.0);
            shaders.set_i32("tx_wall", &2);

            gl::GenTextures(1, &mut self.texture.gate.1);
            create_texture(self.texture.gate.1, &self.texture.gate.0);
            shaders.set_i32("tx_gate", &3);
        }
    }

    fn render(&self, shaders: &Shader) {
        shaders.use_program();
        unsafe {
            gl::ActiveTexture(gl::TEXTURE0);
            gl::BindTexture(gl::TEXTURE_2D, self.texture.floor.1);
            gl::ActiveTexture(gl::TEXTURE1);
            gl::BindTexture(gl::TEXTURE_2D, self.texture.ceiling.1);
            gl::ActiveTexture(gl::TEXTURE2);
            gl::BindTexture(gl::TEXTURE_2D, self.texture.wall.1);
            gl::ActiveTexture(gl::TEXTURE3);
            gl::BindTexture(gl::TEXTURE_2D, self.texture.gate.1);
        }

        self.draw(shaders, &self.wall_model, self.wall_indices.len() as i32);
        self.draw(
            shaders,
            &self.planes_model,
            self.planes_indices.len() as i32,
        );
    }

    fn update(&mut self, _delta_time: f32) {}
}
