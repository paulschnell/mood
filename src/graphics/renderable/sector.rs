use super::{ModelTxs, RenderableShader};
use crate::graphics::shader::Shader;
use image::io::Reader as ImageReader;

pub const FLOOR: u32 = 1;
pub const CEILING: u32 = 2;

#[derive(Clone)]
pub struct Vertex {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub s_horizontal: f32,
    pub t_horizontal: f32,
    pub s_vertical: f32,
    pub t_vertical: f32,
    pub vtype: u32,
}

#[derive(Clone)]
pub struct Sector {
    pub model: ModelTxs,
    pub vertices: Vec<Vertex>,
    pub indices: Vec<u32>,
    pub texture: TextureData,
    txs_ids: Vec<u32>,
}

#[derive(Clone)]
pub struct TextureData {
    pub wall: String,
    pub floor: String,
    pub ceiling: String,
}

impl Sector {
    pub fn new(vertices: Vec<Vertex>, indices: Vec<u32>, texture: TextureData) -> Self {
        Sector {
            model: ModelTxs::default(),
            vertices,
            indices,
            texture,
            txs_ids: Vec::new(),
        }
    }
}

impl RenderableShader for Sector {
    fn create(&mut self, shaders: &Shader) {
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

            gl::VertexAttribPointer(
                1,
                2,
                gl::FLOAT,
                gl::FALSE,
                std::mem::size_of::<Vertex>().try_into().unwrap(),
                (std::mem::size_of::<f32>() * 3) as *const _,
            );
            gl::EnableVertexAttribArray(1);

            gl::VertexAttribPointer(
                2,
                2,
                gl::FLOAT,
                gl::FALSE,
                std::mem::size_of::<Vertex>().try_into().unwrap(),
                (std::mem::size_of::<f32>() * 5) as *const _,
            );
            gl::EnableVertexAttribArray(2);

            gl::VertexAttribIPointer(
                3,
                1,
                gl::UNSIGNED_INT,
                std::mem::size_of::<Vertex>().try_into().unwrap(),
                (std::mem::size_of::<f32>() * 7) as *const _,
            );
            gl::EnableVertexAttribArray(3);

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

            self.txs_ids.resize(3, 0);
            shaders.use_program();

            gl::GenTextures(1, &mut self.txs_ids[0]);
            create_texture(self.txs_ids[0], &self.texture.floor);
            shaders.set_i32("tx_floor", &0);

            gl::GenTextures(1, &mut self.txs_ids[1]);
            create_texture(self.txs_ids[1], &self.texture.ceiling);
            shaders.set_i32("tx_ceiling", &1);

            gl::GenTextures(1, &mut self.txs_ids[2]);
            create_texture(self.txs_ids[2], &self.texture.wall);
            shaders.set_i32("tx_wall", &2);

            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
        }
    }

    fn render(&self, shaders: &Shader) {
        shaders.set_mat4("model", &self.model.transform);
        unsafe {
            for i in 0..3 {
                gl::ActiveTexture(gl::TEXTURE0 + i);
                gl::BindTexture(gl::TEXTURE_2D, self.txs_ids[i as usize]);
            }
        }

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

fn create_texture(id: u32, path: &str) {
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
            gl::RGB as i32,
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
