
// mesh manager
// - mesh storage
// - mesh loader

mod mesh_storage;
mod mesh_loader;
pub mod mesh;

use mesh_storage;
use mesh_loader;
use mesh::{Mesh, MeshIndex};

type UUID = String;

pub struct MeshManager {
    updated: bool,
    storage: mesh_storage,
}

impl MeshManager {
    pub fn new() -> MeshManager {
        MeshManager {
            updated: true,
            storage: mesh_storage::new(),
        }
    }

    pub fn load(&self, id: UUID) -> bool {
        if let Some(index) = self.storage.get(id) {
            true
        }
        else {
            if id == "debug_box" {
                mesh_loader::load_debug_cube();
            }
            false
        }
    }
}