use std::collections::HashMap;

use wasm_bindgen;
use specs::{Join};

use Engine;
use engine::components::Camera;
use javascript;


#[wasm_bindgen]
impl Engine {
    #[wasm_bindgen]
    pub fn mouse_click(&mut self, buttons: u32, x: f32, y: f32) {
        let left = (buttons & 1) > 0;
        let right = (buttons & 2) > 0;
        let middle = (buttons & 4) > 0;
        let m4 = (buttons & 8) > 0;
        let m5 = (buttons & 16) > 0;

        // TODO: handle mouse click
    }
    #[wasm_bindgen]
    pub fn mouse_move(&mut self, buttons: u32, x: f32, y: f32) {

        let left = (buttons & 1) > 0;

        if left {
            use std::f32::consts::PI;

            let mut _camera_storage = self.world().write_storage::<Camera>();

            for camera in (&mut _camera_storage).join() {
                camera.yaw -= x * self.delta();
                camera.pitch += y * self.delta();

                if camera.pitch > PI / 2.0 - 0.1 {
                    camera.pitch = PI / 2.0 - 0.1;
                } else if camera.pitch < -PI / 2.0 + 0.1 {
                    camera.pitch = -PI / 2.0 + 0.1;
                }
                camera.update();
            }
        }
    }
    #[wasm_bindgen]
    pub fn mouse_scroll(&mut self, s: f32) {
        let mut _camera_storage = self.world().write_storage::<Camera>();

        for camera in (&mut _camera_storage).join() {
            camera.pole_arm += s/s.abs();

            if camera.pole_arm < 0.1 {
                camera.pole_arm = 0.1;
            }

            camera.update();
        }
    }

    #[wasm_bindgen]
    pub fn key_down(&mut self,key: i32) {
        // probably clean input here
        if key > 255 || key < 0 {
            javascript::log_1("ERROR: key_down: {}", &key.into());
        }
        else {
            self.keys().press(key);
        }
    }

    #[wasm_bindgen]
    pub fn key_up(&mut self,key: i32) {
        // probably clean input here
        if key > 255 || key < 0 {
            javascript::log_1("ERROR: key_up: {}", &key.into());
        }
        else {
            self.keys().release(key);
        }
    }
}

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub enum Key {
    FORWARD,
    BACKWARD,
    LEFTWARD,
    RIGHTWARD,
}

pub struct KeyMap {
    map: HashMap<Key, usize>,
    board: [bool;256],
}

impl KeyMap {
    pub fn new() -> KeyMap {
        let map = KeyMap::default_key_mapping();
        let board = [false;256];

        KeyMap {
            map,
            board,
        }
    }

    pub fn default_key_mapping() -> HashMap<Key, usize> {
        let mut map = HashMap::new();

        map.insert(Key::FORWARD, 87);
        map.insert(Key::BACKWARD, 83);
        map.insert(Key::LEFTWARD, 65);
        map.insert(Key::RIGHTWARD, 68);

        map
    }

    pub fn val(&self, key: Key) -> usize{
        if let Some(v) = self.map.get(&key) { *v }
        else { 0 as usize }
    }

    pub fn get(&self, key: Key) -> bool {
        self.board[self.val(key)]
    }

    pub fn press(&mut self, key_val: i32) {
        self.board[key_val as usize] = true;
    }

    pub fn release(&mut self, key_val: i32) {
        self.board[key_val as usize] = false;
    }
}

impl Default for KeyMap {
    fn default() -> Self {
        Self::new()
    }
}