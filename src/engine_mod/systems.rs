
use specs::{Read, ReadStorage, WriteStorage, System};

use engine_mod::components::*;
use engine_mod::resources::*;

// systems
pub struct UpdatePosition2D;

impl<'a> System<'a> for UpdatePosition2D {
    type SystemData = (Read<'a, DeltaTime>, WriteStorage<'a, Transform2D>, ReadStorage<'a, Velocity2D>,
                       WriteStorage<'a, TriangleMesh>);

    fn run(&mut self, (delta, mut pos, vel, mut tri): Self::SystemData) {
        use specs::Join;

        // Read implements DeRef
        let delta = delta.0;

        for (pos, vel, tri) in (&mut pos, &vel, &mut tri).join() {
            pos.position[0] += vel.position[0] * delta;
            pos.position[1] += vel.position[1] * delta;
            pos.rotation += vel.rotation * delta;

            use std::f32::consts::PI;
            let a = pos.rotation;
            let r = ((2.0 * PI) / 3.0) as f32;
            // return the vertices of our triangle
            (*tri).vertices = [
                a.cos() * 0.5, a.sin() * 0.5, 0.0,
                (a + r).cos() * 0.5, (a + r).sin() * 0.5, 0.0,
                (a + r * 2.0).cos() * 0.5, (a + r * 2.0).sin() * 0.5, 0.0
            ];
        }
    }
}

// temp
pub struct AddRotation2D;

impl<'a> System<'a> for AddRotation2D {
    type SystemData = (Read<'a, AddVRotation>, WriteStorage<'a, Velocity2D>);

    fn run(&mut self, (rot, mut vel): Self::SystemData) {
        use specs::Join;

        // Read implements DeRef
        let rot = rot.0;

        for v in (&mut vel).join() {
            v.rotation += rot;
        }
    }
}