use crate::{utils::deg_to_rad, INIT_HEIGHT, INIT_WIDTH};
use nalgebra_glm as ng;

pub const UP: ng::DVec3 = ng::DVec3::new(0.0, 1.0, 0.0);

pub struct Camera {
    pos: ng::DVec3,

    front: ng::DVec3,
    xz_front: ng::DVec3,
    yaw: f64,
    pitch: f64,

    cur_last_pos: (f64, f64),
    first_move: bool,
}

impl Camera {
    pub fn new() -> Self {
        Camera {
            pos: ng::DVec3::zeros(),

            front: ng::DVec3::new(0.0, 0.0, -1.0),
            xz_front: ng::DVec3::new(0.0, 0.0, -1.0),
            yaw: -90.0,
            pitch: 0.0,

            cur_last_pos: (INIT_WIDTH as f64 / 2.0, INIT_HEIGHT as f64 / 2.0),
            first_move: true,
        }
    }

    pub fn put(&mut self, x: f64, y: f64, z: f64, yaw: f64, pitch: f64) {
        self.pos.x = x;
        self.pos.y = y;
        self.pos.z = z;
        self.yaw = yaw;
        self.pitch = pitch;
    }

    pub fn put_xz(&mut self, x: f64, z: f64) {
        self.pos.x = x;
        self.pos.z = z;
    }

    pub fn put_y(&mut self, y: f64) {
        self.pos.y = y;
    }

    pub fn cur_mov(&mut self, x: f64, y: f64, sensitivity: f64) {
        if self.first_move {
            self.cur_last_pos = (x, y);
            self.first_move = false;
        }

        let offset = (
            (x - self.cur_last_pos.0) * sensitivity,
            (self.cur_last_pos.1 - y) * sensitivity,
        );
        self.yaw += offset.0;
        self.pitch += offset.1;
        self.cur_last_pos = (x, y);

        if self.pitch > 89.9 {
            self.pitch = 89.9;
        }
        if self.pitch < -89.9 {
            self.pitch = -89.9;
        }

        let direction = ng::DVec3::new(
            f64::cos(deg_to_rad(self.yaw)) * f64::cos(deg_to_rad(self.pitch)),
            f64::sin(deg_to_rad(self.pitch)),
            f64::sin(deg_to_rad(self.yaw)) * f64::cos(deg_to_rad(self.pitch)),
        );
        self.front = ng::normalize(&direction);

        let direction = ng::DVec3::new(
            f64::cos(deg_to_rad(self.yaw)),
            0.0,
            f64::sin(deg_to_rad(self.yaw)),
        );
        self.xz_front = ng::normalize(&direction);
    }

    pub fn pos(&mut self) -> &mut ng::DVec3 {
        &mut self.pos
    }

    pub fn view(&self) -> ng::DMat4 {
        ng::look_at(&self.pos, &(self.pos + self.front), &UP)
    }

    pub fn set_first_move(&mut self, value: bool) {
        self.first_move = value;
    }

    pub fn xz_front(&self) -> ng::DVec3 {
        self.xz_front.clone()
    }
}
