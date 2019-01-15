
use std::collections::HashMap;

use super::UUID;
use super::mesh::{Mesh, MeshIndex};

pub struct MeshStorage {
    registry: HashMap<UUID, MeshIndex>,
    vertices: Vec<f32>,
    colors: Vec<f32>,
    indices: Vec<u16>,
}

impl MeshStorage {
    pub fn new() -> MeshStorage {
        MeshStorage {
            registry: HashMap::new(),
            vertices: Vec::new(),
            colors: Vec::new(),
            indices: Vec::new(),
        }
    }
    pub fn get(&self, id: &UUID) -> Option<&MeshIndex> {
        self.registry.get(&id)
    }
    pub fn store(&mut self, id: UUID, mesh: Mesh) {
        let index = MeshIndex {
            offset: self.vertices.len() as u32,
            size: mesh.indices.len() as u32,
        };
        // vertices
        self.vertices.extend(&mesh.vertices);
        // colors
        if let Some(vec) = mesh.colors {
            self.colors.extend(&vec);
        }
        else {
            for i in 0..mesh.indices.len() {
                self.colors.push(0.0);
            }
        }
        // indices
        self.indices.extend(&mesh.indices);

        // register on the hashmap
        self.registry.insert(id, index);
    }
}