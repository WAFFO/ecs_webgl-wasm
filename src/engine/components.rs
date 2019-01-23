
use specs::{Component, VecStorage};
use glm::{Vec3, vec3};

use engine::mesh_manager::UUID;


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
    pub mesh_id: UUID,
}

impl Component for StaticMesh {
    type Storage = VecStorage<Self>;
}


pub struct Camera {
    pub rotation: Vec3,
    pub target: Vec3,
}

impl Component for Camera {
    type Storage = VecStorage<Self>;
}

impl Default for Camera {
    fn default() -> Camera {
        Camera {
            rotation: vec3(0.0, 0.0, 1.0),
            target: vec3(0.0, 0.0, 0.0),
        }
    }
}