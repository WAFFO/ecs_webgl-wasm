
use specs::{Component, VecStorage};
use glm::{Vec3, vec3};


// components
pub struct Transform {
    pub translation: Vec3,
    pub rotation: Vec3,
    pub scale: Vec3,
}

impl Component for Transform {
    type Storage = VecStorage<Self>;
}

impl Default for Transform {
    fn default() -> Transform {
        Transform {
            translation: vec3(0.0, 0.0, 0.0),
            rotation: vec3(0.0, 0.0, 0.0),
            scale: vec3(0.0, 0.0, 0.0),
        }
    }
}

pub struct Velocity {
    pub translation: Vec3,
    pub rotation: Vec3,
}

impl Component for Velocity {
    type Storage = VecStorage<Self>;
}

impl Default for Velocity {
    fn default() -> Velocity {
        Velocity {
            translation: vec3(0.0, 0.0, 0.0),
            rotation: vec3(0.0, 0.0, 0.0),
        }
    }
}

// TODO: impl std::ops::Add for Transform + Velocity

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