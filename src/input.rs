
use wasm_bindgen;
use specs::{Join};

use Engine;
use engine::components::Camera;
use javascript;


#[wasm_bindgen]
impl Engine {
    #[wasm_bindgen]
    pub fn mouse_move(&mut self, x: f32, y: f32) {
        use std::f32::consts::PI;

        let mut _camera_storage = self.world().write_storage::<Camera>();

        for camera in (&mut _camera_storage).join() {
            camera.rotation[0] += x * self.delta();
            camera.rotation[1] += y * self.delta();

            if camera.rotation[1] > PI/2.0 - 0.1 {
                camera.rotation[1] = PI/2.0 - 0.1;
            }
            else if camera.rotation[1] < -PI/2.0 + 0.1 {
                camera.rotation[1] = -PI/2.0 + 0.1;
            }
        }
    }

    #[wasm_bindgen]
    pub fn key_down(&mut self,key: i32) {
        // probably clean input here
        if key > 255 {
            javascript::log_1("ERROR: key_down: {}", &key.into())
        }
        else {
            self.keys().press(key as usize);
        }
    }

    #[wasm_bindgen]
    pub fn key_up(&mut self,key: i32) {
        // probably clean input here
        if key > 255 {
            javascript::log_1("ERROR: key_up: {}", &key.into())
        }
        else {
            self.keys().release(key as usize);
        }
    }
}