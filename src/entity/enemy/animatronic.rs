use super::{Enemy, EnemyTrait};

pub struct Animatronic {
    name: String,
    enemy: Enemy,
}

impl Animatronic {
    fn new(name: String) -> Self {
        let mut animatronic = Animatronic {
            name,
            enemy: Enemy::default(),
        };

        animatronic.load();

        animatronic
    }
}

impl EnemyTrait for Animatronic {
    fn load(&mut self, json_path: &json::JsonValue) {
        
    }

    fn update(&mut self) {
        
    }

    fn destroy(&mut self) {
        
    }

    fn go(&mut self, dir: nalgebra_glm::DVec2) {
        
    }

    fn target(&mut self, pos: (f64, f64), precision: f64) {
        
    }
}
