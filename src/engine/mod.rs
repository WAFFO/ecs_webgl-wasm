use wasm_bindgen::prelude::*;
use specs::{World, Entity};
use glm::vec3;

pub mod components;
pub mod entities;
pub mod resources;
pub mod systems;
pub mod mesh_manager;

use self::components::*;
use self::entities::*;
use self::resources::*;
use self::systems::*;
use self::mesh_manager::MeshManager;
use renderer::Renderer;
use timer::Timer;


// Engine
pub struct Engine {
    world: World,
    entities: Vec<Entity>,
    renderer: Renderer,
    timer: Timer,
    mesh_manager: MeshManager,
}

impl Engine {

    pub fn new() -> Result<(Engine), JsValue> {

        let world = Engine::build_world();

        let entities : Vec<Entity> = Vec::new();

        let renderer = Renderer::new()?;

        let timer = Timer::new();

        let mesh_manager = MeshManager::new();

        Ok (Engine {
            world,
            entities,
            renderer,
            timer,
            mesh_manager,
        })
    }

    pub fn init(&mut self) {

        self.entities.push(
            test_3d(&mut self.world, self.mesh_manager.load(String::from("debug_color_box")),
                    vec3( 0.0, 0.0, 0.0 ),
                    1.0,
                    vec3( 0.4, 0.4, 0.2 ),
            )
        );

        self.entities.push(
            test_3d(&mut self.world, self.mesh_manager.load(String::from("debug_color_box")),
                    vec3( -7.0, 0.0, -1.0 ),
                    1.0,
                    vec3( 0.0, 0.0, -0.45 ),
            )
        );

        self.entities.push(
            test_3d(&mut self.world, self.mesh_manager.load(String::from("debug_d20")),
                    vec3( -3.0, 0.0, -7.0 ),
                    2.0,
                    vec3( 1.0, 0.0, -0.45 ),
            )
        );

        self.entities.push(
            test_3d(&mut self.world, self.mesh_manager.load(String::from("debug_color_box")),
                    vec3( -5.0, 1.0, -7.0 ),
                    0.5,
                    vec3( 1.0, 0.0, -0.45 ),
            )
        );

        self.world.maintain();

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
        self.renderer.draw(&self.world, &mut self.mesh_manager)
    }

    fn build_world() -> World {
        let mut world = World::new();

        world.register::<Transform>();
        world.register::<Velocity>();
        world.register::<StaticMesh>();

        world.add_resource(DeltaTime(0.0));

        world.maintain();

        world
    }
}