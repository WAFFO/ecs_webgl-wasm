
use specs::{World, Builder, Entity};
use glm::{Vec3, vec3, vec4};

use engine::components::*;
use engine::mesh_manager::UUID;

pub fn test_solid(world: &mut World, mesh: UUID, translation: Vec3, scale: f32, rotation: Vec3) -> Entity {
    world.create_entity()
        .with(Transform  { translation, rotation: vec3( 0.0, 0.0, 0.0 ), scale: vec3(scale, scale, scale) })
        .with(Velocity   { translation: vec3( 0.0, 0.0, 0.0 ), rotation })
        .with(StaticMesh { mesh_id: mesh } )
        .with(Solid)
        .build()
}

pub fn test_light(world: &mut World, mesh: UUID, translation: Vec3, scale: f32, rotation: Vec3) -> Entity {
    world.create_entity()
        .with(Transform  { translation, rotation: vec3( 0.0, 0.0, 0.0 ), scale: vec3(scale, scale, scale) })
        .with(Velocity   { translation: vec3( 0.0, 0.0, 0.0 ), rotation })
        .with(StaticMesh { mesh_id: mesh } )
        .with(Light { color: vec4(1.0, 0.5, 0.5, 1.0) })
        .build()
}

pub fn camera(world: &mut World, pitch: f32, yaw: f32, target: Vec3) -> Entity {
    world.create_entity()
        .with(Camera  { rotation: vec3(0.0, 0.0, 0.0), target, pitch, yaw, pole_arm: 0.1 })
        .build()
}