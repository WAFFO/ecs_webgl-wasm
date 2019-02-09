
// mesh manager
// - mesh storage
// - mesh loader

mod mesh_storage;
mod mesh_loader;
pub mod mesh_exploder;
pub mod mesh;

use self::mesh_storage::MeshStorage;
use self::mesh::MeshIndex;
use self::mesh_exploder::Explodable;

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
        if self.storage.get(&id).is_none() {
            if id == "debug_box" {
                self.storage.store(id.clone(),&mesh_loader::load_debug_cube().explode());
                self.updated = true;
            }
            else if id == "debug_color_box" {
                self.storage.store(id.clone(),&mesh_loader::load_debug_color_cube().explode());
                self.updated = true;
            }
            else if id == "debug_d20" {
                self.storage.store(id.clone(),&mesh_loader::load_debug_d20().explode());
                self.updated = true;
            }
        }
        id
    }

    pub fn get_storage(&mut self) -> (&Vec<f32>, &Vec<f32>, &Vec<f32>) {
        self.updated = false;
        self.storage.get_storage()
    }

    pub fn get(&self, id: &UUID) -> Option<MeshIndex> {
        self.storage.get(&id).cloned()
    }

    pub fn updated(&self) -> bool {
        self.updated
    }

}

impl Default for MeshManager {
    fn default() -> Self {
        Self::new()
    }
}
