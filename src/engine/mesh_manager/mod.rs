
// mesh manager
// - mesh storage
// - mesh loader

mod mesh_storage;
mod mesh_loader;
pub mod mesh;

use self::mesh_storage::MeshStorage;
use self::mesh::{Mesh, MeshIndex};

// type must impl Clone
pub type UUID = String;

pub struct MeshManager {
    updated: bool,
    storage: MeshStorage,
}

impl MeshManager {

    pub fn new() -> MeshManager {
        MeshManager {
            updated: true,
            storage: MeshStorage::new(),
        }
    }

    pub fn load(&mut self, id: UUID) -> UUID {
        if let None = self.storage.get(&id) {
            if id == "debug_box" {
                self.storage.store(id.clone(),mesh_loader::load_debug_cube());
                self.updated = true;
            }
        }
        id
    }

    pub fn get_storage(&mut self) -> (&Vec<f32>, &Vec<f32>, &Vec<u16>) {
        self.updated = false;
        self.storage.get_storage()
    }

    pub fn get(&self, id: UUID) -> Option<MeshIndex> {
        self.storage.get(&id).cloned()
    }

    pub fn updated(&self) -> bool {
        self.updated
    }

}