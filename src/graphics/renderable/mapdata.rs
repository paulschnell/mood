use super::RenderableShader;
use crate::graphics::renderable::sector::{
    Sector as SectorMesh, TextureData, Vertex, CEILING, FLOOR,
};
use crate::graphics::shader::Shader;
use crate::utils::{get_item, index_of};
use nalgebra_glm as ng;

pub struct Map {
    pub path: String,
    pub name: String,
    pub description: String,
    pub sectors: Vec<Sector>,
    pub spawn: (f32, f32, f32),
}

#[derive(Clone)]
pub struct Sector {
    pub floor: f32,
    pub ceiling: f32,
    pub corners: Vec<Corner>,
    pub gates: Vec<Gate>,
    pub mesh: SectorMesh,
}

pub type Corner = (f32, f32); // x, z

#[derive(Clone)]
pub struct Gate {
    pub own: u32,
    pub target_sector: u32,
    pub target_gate: u32,
}

impl Map {
    pub fn new() -> Self {
        Map {
            path: String::new(),
            name: String::new(),
            description: String::new(),
            sectors: Vec::new(),
            spawn: (0.0, 0.0, 0.0),
        }
    }

    pub fn load_from_file(&mut self, path: &str, shaders: &Shader) {
        self.path = format!("assets/maps/{path}");

        let mut map_json;
        {
            let input =
                std::fs::read_to_string(&self.path).expect("Could not find file '{self.path}'");
            map_json = json::parse(&input).unwrap();
        }
        self.name = map_json["name"].take_string().unwrap();
        self.description = map_json["description"].take_string().unwrap();
        self.spawn.2 = map_json["spawn"].pop().as_f32().unwrap();
        self.spawn.1 = map_json["spawn"].pop().as_f32().unwrap();
        self.spawn.0 = map_json["spawn"].pop().as_f32().unwrap();

        for sector_json in map_json["sectors"].members_mut() {
            let floor = sector_json["floor"].as_f32().unwrap();
            let ceiling = sector_json["ceiling"].as_f32().unwrap();

            let mut corners: Vec<Corner> = Vec::new();
            let mut gates: Vec<Gate> = Vec::new();
            let mut vertices: Vec<Vertex> = Vec::new();
            let mut indices: Vec<u32> = Vec::new();

            //
            // Create mesh
            //
            // Create vertices on floor and ceiling level#
            let mut last = (0.0, 0.0);
            for corner_json in sector_json["corners"].members_mut() {
                let z = corner_json.pop().as_f32().unwrap();
                let x = corner_json.pop().as_f32().unwrap();
                let diff = f32::sqrt(f32::abs(x - last.0).powi(2) + f32::abs(z - last.1).powi(2));
                vertices.push(Vertex {
                    x,
                    y: floor,
                    z: z * -1.0,
                    s_horizontal: x,
                    t_horizontal: z,
                    s_vertical: diff,
                    t_vertical: 0.0,
                    vtype: FLOOR,
                });
                vertices.push(Vertex {
                    x,
                    y: ceiling,
                    z: z * -1.0,
                    s_horizontal: x,
                    t_horizontal: z,
                    s_vertical: diff,
                    t_vertical: 1.0,
                    vtype: CEILING,
                });

                corners.push((x, z).clone());

                last = (x, z);
            }

            // Get Gates for Indices
            for _ in 0..sector_json["gates"].len() {
                let gate = sector_json["gates"].pop();
                gates.insert(
                    0,
                    Gate {
                        own: gate["own"].as_u32().unwrap(),
                        target_sector: gate["targetSector"].as_u32().unwrap(),
                        target_gate: gate["targetGate"].as_u32().unwrap(),
                    },
                );
            }

            // Create Indices
            let mut gates_index: Vec<u32> = Vec::new();
            for ele in gates.clone() {
                gates_index.push(ele.own);
            }

            //                infront of last element not
            for i in 0..(corners.len() as u32 - 1) {
                if !gates_index.contains(&i) {
                    indices.push(i * 2);
                    indices.push(i * 2 + 1);
                    indices.push(i * 2 + 2);
                    indices.push(i * 2 + 1);
                    indices.push(i * 2 + 3);
                    indices.push(i * 2 + 2);
                }
            }
            // between last and first element
            if !gates_index.contains(&(corners.len() as u32 - 1)) {
                indices.push((corners.len() as u32 - 1) * 2);
                indices.push((corners.len() as u32 - 1) * 2 + 1);
                indices.push(0);
                indices.push((corners.len() as u32 - 1) * 2 + 1);
                indices.push(1);
                indices.push(0);
            }

            // Floor and ceiling indices (polygon triangulation) -> Ear Clipping
            /*
            Rules:
             - ccw
             - defined in order
             - 2 edges
             - no intersection
             - no holes
             - not colinear (angle != 180°)
             */
            {
                let mut index_list: Vec<(f32, f32)> = corners.clone();
                while index_list.len() > 3 {
                    for i in 0..index_list.len() as isize {
                        // find valid ear
                        {
                            let a = &index_list[i as usize];
                            let b = get_item(&index_list, i + 1).unwrap();
                            let c = get_item(&index_list, i - 1).unwrap();

                            let va = ng::vec2(a.0, a.1);
                            let vb = ng::vec2(b.0, b.1);
                            let vc = ng::vec2(c.0, c.1);

                            // check if angle abc is convex
                            {
                                let va_to_vb = vb - va;
                                let va_to_vc = vc - va;
                                if ng::cross(
                                    &ng::vec2_to_vec3(&va_to_vb),
                                    &ng::vec2_to_vec3(&va_to_vc),
                                )
                                .z < 0.0
                                {
                                    continue;
                                }
                            }

                            // check if other points are in triangle
                            let mut in_triangle = false;
                            for j in 0..index_list.len() as isize {
                                if j == i || j == i - 1 || j == i + 1 {
                                    continue;
                                }

                                let p =
                                    ng::vec2(index_list[j as usize].0, index_list[j as usize].1);

                                if ng::cross(
                                    &ng::vec2_to_vec3(&(vc - va)),
                                    &ng::vec2_to_vec3(&(p - va)),
                                )
                                .z > 0.0
                                    && ng::cross(
                                        &ng::vec2_to_vec3(&(vb - vc)),
                                        &ng::vec2_to_vec3(&(p - vc)),
                                    )
                                    .z > 0.0
                                    && ng::cross(
                                        &ng::vec2_to_vec3(&(va - vb)),
                                        &ng::vec2_to_vec3(&(p - vb)),
                                    )
                                    .z > 0.0
                                {
                                    in_triangle = true;
                                    break;
                                }
                            }

                            if in_triangle {
                                continue;
                            }

                            // Add trinangles to Element Buffer
                            let ta = index_of(&corners, a).unwrap() as u32 * 2;
                            let tb = index_of(&corners, b).unwrap() as u32 * 2;
                            let tc = index_of(&corners, c).unwrap() as u32 * 2;

                            // floor
                            indices.push(ta);
                            indices.push(tb);
                            indices.push(tc);

                            // ceiling
                            indices.push(ta + 1);
                            indices.push(tc + 1);
                            indices.push(tb + 1);
                        } // let reference run out of scope

                        // Remove i from Indexlist
                        index_list.remove(i as usize);
                        index_list.shrink_to_fit();

                        break;
                    }
                }

                // Last triangle
                let ta = index_of(&corners, &index_list[0]).unwrap() as u32 * 2;
                let tb = index_of(&corners, &index_list[1]).unwrap() as u32 * 2;
                let tc = index_of(&corners, &index_list[2]).unwrap() as u32 * 2;

                // floor
                indices.push(ta);
                indices.push(tb);
                indices.push(tc);

                // ceiling
                indices.push(ta + 1);
                indices.push(tc + 1);
                indices.push(tb + 1);
            }

            //
            // Initilize and send first bumb of data
            //
            let sector = Sector {
                floor,
                ceiling,
                corners,
                gates,
                mesh: SectorMesh::new(
                    vertices,
                    indices,
                    TextureData {
                        wall: match sector_json["textures"]["wall"].as_str() {
                            Some(s) => format!("assets/textures/{s}").to_string(),
                            None => "assets/textures/fallback.png".to_string(),
                        },
                        floor: match sector_json["textures"]["floor"].as_str() {
                            Some(s) => format!("assets/textures/{s}").to_string(),
                            None => "assets/textures/fallback.png".to_string(),
                        },
                        ceiling: match sector_json["textures"]["ceiling"].as_str() {
                            Some(s) => format!("assets/textures/{s}").to_string(),
                            None => "assets/textures/fallback.png".to_string(),
                        },
                    },
                ),
            };

            self.sectors.push(sector);
        }

        {
            let cpy_sectors = self.sectors.clone();
            for sector in &mut self.sectors {
                // Indices between sectors
                for gate in &sector.gates {
                    // Floor
                    if sector.floor > cpy_sectors[gate.target_sector as usize].floor {
                        sector.mesh.vertices.push(Vertex {
                            x: sector.corners[gate.own as usize].0,
                            y: cpy_sectors[gate.target_sector as usize].floor,
                            z: -1.0 * sector.corners[gate.own as usize].1,
                            s_horizontal: 0.0,
                            t_horizontal: 0.0,
                            s_vertical: 0.0,
                            t_vertical: 0.0,
                            vtype: 0,
                        });

                        // if is between last and first
                        if gate.own as i32 == sector.corners.len() as i32 - 1 {
                            sector.mesh.vertices.push(Vertex {
                                x: sector.corners[0].0,
                                y: cpy_sectors[gate.target_sector as usize].floor,
                                z: -1.0 * sector.corners[0].1,
                                s_horizontal: 0.0,
                                t_horizontal: 0.0,
                                s_vertical: 0.0,
                                t_vertical: 0.0,
                                vtype: 0,
                            });
                        } else {
                            sector.mesh.vertices.push(Vertex {
                                x: sector.corners[gate.own as usize + 1].0,
                                y: cpy_sectors[gate.target_sector as usize].floor,
                                z: -1.0 * sector.corners[gate.own as usize + 1].1,
                                s_horizontal: 0.0,
                                t_horizontal: 0.0,
                                s_vertical: 0.0,
                                t_vertical: 0.0,
                                vtype: 0,
                            });
                        }

                        let vert_len = sector.mesh.vertices.len() as u32;
                        sector.mesh.indices.push(vert_len - 2);
                        sector.mesh.indices.push(vert_len - 1);
                        sector.mesh.indices.push(gate.own * 2);

                        sector.mesh.indices.push(gate.own * 2);
                        sector.mesh.indices.push(vert_len - 1);
                        sector.mesh.indices.push(
                            if gate.own as i32 == sector.corners.len() as i32 - 1 {
                                0
                            } else {
                                gate.own * 2 + 2
                            },
                        );
                    }

                    // Ceiling
                    if sector.ceiling < cpy_sectors[gate.target_sector as usize].ceiling {
                        sector.mesh.vertices.push(Vertex {
                            x: sector.corners[gate.own as usize].0,
                            y: cpy_sectors[gate.target_sector as usize].ceiling,
                            z: -1.0 * sector.corners[gate.own as usize].1,
                            s_horizontal: 0.0,
                            t_horizontal: 0.0,
                            s_vertical: 0.0,
                            t_vertical: 0.0,
                            vtype: 0,
                        });

                        // if is between last and first
                        if gate.own as i32 == sector.corners.len() as i32 - 1 {
                            sector.mesh.vertices.push(Vertex {
                                x: sector.corners[0].0,
                                y: cpy_sectors[gate.target_sector as usize].ceiling,
                                z: -1.0 * sector.corners[0].1,
                                s_horizontal: 0.0,
                                t_horizontal: 0.0,
                                s_vertical: 0.0,
                                t_vertical: 0.0,
                                vtype: 0,
                            });
                        } else {
                            sector.mesh.vertices.push(Vertex {
                                x: sector.corners[gate.own as usize + 1].0,
                                y: cpy_sectors[gate.target_sector as usize].ceiling,
                                z: -1.0 * sector.corners[gate.own as usize + 1].1,
                                s_horizontal: 0.0,
                                t_horizontal: 0.0,
                                s_vertical: 0.0,
                                t_vertical: 0.0,
                                vtype: 0,
                            });
                        }

                        let vert_len = sector.mesh.vertices.len() as u32;
                        sector.mesh.indices.push(vert_len - 2);
                        sector.mesh.indices.push(gate.own * 2 + 1);
                        sector.mesh.indices.push(vert_len - 1);

                        sector.mesh.indices.push(gate.own * 2 + 1);
                        sector.mesh.indices.push(
                            if gate.own as i32 == sector.corners.len() as i32 - 1 {
                                1
                            } else {
                                gate.own * 2 + 3
                            },
                        );
                        sector.mesh.indices.push(vert_len - 1);
                    }
                }
            }
        }

        self.create(shaders);
    }
}

impl RenderableShader for Map {
    fn create(&mut self, shaders: &Shader) {
        for sector in &mut self.sectors {
            sector.mesh.create(shaders);
        }
    }

    fn render(&self, shaders: &Shader) {
        for sector in &self.sectors {
            sector.mesh.render(shaders);
        }
    }

    fn update(&mut self, delta_time: f32) {
        for sector in &mut self.sectors {
            sector.mesh.update(delta_time);
        }
    }
}
