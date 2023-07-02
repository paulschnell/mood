use super::{
    guis::*,
    renderable::{
        create_texture,
        gui::{Gui as Interface, Vertex},
        Renderable,
    },
    shader::Shader,
};
use crate::{utils::Rect, INIT_HEIGHT, INIT_WIDTH};

pub const ATLAS_PATH: &str = "assets/textures/gui.png";

#[derive(Clone, Debug)]
pub enum ActiveInterface {
    NONE,
    TEST,
    PAUSE,
    HELP,
}

pub struct GuiManager {
    shader: Shader,
    tx_id: u32,
    active: ActiveInterface,

    empty: Interface,
    test: Interface,
    pause: Interface,
    help: Interface,
}

impl GuiManager {
    pub fn new(active: ActiveInterface) -> Self {
        let mut manager = GuiManager {
            shader: Shader::new(
                "assets/shaders/gui.glsl.vert",
                "assets/shaders/gui.glsl.frag",
            ),
            tx_id: 0,
            active,

            empty: Interface::new(),
            test: Interface::new(),
            pause: Interface::new(),
            help: Interface::new(),
        };

        test_gui(
            &mut manager.test,
            &Rect {
                left: 0.0,
                top: INIT_HEIGHT as f32,
                right: INIT_WIDTH as f32,
                bottom: 0.0,
            },
        );

        manager.shader.use_program();
        unsafe {
            gl::GenTextures(1, &mut manager.tx_id);
        }
        create_texture(manager.tx_id, ATLAS_PATH);
        manager.shader.set_i32("atlas", &0);
        unsafe {
            gl::ActiveTexture(gl::TEXTURE0);
            gl::BindTexture(gl::TEXTURE_2D, manager.tx_id);
        }

        manager
    }

    pub fn set_gui(&mut self, interface: ActiveInterface) -> ActiveInterface {
        let before = self.active.clone();
        self.active = interface;
        before
    }

    pub fn active_interface(&self) -> &Interface {
        match self.active {
            ActiveInterface::NONE => &self.empty,
            ActiveInterface::TEST => &self.test,
            ActiveInterface::PAUSE => &self.pause,
            ActiveInterface::HELP => &self.help,
        }
    }

    pub fn active_interface_mut(&mut self) -> &mut Interface {
        match self.active {
            ActiveInterface::NONE => &mut self.empty,
            ActiveInterface::TEST => &mut self.test,
            ActiveInterface::PAUSE => &mut self.pause,
            ActiveInterface::HELP => &mut self.help,
        }
    }

    pub fn render(&self) {
        self.shader.use_program();
        self.shader.set_i32("atlas", &0);
        unsafe {
            gl::ActiveTexture(gl::TEXTURE0);
            gl::BindTexture(gl::TEXTURE_2D, self.tx_id);
        }
        self.active_interface().render(&self.shader);
    }

    pub fn set_hp(&self, num: u32) {
        let mut data = self.test.vertices_clone();
        // remove every other vertex which does not belong to the hp points
        for _ in 0..(2 * 4) {
            data.remove(0);
        }
        while data.len() > 5 * 4 {
            data.remove(data.len() - 1);
        }

        // set z
        for i in 0..5 {
            for j in 0..4 {
                data[i * 4 + j].z = if i as isize >= num as isize { 0.0 } else { 0.2 } * -1.0;
            }
        }

        self.test
            .update_vertex(2 * 4 * std::mem::size_of::<Vertex>() as isize, &data);
    }
}

pub struct Component {
    rect: Rect<f32>, // x, y, width, height
    tx_rect: Rect<f32>,
    z: f32,
}

impl Component {
    pub fn new(x: f32, y: f32, width: f32, height: f32, tx_rect: Rect<f32>, z: f32) -> Self {
        Component {
            rect: Rect {
                left: x,
                top: y,
                right: x + width,
                bottom: y + height,
            },
            tx_rect,
            z: -z,
        }
    }

    pub fn create_model(&self, out_vertices: &mut Vec<Vertex>, out_indices: &mut Vec<u32>) {
        let rect = &self.rect;
        let texture = &self.tx_rect;

        out_vertices.push(Vertex {
            x: rect.right * 2.0 - 1.0,
            y: rect.top * 2.0 - 1.0,
            z: self.z,
            r: texture.left + texture.right,
            s: texture.top + texture.bottom,
        });
        out_vertices.push(Vertex {
            x: rect.left * 2.0 - 1.0,
            y: rect.top * 2.0 - 1.0,
            z: self.z,
            r: texture.left,
            s: texture.top + texture.bottom,
        });
        out_vertices.push(Vertex {
            x: rect.left * 2.0 - 1.0,
            y: rect.bottom * 2.0 - 1.0,
            z: self.z,
            r: texture.left,
            s: texture.top,
        });
        out_vertices.push(Vertex {
            x: rect.right * 2.0 - 1.0,
            y: rect.bottom * 2.0 - 1.0,
            z: self.z,
            r: texture.left + texture.right,
            s: texture.top,
        });

        out_indices.push(out_vertices.len() as u32 - 4);
        out_indices.push(out_vertices.len() as u32 - 1);
        out_indices.push(out_vertices.len() as u32 - 2);

        out_indices.push(out_vertices.len() as u32 - 4);
        out_indices.push(out_vertices.len() as u32 - 2);
        out_indices.push(out_vertices.len() as u32 - 3);
    }
}
