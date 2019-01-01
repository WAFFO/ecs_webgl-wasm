
use specs::{World, Builder, Entity};

use engine_mod::components::*;

pub fn triangle(world: &mut World, x: f32, y: f32, size: f32, velocity: (f32, f32, f32)) -> Entity {
    world.create_entity()
        .with(Transform2D  { position: [ x, y, 0.0], rotation: 0.0, size })
        .with(Velocity2D   { position: [ velocity.0, velocity.1, 0.0], rotation: velocity.2 })
        .with(TriangleMesh::default() )
        .build()
}