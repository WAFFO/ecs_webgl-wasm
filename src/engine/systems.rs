
use specs::{Read, ReadStorage, WriteStorage, System};

use engine::components::*;
use engine::resources::*;

// systems
pub struct UpdatePosition;

impl<'a> System<'a> for UpdatePosition {
    type SystemData = (Read<'a, DeltaTime>, WriteStorage<'a, Transform>, ReadStorage<'a, Velocity>);

    fn run(&mut self, (delta, mut pos, vel): Self::SystemData) {
        use specs::Join;

        // Read implements DeRef
        let delta = delta.0;

        for (pos, vel) in (&mut pos, &vel).join() {
            pos.translation[0] += vel.translation[0] * delta;
            pos.translation[1] += vel.translation[1] * delta;
            pos.translation[2] += vel.translation[2] * delta;
            pos.rotation[0] += vel.rotation[0] * delta;
            pos.rotation[1] += vel.rotation[1] * delta;
            pos.rotation[2] += vel.rotation[2] * delta;
        }
    }
}