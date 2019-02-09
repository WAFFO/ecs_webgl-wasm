
use std::collections::HashMap;

use super::UUID;
use super::mesh::{Mesh, MeshIndex};

pub struct MeshStorage {
    registry: HashMap<UUID, MeshIndex>,
    vertices: Vec<f32>,
    colors: Vec<f32>,
    normals: Vec<f32>,
}

impl MeshStorage {

    pub fn new() -> MeshStorage {
        MeshStorage {
            registry: HashMap::new(),
            vertices: Vec::new(),
            colors: Vec::new(),
            normals: Vec::new(),
        }
    }

    pub fn get(&self, id: &UUID) -> Option<&MeshIndex> {
        self.registry.get(id)
    }

    pub fn store(&mut self, id: UUID, mesh: &Mesh) {
        let mesh_index = MeshIndex {
            index: (self.vertices.len()/3) as i32,
            count: (mesh.vertices.len()/3) as i32,
        };

        // vertices
        self.vertices.extend(&mesh.vertices);
        // colors
        self.colors.extend(&mesh.colors);
        // normals
        self.normals.extend(&mesh.normals);

        // register on the hashmap
        self.registry.insert(id, mesh_index);
    }

    pub fn get_storage(&self) -> (&Vec<f32>, &Vec<f32>, &Vec<f32>) {
        (&self.vertices, &self.colors, &self.normals)
    }

    // TODO write a way to remove storage

}