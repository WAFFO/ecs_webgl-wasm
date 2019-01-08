use wasm_bindgen::prelude::*;
use specs::{World, Entity};
use glm::{Vec3, vec3};

pub mod components;
pub mod entities;
pub mod resources;
pub mod systems;

use self::components::*;
use self::entities::*;
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
            _delta.0 = self.timer.tick_delta() as f32;
        }

        // do stuff here
        use specs::RunNow;
        let mut tick = UpdatePosition;
        tick.run_now(&self.world.res);

        // the last thing we do
        self.renderer.draw(&self.world)
    }

    fn build_world(entities: &mut Vec<Entity>) -> World {
        let mut world = World::new();

        world.register::<Transform>();
        world.register::<Velocity>();
        world.register::<StaticMesh>();
        world.register::<StaticColorMesh>();

        world.add_resource(DeltaTime(0.0));

        entities.push(
            test_3d(&mut world,
                vec3( 0.0, 0.0, 0.0 ),
                1.0,
                vec3( 0.4, 0.4, 0.2 ),
            )
        );

        world.maintain();

        world
    }
}