
use specs::{Component, VecStorage};

// components
#[derive(Default)]
pub struct Transform2D {
    pub position: [f32; 3],
    pub rotation: f32,
    pub size: f32,
}

impl Component for Transform2D {
    type Storage = VecStorage<Self>;
}

#[derive(Default)]
pub struct Velocity2D {
    pub position: [f32; 3],
    pub rotation: f32,
}

impl Component for Velocity2D {
    type Storage = VecStorage<Self>;
}

#[derive(Default)]
pub struct TriangleMesh {
    pub vertices: [f32; 9],
}

impl Component for TriangleMesh {
    type Storage = VecStorage<Self>;
}


