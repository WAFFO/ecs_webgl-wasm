use wasm_bindgen::prelude::*;
use specs::{World, Builder, Entity};

pub mod rcs_mod;

use self::rcs_mod::*;
use render_mod::Renderer;
use timer_mod::Timer;


// Engine
#[wasm_bindgen]
pub struct Engine<'a> {
    world: World,
    entities: Vec<Entity>,
    renderer: Renderer,
    timer: Timer,
}

#[wasm_bindgen]
impl Engine {

    #[wasm_bindgen]
    pub fn new() -> Result<(Engine), JsValue> {

        let timer = Timer::new();

        let renderer = Renderer::new()?;

        let mut world = World::new();

        world.register::<Transform2D>();
        world.register::<Velocity2D>();
        world.register::<TriangleMesh>();

        world.add_resource(DeltaTime(0.0));
        world.add_resource(AddVRotation(0.0));

        use std::f32::consts::PI;
        let r = ((2.0 * PI) / 3.0) as f32;

        let mut entities: Vec<Entity> = Vec::new();

        entities.push(world.create_entity()
            .with(Transform2D  { position: [0.0, 0.0, 0.0], rotation: 0.0 })
            .with(Velocity2D   { position: [0.0, 0.0, 0.0], rotation: 1.0 })
            .with(TriangleMesh { vertices: [
                    0.7, 0.0, 0.0,
                    r.cos() * 0.7, r.sin() * 0.7, 0.0,
                    (r * 2.0).cos() * 0.7, (r * 2.0).sin() * 0.7, 0.0
                ]})
            .build());

        world.maintain();

        Ok (Engine {
            world,
            entities,
            renderer,
            timer,
        })
    }

    #[wasm_bindgen]
    pub fn tick(&mut self) -> Result<(), JsValue> {
        // first tick delta time
        let delta =self.timer.tick_delta();

        // do stuff here
        //self.triangle.rotate(delta as f32);

        // the last thing we do
        self.renderer.draw(&self.triangle)
    }
}