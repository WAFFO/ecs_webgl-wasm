//
// Where all the resources, components, and systems will be defined
//
// TODO: split into separate files: resources_mod.rs, components_mod.rs, systems_mod.rs

use specs::{Component, VecStorage, Read, ReadStorage, WriteStorage, System};

// resources
#[derive(Default)]
pub struct DeltaTime(pub f32);
#[derive(Default)]
pub struct AddVRotation(pub f32); // this is temp, TODO make this input

// components
#[derive(Default)]
pub struct Transform2D {
    pub position: [f32; 3],
    pub rotation: f32,
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