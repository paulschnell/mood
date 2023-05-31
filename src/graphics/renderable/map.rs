extern crate json;

use crate::graphics::renderable::{Model, Renderable};
use crate::graphics::shader::Shader;
use crate::mapdata::{Data, Gate, Sector};

type Vertex = [f32; 3];

pub struct Map {
    model: Model,
    vertices: Vec<Vertex>,
    indices: Vec<u32>,
    data: Data,
}

impl Map {
    pub fn new(path: &str) -> Self {
        let mut new = Map {
            model: Model::default(),
            vertices: Vec::new(),
            indices: Vec::new(),
            data: Data::default(),
        };

        new.data.path = format!("assets/maps/{path}");

        let input =
            std::fs::read_to_string(&new.data.path).expect("Could not find file '{new.path}'");
        let mut map_json = json::parse(&input).unwrap();

        new.data.name = map_json["name"].take_string().unwrap();
        new.data.description = map_json["description"].take_string().unwrap();

        for sector in map_json["sectors"].members_mut() {
            let floor = sector["floor"].as_f32().unwrap();
            let ceiling = sector["ceiling"].as_f32().unwrap();

            let mut data = Sector {
                floor: floor.clone(),
                ceiling: ceiling.clone(),
                corners: Vec::new(),
                gates: Vec::new(),
            };

            // Create vertices on floor and ceiling level in current sector
            for corner in sector["corners"].members_mut() {
                let z = -1.0 * corner.pop().as_f32().unwrap();
                let x = corner.pop().as_f32().unwrap();
                new.vertices.push([x, floor - 0.5, z]); // floor
                new.vertices.push([x, ceiling - 0.5, z]); // ceiling

                data.corners.push((x, z).clone());
            }

            for _ in 0..sector["gates"].len() {
                let gate = sector["gates"].pop();
                data.gates.insert(
                    0,
                    Gate {
                        own: gate["own"].as_u32().unwrap(),
                        target_sector: gate["targetSector"].as_u32().unwrap(),
                        target_gate: gate["targetGate"].as_u32().unwrap(),
                    },
                );
            }

            let offset = new.vertices.len() as u32 - data.corners.len() as u32 * 2;

            {
                let mut gates_index: Vec<u32> = Vec::new();
                for ele in data.gates.clone() {
                    gates_index.push(ele.own);
                }

                // Create indices for sector
                for i in 0..(data.corners.len() as u32 - 1) {
                    if !gates_index.contains(&i) {
                        new.indices.push(offset + i * 2);
                        new.indices.push(offset + i * 2 + 1);
                        new.indices.push(offset + i * 2 + 2);
                        new.indices.push(offset + i * 2 + 1);
                        new.indices.push(offset + i * 2 + 3);
                        new.indices.push(offset + i * 2 + 2);
                    }
                }
                // between last and first element
                if !gates_index.contains(&(data.corners.len() as u32 - 1)) {
                    new.indices
                        .push(offset + (data.corners.len() as u32 - 1) * 2);
                    new.indices
                        .push(offset + (data.corners.len() as u32 - 1) * 2 + 1);
                    new.indices.push(offset);
                    new.indices
                        .push(offset + (data.corners.len() as u32 - 1) * 2 + 1);
                    new.indices.push(offset + 1);
                    new.indices.push(offset);
                }
            }

            // Floor and ceiling indices
            for i in 0..(data.corners.len() as u32 - 2) {
                new.indices.push(offset);
                new.indices.push(offset + i * 2 + 2);
                new.indices.push(offset + i * 2 + 4);
            }
            for i in 0..(data.corners.len() as u32 - 2) {
                new.indices.push(offset + 1);
                new.indices.push(offset + i * 2 + 5);
                new.indices.push(offset + i * 2 + 3);
            }

            new.data.sectors.push(data);
        }

        let mut offset_sector = 0;
        for i in 0..new.data.sectors.len() {
            let sector = &new.data.sectors[i];

            for gate in &sector.gates {
                let last_wall;
                let offset_gate = if gate.own == sector.corners.len() as u32 - 1 {
                    // if gate == last wall
                    last_wall = true;
                    0
                } else {
                    last_wall = false;
                    gate.own * 2
                };

                let mut offset_other_sector = 0;
                for j in 0..gate.target_sector {
                    offset_other_sector += new.data.sectors[j as usize].corners.len() as u32 * 2;
                }

                let other_last_wall;
                let offset_other_gate = if gate.target_gate
                    == new.data.sectors[gate.target_sector as usize].corners.len() as u32 - 1
                {
                    other_last_wall = true;
                    0
                } else {
                    other_last_wall = false;
                    gate.target_gate * 2
                };

                let other_floor_val = &new.data.sectors[gate.target_sector as usize].floor;
                let other_ceiling_val = &new.data.sectors[gate.target_sector as usize].ceiling;

                if sector.floor > *other_floor_val {
                    let mine_floor_p0 = offset_sector + offset_gate;
                    let mine_floor_p1 = if last_wall {
                        offset_sector + gate.own * 2
                    } else {
                        offset_sector + offset_gate + 2
                    };

                    let other_floor_p0 = offset_other_sector + offset_other_gate;
                    let other_floor_p1 = if other_last_wall {
                        offset_other_sector + gate.target_gate * 2
                    } else {
                        offset_other_sector + offset_other_gate + 2
                    };

                    new.indices.push(mine_floor_p0.clone());
                    new.indices.push(mine_floor_p1.clone());
                    new.indices.push(other_floor_p0.clone());

                    new.indices.push(other_floor_p0.clone());
                    new.indices.push(mine_floor_p1.clone());
                    new.indices.push(other_floor_p1.clone());
                }
                if sector.ceiling < *other_ceiling_val {
                    let mine_floor_p0 = offset_sector + offset_gate;
                    let mine_floor_p1 = if last_wall {
                        offset_sector + gate.own * 2
                    } else {
                        offset_sector + offset_gate + 2
                    };

                    let other_floor_p0 = offset_other_sector + offset_other_gate;
                    let other_floor_p1 = if other_last_wall {
                        offset_other_sector + gate.target_gate * 2
                    } else {
                        offset_other_sector + offset_other_gate + 2
                    };

                    new.indices.push(mine_floor_p0.clone() + 1);
                    new.indices.push(other_floor_p0.clone() + 1);
                    new.indices.push(mine_floor_p1.clone() + 1);

                    new.indices.push(other_floor_p0.clone() + 1);
                    new.indices.push(other_floor_p1.clone() + 1);
                    new.indices.push(mine_floor_p1.clone() + 1);
                }
            }

            offset_sector += new.data.sectors[i].corners.len() as u32 * 2;
        }

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
