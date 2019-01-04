use wasm_bindgen::prelude::*;
use specs::{World, Entity};

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

        for i in 0..9 {
            entities.push(triangle(&mut world,
                                   (i as f32%3.0-1.0)*0.5,
                                   ((i/3) as f32%3.0-1.0)*0.5,
                                   0.25,
                                   (0.0, 0.0, if i==4{-2.0}else{1.0})));
        }

        world.maintain();

        world
    }
}