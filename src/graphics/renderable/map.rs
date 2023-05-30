extern crate json;

use crate::graphics::renderable::{Model, Renderable};
use crate::graphics::shader::Shader;

type Vertex = [f32; 6];

pub struct Map {
    model: Model,
    path: String,
    vertices: Vec<Vertex>,
    indices: Vec<u32>,
    name: String,
    description: String,
}

impl Map {
    pub fn new(path: &str) -> Self {
        let mut new = Map {
            model: Model::default(),
            path: format!("assets/maps/{}", path),
            vertices: Vec::new(),
            indices: Vec::new(),
            name: String::new(),
            description: String::new(),
        };

        let input = std::fs::read_to_string(&new.path).expect("Could not find file '{new.path}'");
        let mut map_json = json::parse(&input).unwrap();

        new.name = map_json["name"].take_string().unwrap();
        new.description = map_json["description"].take_string().unwrap();

        for sector in map_json["sectors"].members_mut() {
            let floor = sector["floor"].as_f32().unwrap();
            let ceiling = sector["ceiling"].as_f32().unwrap();

            // Create vertices on floor and ceiling level in current sector
            let mut n_corners = 0;
            for corner in sector["corners"].members_mut() {
                let z = -1.0 * corner.pop().as_f32().unwrap();
                let x = corner.pop().as_f32().unwrap();
                new.vertices.push([x, floor - 0.5, z, 0.3, 0.3, 0.3]); // floor
                new.vertices.push([x, ceiling - 0.5, z, 0.7, 0.7, 0.7]); // ceiling
                n_corners += 1;
            }

            let mut gates: Vec<u32> = Vec::new();
            for _ in 0..sector["gates"].len() {
                gates.insert(0, sector["gates"].pop().as_u32().unwrap());
            }
            gates.sort();

            // Create indices for sector
            let offset = new.vertices.len() as u32 - n_corners * 2;
            for i in 0..(n_corners - 1) {
                if !gates.contains(&i) {
                    new.indices.push(offset + i * 2);
                    new.indices.push(offset + i * 2 + 1);
                    new.indices.push(offset + i * 2 + 2);
                    new.indices.push(offset + i * 2 + 1);
                    new.indices.push(offset + i * 2 + 3);
                    new.indices.push(offset + i * 2 + 2);
                }
            }
            // between last and first element
            if !gates.contains(&(n_corners - 1)) {
                new.indices.push(offset + (n_corners - 1) * 2);
                new.indices.push(offset + (n_corners - 1) * 2 + 1);
                new.indices.push(offset);
                new.indices.push(offset + (n_corners - 1) * 2 + 1);
                new.indices.push(offset + 1);
                new.indices.push(offset);
            }
        }

        // Create indices between sectors in gates

        new.create();

        new
    }
}

impl Renderable for Map {
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
                self.indices
                    .as_ptr_range()
                    .end
                    .offset_from(self.indices.as_ptr_range().start)
                    * std::mem::size_of::<u32>() as isize,
                self.indices.as_ptr().cast(),
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
                self.indices.len() as i32,
                gl::UNSIGNED_INT,
                0 as _,
            );
        }
    }

    fn update(&mut self, delta_time: f32) {}
}
