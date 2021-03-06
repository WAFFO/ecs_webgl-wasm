use wasm_bindgen::prelude::*;
use specs::{World, Entity, Join};

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
use input::KeyMap;
use input::Key::*;
use renderer::Renderer;
use timer::Timer;
use math::{Vert3,Vert4};


// Engine
#[wasm_bindgen]
pub struct Engine {
    world: World,
    entities: Vec<Entity>,
    renderer: Renderer,
    timer: Timer,
    mesh_manager: MeshManager,
    keys: KeyMap,
}

#[wasm_bindgen]
impl Engine {
    #[wasm_bindgen]
    pub fn tick(&mut self) -> Result<(), JsValue> {

        {
            // first tick delta time
            let mut _delta = self.world.write_resource::<DeltaTime>();
            _delta.0 = self.timer.tick_delta() as f32;
        }

        // input
        self.run_input();

        // do stuff here
        use specs::RunNow;
        let mut tick = UpdatePosition;
        tick.run_now(&self.world.res);

        // the last thing we do
        self.renderer.draw(&self.world, &mut self.mesh_manager)
    }
}

impl Engine {
    pub fn new() -> Result<(Engine), JsValue> {

        let world = Engine::build_world();

        let entities : Vec<Entity> = Vec::new();

        let renderer = Renderer::new()?;

        let timer = Timer::new();

        let mesh_manager = MeshManager::new();

        let keys = KeyMap::new();

        Ok (Engine {
            world,
            entities,
            renderer,
            timer,
            mesh_manager,
            keys,
        })
    }

    pub fn init(&mut self) {

        self.entities.push(
            test_light(
                &mut self.world,
                self.mesh_manager.load(String::from("debug_color_box")),
                Vert3::new(0.0, 0.0, 0.0),
                0.5,
                Vert3::new( 1.0, 0.0, -0.45 ),
            )
        );

        for i in -4..5 {
            for k in -4..5 {
                for l in -4..5 {
                    if i != 0 || k != 0 || l != 0 {
                        self.entities.push(
                            test_solid(
                                &mut self.world,
                                self.mesh_manager.load(String::from("debug_color_box")),
                                Vert3::new(6.0 * i as f32, 6.0 * k as f32, 6.0 * l as f32),
                                1.0,
                                Vert3::new(1.0, 0.0, -0.45),
                            )
                        );
                    }
                }
            }
        }

        use std::f32::consts::PI;
        self.entities.push(
            camera(
                &mut self.world,
                0.0,
                PI,
                Vert3::new( 0.0,0.0,0.0 ),
            )
        );

        self.world.maintain();

        self.mouse_move(1, 0.0,0.0);

    }

    fn build_world() -> World {
        let mut world = World::new();

        world.register::<Transform>();
        world.register::<Velocity>();
        world.register::<StaticMesh>();
        world.register::<Camera>();
        world.register::<Solid>();
        world.register::<Light>();

        world.add_resource(DeltaTime(0.0));

        world.maintain();

        world
    }

    pub fn world(&self) -> &World {
        &self.world
    }

    pub fn delta(&self) -> f32 {
        self.timer.get_delta() as f32
    }

    pub fn keys(&mut self) -> &mut KeyMap {
        &mut self.keys
    }

    pub fn run_input(&mut self) {

        let mut velocity : f32 = 5.0;

        if self.keys.get(SPRINT) {
            velocity *= 2.0;
        }

        let mut _camera_storage = self.world().write_storage::<Camera>();

        for camera in (&mut _camera_storage).join() {
            let forward : Vert3
                = camera.rotation.normalize();
            let right : Vert3
                = camera.rotation.cross(&Vert3::new(0.0, 1.0, 0.0)).normalize();

            if self.keys.get(FORWARD) {
                camera.target -= forward * self.delta() * velocity;
            }
            if self.keys.get(BACKWARD) {
                camera.target += forward * self.delta() * velocity;
            }
            if self.keys.get(LEFTWARD) {
                camera.target += right * self.delta() * velocity;
            }
            if self.keys.get(RIGHTWARD) {
                camera.target -= right * self.delta() * velocity;
            }
        }
    }
}