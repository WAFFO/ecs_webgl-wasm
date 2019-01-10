
use specs::{Read, ReadStorage, WriteStorage, System};

use engine_mod::components::*;
use engine_mod::resources::*;

// systems
pub struct UpdatePosition;

impl<'a> System<'a> for UpdatePosition {
    type SystemData = (Read<'a, DeltaTime>, WriteStorage<'a, Transform>, ReadStorage<'a, Velocity>);

    fn run(&mut self, (delta, mut pos, vel): Self::SystemData) {
        use specs::Join;

        // Read implements DeRef
        let delta = delta.0;

        for (pos, vel) in (&mut pos, &vel).join() {
            pos.translation += vel.translation * delta;
            pos.rotation += vel.rotation * delta;
        }
    }
}


// !! probably a cancelled idea !! //

pub struct RenderSystem;

impl<'a> System<'a> for RenderSystem {
    type SystemData = (ReadStorage<'a, Transform>, WriteStorage<'a, Render>);

    fn run(&mut self, (pos, mut ren): Self::SystemData) {
        use specs::Join;

        for (pos, ren) in (&pos, &mut ren).join() {
            // TODO fill this out
        }
    }
}