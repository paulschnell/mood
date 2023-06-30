use crate::graphics::camera::{Camera, UP};
use crate::graphics::renderable::mapdata::Map;
use crate::utils::Line;
use nalgebra_glm as ng;

pub const CAMERA_SENSITIVITY: f64 = 7.0;

const FORWARD_SPEED: f64 = 1.0;
const BACK_SPEED: f64 = 0.5;
const STRAVE_SPEED: f64 = 0.75;
const FLY_SPEED: f64 = 1.0;
const SPEED_FAC: f64 = 2.0;

const PLAYER_HEIGHT: f32 = 0.65;

pub struct Player {
    camera: Camera,
    spectator: bool,
    grounded: bool,

    cur_sector: u32,
    next_pos: (f64, f64),
}

impl Player {
    pub fn new(spawn: (f32, f32, f32)) -> Self {
        let mut player = Player {
            camera: Camera::new(),
            spectator: false,
            grounded: false,

            cur_sector: 0,
            next_pos: (spawn.0 as f64, spawn.2 as f64 * -1.0),
        };

        player.camera.put(
            spawn.0 as f64,
            (PLAYER_HEIGHT + spawn.1) as f64,
            -1.0 * spawn.2 as f64,
            0.0,
            0.0,
        );

        player
    }

    pub fn cam_view(&self) -> ng::Mat4 {
        self.camera.view().cast()
    }

    pub fn mouse_input(&mut self, x: f64, y: f64, delta_time: f64) {
        self.camera.cur_mov(x, y, delta_time * CAMERA_SENSITIVITY);
    }

    pub fn key_input(&mut self, window: &glfw::Window, delta_time: f64, map: &Map) {
        let xz_front = &self.camera.xz_front();
        let cur_pos = (self.camera.pos().x, self.camera.pos().z);

        let mut mov_change = ng::DVec3::zeros();

        if window.get_key(glfw::Key::W) == glfw::Action::Press {
            mov_change += xz_front
                * delta_time
                * if self.spectator {
                    FLY_SPEED
                } else {
                    FORWARD_SPEED
                        * if (window.get_key(glfw::Key::D) == glfw::Action::Press)
                            ^ (window.get_key(glfw::Key::A) == glfw::Action::Press)
                        {
                            0.5
                        } else {
                            1.0
                        }
                }
                * SPEED_FAC;
        }

        if window.get_key(glfw::Key::S) == glfw::Action::Press {
            mov_change += xz_front
                * delta_time
                * if self.spectator {
                    FLY_SPEED
                } else {
                    BACK_SPEED
                        * if (window.get_key(glfw::Key::D) == glfw::Action::Press)
                            ^ (window.get_key(glfw::Key::A) == glfw::Action::Press)
                        {
                            0.5
                        } else {
                            1.0
                        }
                }
                * SPEED_FAC
                * -1.0;
        }

        if window.get_key(glfw::Key::D) == glfw::Action::Press {
            mov_change += ng::normalize(&ng::cross(xz_front, &UP))
                * delta_time
                * if self.spectator {
                    FLY_SPEED
                } else {
                    STRAVE_SPEED
                        * if (window.get_key(glfw::Key::W) == glfw::Action::Press)
                            ^ (window.get_key(glfw::Key::S) == glfw::Action::Press)
                        {
                            0.5
                        } else {
                            1.0
                        }
                }
                * SPEED_FAC;
        }

        if window.get_key(glfw::Key::A) == glfw::Action::Press {
            mov_change += ng::normalize(&ng::cross(xz_front, &UP))
                * delta_time
                * if self.spectator {
                    FLY_SPEED
                } else {
                    STRAVE_SPEED
                        * if (window.get_key(glfw::Key::W) == glfw::Action::Press)
                            ^ (window.get_key(glfw::Key::S) == glfw::Action::Press)
                        {
                            0.5
                        } else {
                            1.0
                        }
                }
                * SPEED_FAC
                * -1.0;
        }

        if window.get_key(glfw::Key::Space) == glfw::Action::Press {
            if self.spectator {
                mov_change += UP * delta_time * FLY_SPEED * SPEED_FAC;
            } else if self.grounded {
                self.jump();
            }
        }
        if window.get_key(glfw::Key::LeftShift) == glfw::Action::Press {
            if self.spectator {
                mov_change += UP * delta_time * FLY_SPEED * SPEED_FAC * -1.0;
            }
        }

        if mov_change != ng::DVec3::zeros() {
            if !self.spectator {
                self.next_pos = (
                    (*self.camera.pos() + mov_change).x,
                    (*self.camera.pos() + mov_change).z,
                );

                let sector = &map.sectors[self.cur_sector as usize];
                for i in 0..sector.corners.len() {
                    let corner0 = &sector.corners[i];
                    let corner1 = &sector.corners[if i + 1 == sector.corners.len() {
                        0
                    } else {
                        i + 1
                    }];

                    if Line::new_tuples(cur_pos, self.next_pos).crosses(&Line::new(
                        corner0.0 as f64,
                        corner0.1 as f64 * -1.0,
                        corner1.0 as f64,
                        corner1.1 as f64 * -1.0,
                    )) {
                        // Check if collided with a gate
                        let mut is_gate = false;
                        for gate in &sector.gates {
                            if i == gate.own as usize {
                                is_gate = true;
                                let entering = &map.sectors[gate.target_sector as usize];
                                if entering.ceiling - entering.floor > PLAYER_HEIGHT {
                                    self.cur_sector = gate.target_sector;
                                } else {
                                    // Cancel Move
                                    mov_change = ng::DVec3::zeros();
                                }
                                break;
                            }
                        }

                        if !is_gate {
                            mov_change = ng::DVec3::zeros();
                        }
                        break;
                    }
                }

                let sector = &map.sectors[self.cur_sector as usize];
                // Put onto ground
                self.camera.put_y((sector.floor + PLAYER_HEIGHT) as f64);
            }

            *self.camera.pos() += mov_change; // Move
        }
    }

    fn jump(&mut self) {}

    pub fn toggle_spectator(&mut self) {
        self.spectator = !self.spectator;
    }
}
