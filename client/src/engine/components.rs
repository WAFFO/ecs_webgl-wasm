
use specs::{Component, VecStorage};
use math::{Vert3, Vert4};

use engine::mesh_manager::UUID;


// components
pub struct Transform {
    pub translation: Vert3,
    pub rotation: Vert3,
    pub scale: Vert3,
}

impl Component for Transform {
    type Storage = VecStorage<Self>;
}

impl Default for Transform {
    fn default() -> Transform {
        Transform {
            translation: Vert3::new(0.0, 0.0, 0.0),
            rotation: Vert3::new(0.0, 0.0, 0.0),
            scale: Vert3::new(0.0, 0.0, 0.0),
        }
    }
}

pub struct Velocity {
    pub translation: Vert3,
    pub rotation: Vert3,
}

impl Component for Velocity {
    type Storage = VecStorage<Self>;
}

impl Default for Velocity {
    fn default() -> Velocity {
        Velocity {
            translation: Vert3::new(0.0, 0.0, 0.0),
            rotation: Vert3::new(0.0, 0.0, 0.0),
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
    pub rotation: Vert3,
    pub target: Vert3,
    pub pitch: f32,
    pub yaw: f32,
    pub pole_arm: f32,
}

//   side view    |    top view
//      c         | t<----z cos(yaw)
//     /|y sin(p) |   \  |
//    / |         |    \ |
//   /  |         |     \|x sin(yaw)
// t<---|xz cos(p)|      c

impl Camera {
    pub fn update(&mut self) {
        self.rotation[0] = self.pitch.cos() * self.yaw.sin() * self.pole_arm;
        self.rotation[1] = self.pitch.sin() * self.pole_arm;
        self.rotation[2] = self.pitch.cos() * self.yaw.cos() * self.pole_arm;
    }
}

impl Component for Camera {
    type Storage = VecStorage<Self>;
}

impl Default for Camera {
    fn default() -> Camera {
        Camera {
            rotation: Vert3::new(0.0, 0.0, 0.0),
            target: Vert3::new(0.0, 0.0, 0.0),
            pitch: 0.0,
            yaw: 0.0,
            pole_arm: 0.1,
        }
    }
}


// entity types
#[derive(Default)]
pub struct Solid;
impl Component for Solid {
    type Storage = VecStorage<Self>;
}

pub struct Light{
    pub color: Vert4,
}
impl Component for Light {
    type Storage = VecStorage<Self>;
}
impl Default for Light {
    fn default() -> Light {
        Light {
            color: Vert4::new(1.0, 1.0, 1.0, 1.0),
        }
    }
}