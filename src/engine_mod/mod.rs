use wasm_bindgen::prelude::*;
use specs::{World, Builder, Entity};

pub mod components;
pub mod resources;
pub mod systems;

use self::components::*;
use self::resources::*;
use self::systems::*;
use render_mod::Renderer;
use timer_mod::Timer;


// Engine
pub struct Engine {
    world: World,
    entities: Vec<Entity>,
    renderer: Renderer,
    timer: Timer,
}

impl Engine {

    pub fn new() -> Result<(Engine), JsValue> {

        let timer = Timer::new();

        let renderer = Renderer::new()?;

        let mut entities : Vec<Entity> = Vec::new();

        let world = Engine::build_world(&mut entities);

        Ok (Engine {
            world,
            entities,
            renderer,
            timer,
        })
    }

    pub fn tick(&mut self) -> Result<(), JsValue> {

        {
            // first tick delta time
            let mut _delta = self.world.write_resource::<DeltaTime>();
            *_delta = DeltaTime(self.timer.tick_delta() as f32);
        }

        // do stuff here
        use specs::RunNow;
        let mut tick = UpdatePosition2D;
        tick.run_now(&self.world.res);

        // the last thing we do
        self.renderer.draw(&self.world)
    }

    fn build_world(entities: &mut Vec<Entity>) -> World {
        let mut world = World::new();

        world.register::<Transform2D>();
        world.register::<Velocity2D>();
        world.register::<TriangleMesh>();

        world.add_resource(DeltaTime(0.0));
        world.add_resource(AddVRotation(0.0));

        use std::f32::consts::PI;
        let r = ((2.0 * PI) / 3.0) as f32;

        entities.push(world.create_entity()
            .with(Transform2D  { position: [0.0, 0.0, 0.0], rotation: 0.0 })
            .with(Velocity2D   { position: [0.0, 0.0, 0.0], rotation: 1.0 })
            .with(TriangleMesh { vertices: [
                0.5, 0.0, 0.0,
                r.cos() * 0.5, r.sin() * 0.5, 0.0,
                (r * 2.0).cos() * 0.5, (r * 2.0).sin() * 0.5, 0.0
            ]})
            .build());

        world.maintain();

        world
    }
}