use crate::{utils::deg_to_rad, INIT_HEIGHT, INIT_WIDTH};
use nalgebra_glm as ng;

pub struct Camera {
    pos: ng::Vec3,
    front: ng::Vec3,
    up: ng::Vec3,
    yaw: f32,
    pitch: f32,
    cur_last_pos: (f32, f32),
    first_move: bool,
}

impl Camera {
    pub fn new() -> Self {
        Camera {
            pos: ng::Vec3::zeros(),
            front: ng::Vec3::new(0.0, 0.0, -1.0),
            up: ng::Vec3::new(0.0, 1.0, 0.0),
            yaw: -90.0,
            pitch: 0.0,
            cur_last_pos: (INIT_WIDTH as f32 / 2.0, INIT_HEIGHT as f32 / 2.0),
            first_move: true,
        }
    }

    pub fn foward(&mut self, speed: f32) {
        self.pos += self.front * speed;
    }

    pub fn right(&mut self, speed: f32) {
        self.pos += ng::normalize(&ng::cross(&self.front, &self.up)) * speed;
    }

    pub fn up(&mut self, speed: f32) {
        self.pos += self.up * speed;
    }

    pub fn cur_mov(&mut self, x: f32, y: f32, sensitivity: f32) {
        if self.first_move {
            self.cur_last_pos = (x, y);
            self.first_move = false;
        }

        let offset = (
            (x as f32 - self.cur_last_pos.0) * sensitivity,
            (self.cur_last_pos.1 - y as f32) * sensitivity,
        );
        self.yaw += offset.0;
        self.pitch += offset.1;
        self.cur_last_pos = (x as f32, y as f32);

        if self.pitch > 89.9 {
            self.pitch = 89.9;
        }
        if self.pitch < -89.9 {
            self.pitch = -89.9;
        }

        let direction = ng::Vec3::new(
            f32::cos(deg_to_rad(self.yaw)) * f32::cos(deg_to_rad(self.pitch)),
            f32::sin(deg_to_rad(self.pitch)),
            f32::sin(deg_to_rad(self.yaw)) * f32::cos(deg_to_rad(self.pitch)),
        );
        self.front = ng::normalize(&direction);
    }

    pub fn view(&self) -> ng::Mat4 {
        ng::look_at(&self.pos, &(self.pos + self.front), &self.up)
    }

    pub fn track_input(&mut self, window: &glfw::Window, speed: f32) {
        if window.get_key(glfw::Key::W) == glfw::Action::Press {
            self.foward(speed);
        }
        if window.get_key(glfw::Key::S) == glfw::Action::Press {
            self.foward(-1.0 * speed);
        }
        if window.get_key(glfw::Key::A) == glfw::Action::Press {
            self.right(-1.0 * speed);
        }
        if window.get_key(glfw::Key::D) == glfw::Action::Press {
            self.right(speed);
        }
        if window.get_key(glfw::Key::Space) == glfw::Action::Press {
            self.up(speed);
        }
        if window.get_key(glfw::Key::C) == glfw::Action::Press {
            self.up(-1.0 * speed);
        }
    }
}
