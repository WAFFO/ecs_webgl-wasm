
use specs::{Component, VecStorage};

// components
#[derive(Default)]
pub struct Transform {
    pub translation: [f32; 4],
    pub rotation: [f32; 4],
    pub scale: [f32; 4],
}

impl Component for Transform {
    type Storage = VecStorage<Self>;
}

#[derive(Default)]
pub struct Velocity {
    pub translation: [f32; 4],
    pub rotation: [f32; 4],
}

impl Component for Velocity {
    type Storage = VecStorage<Self>;
}

#[derive(Default)]
pub struct StaticMesh {
    pub vertices: Vec<f32>,
}

impl Component for StaticMesh {
    type Storage = VecStorage<Self>;
}

#[derive(Default)]
pub struct StaticColorMesh {
    pub vertices: Vec<f32>,
}

impl Component for StaticColorMesh {
    type Storage = VecStorage<Self>;
}

pub struct Render {
    pub vertices: Vec<f32>,
    visible: bool,
}

impl Component for Render {
    type Storage = VecStorage<Self>;
}

impl Default for Render {
    fn default() -> Render {
        Render {
            vertices: Vec::new(),
            visible: false,
        }
    }
}

impl Render {
    pub fn visible(&self) -> bool {
        self.visible
    }
    pub fn set_visible(&mut self, v: bool) {
        self.visible = v;
    }
}