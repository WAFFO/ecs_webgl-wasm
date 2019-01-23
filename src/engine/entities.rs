
use specs::{World, Builder, Entity};
use glm::{Vec3, vec3};

use engine::components::*;
use engine::mesh_manager::UUID;

pub fn test_3d(world: &mut World, mesh: UUID, translation: Vec3, scale: f32, rotation: Vec3) -> Entity {
    world.create_entity()
        .with(Transform  { translation, rotation: vec3( 0.0, 0.0, 0.0 ), scale: vec3(scale, scale, scale) })
        .with(Velocity   { translation: vec3( 0.0, 0.0, 0.0 ), rotation })
        .with(StaticMesh { mesh_id: mesh } )
        .build()
}

pub fn camera(world: &mut World, rotation: Vec3, target: Vec3) -> Entity {
    world.create_entity()
        .with(Camera  { rotation, target })
        .build()
}