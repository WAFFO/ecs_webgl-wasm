
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
        self.registry.get(id)
    }

    pub fn store(&mut self, id: UUID, mesh: Mesh) {
        let mesh_index = MeshIndex {
            offset: self.indices.len() as i32,
            size: mesh.indices.len() as i32,
        };
        let vertex_offset =
            if self.vertices.len() > 0 {
                (self.vertices.len() / 3) as u16
            } else {
                0
            };

        // vertices
        self.vertices.extend(&mesh.vertices);
        // colors
        if let Some(vec) = mesh.colors {
            self.colors.extend(&vec);
        }
        else {
            for _ in 0..mesh.indices.len() {
                self.colors.push(0.0);
            }
        }
        // indices
        for index in mesh.indices {
            self.indices.push(index + vertex_offset);
        }

        // register on the hashmap
        self.registry.insert(id, mesh_index);
    }

    pub fn get_storage(&self) -> (&Vec<f32>, &Vec<f32>, &Vec<u16>) {
        (&self.vertices, &self.colors, &self.indices)
    }

    // TODO write a way to remove storage

}