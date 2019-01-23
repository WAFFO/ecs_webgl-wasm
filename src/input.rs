
use wasm_bindgen;
use specs::{Join};

use Engine;
use engine::components::Camera;


#[wasm_bindgen]
impl Engine {
    #[wasm_bindgen]
    pub fn mouse_move(&mut self, x: f32, y: f32) {
        let mut _camera_storage = self.world().write_storage::<Camera>();

        for camera in (&mut _camera_storage).join() {
            camera.position[0] += x * self.delta();
            camera.position[1] += y * self.delta();
        }
    }
}