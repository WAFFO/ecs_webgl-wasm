//
// Where all the resources, components, and systems will be defined
//
// TODO: split into separate files: resources_mod.rs, components_mod.rs, systems_mod.rs

use specs::{Component, VecStorage, Read, ReadStorage, WriteStorage, System};

// resources
#[derive(Default)]
pub struct DeltaTime(f32);
#[derive(Default)]
pub struct AddVRotation(f32); // this is temp, TODO make this input

// components
#[derive(Default)]
pub struct Transform2D {
    position: [f32; 3],
    rotation: f32,
}

impl Component for Transform2D {
    type Storage = VecStorage<Self>;
}

#[derive(Default)]
pub struct Velocity2D {
    position: [f32; 3],
    rotation: f32,
}

impl Component for Velocity2D {
    type Storage = VecStorage<Self>;
}

#[derive(Default)]
pub struct TriangleMesh {
    vertices: [f32; 9],
}

impl Component for TriangleMesh {
    type Storage = VecStorage<Self>;
}



// systems
pub struct UpdatePosition2D;

impl<'a> System<'a> for UpdatePosition2D {
    type SystemData = (Read<'a, DeltaTime>, WriteStorage<'a, Transform2D>, ReadStorage<'a, Velocity2D>);

    fn run(&mut self, (delta, mut pos, vel): Self::SystemData) {
        use specs::Join;

        // Read implements DeRef
        let delta = delta.0;

        for (pos, vel) in (&mut pos, &vel).join() {
            pos.position[0] += vel.position[0] * delta;
            pos.position[1] += vel.position[1] * delta;
            pos.rotation += vel.rotation * delta;
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