
use specs::{World, Builder, Entity};

use engine::components::*;
use engine::mesh_manager::UUID;
use math::{Vert3, Vert4};

pub fn test_solid(world: &mut World, mesh: UUID, translation: Vert3, scale: f32, rotation: Vert3) -> Entity {
    world.create_entity()
        .with(Transform  { translation, rotation: Vert3::new(0.0, 0.0, 0.0), scale: Vert3::new(scale, scale, scale) })
        .with(Velocity   { translation: Vert3::new(0.0, 0.0, 0.0), rotation })
        .with(StaticMesh { mesh_id: mesh } )
        .with(Solid)
        .build()
}

pub fn test_light(world: &mut World, mesh: UUID, translation: Vert3, scale: f32, rotation: Vert3) -> Entity {
    world.create_entity()
        .with(Transform  { translation, rotation: Vert3::new(0.0, 0.0, 0.0), scale: Vert3::new(scale, scale, scale) })
        .with(Velocity   { translation: Vert3::new(0.0, 0.0, 0.0), rotation })
        .with(StaticMesh { mesh_id: mesh } )
        .with(Light { color: Vert4::new(1.0, 1.0, 1.0, 1.0) })
        .build()
}

pub fn camera(world: &mut World, pitch: f32, yaw: f32, target: Vert3) -> Entity {
    world.create_entity()
        .with(Camera  { rotation: Vert3::new(0.0, 0.0, 0.0), target, pitch, yaw, pole_arm: 0.1 })
        .build()
}