use nalgebra_glm as ng;

pub mod animatronic;

pub enum Status {
    Idle,
    Dead,
    Attack,
}

pub struct Enemy {
    status: Status,
    pos: (f64, f64, f64),
    speed: f64,
    health: u32,
    dmg: u32,
}

trait EnemyTrait {
    fn load(&mut self, json_path: &json::JsonValue);
    fn update(&mut self);
    fn destroy(&mut self);

    fn go(&mut self, dir: ng::DVec2);
    fn target(&mut self, pos: (f64, f64), precision: f64);
}

impl Default for Enemy {
    fn default() -> Self {
        Enemy {
            status: Status::Idle,
            pos: (0.0, 0.0, 0.0),
            speed: 1.0,
            health: 100,
            dmg: 10,
        }
    }
}
