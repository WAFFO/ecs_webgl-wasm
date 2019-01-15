
// mesh manager
// - mesh storage
// - mesh loader

mod mesh_storage;
mod mesh_loader;
pub mod mesh;

use self::mesh_storage::MeshStorage;
use self::mesh_loader;
use self::mesh::{Mesh, MeshIndex};

type UUID = String;

pub struct MeshManager {
    updated: bool,
    storage: MeshStorage,
}

impl MeshManager {
    pub fn new() -> MeshManager {
        MeshManager {
            updated: true,
            storage: mesh_storage::new(),
        }
    }

    pub fn load(&mut self, id: UUID) -> bool {
        if let Some(index) = self.storage.get(&id) {
            true
        }
        else {
            if id == "debug_box" {
                self.storage.store(id,mesh_loader::load_debug_cube());
                self.updated = true;
                true
            }
            false
        }
    }
}