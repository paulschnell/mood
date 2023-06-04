pub struct Data {
    pub path: String,
    pub name: String,
    pub description: String,
    pub sectors: Vec<Sector>,
}

pub struct Sector {
    pub floor: f32,
    pub ceiling: f32,
    pub corners: Vec<Corner>,
    pub gates: Vec<Gate>,
    pub textures: Texture,
}

pub type Corner = (f32, f32); // x, z

#[derive(Clone)]
pub struct Gate {
    pub own: u32,
    pub target_sector: u32,
    pub target_gate: u32,
}

impl Default for Data {
    fn default() -> Self {
        Data {
            path: String::new(),
            name: String::new(),
            description: String::new(),
            sectors: Vec::new(),
        }
    }
}

pub struct Texture {
    pub wall: String,
    pub floor: String,
    pub ceiling: String,
}

impl Default for Texture {
    fn default() -> Self {
        Texture {
            wall: "fallback.png".to_string(),
            floor: "fallback.png".to_string(),
            ceiling: "fallback.png".to_string(),
        }
    }
}
