#[derive(Debug)]
pub struct Mesh {
    pub vertices: Vec<f32>,
    pub colors: Vec<f32>,
}

#[derive(Debug)]
pub struct MeshIndexed {
    pub vertices: Vec<f32>,
    pub colors: Vec<f32>,
    pub indices: Vec<u16>,
}

#[derive(Debug, Default, Copy, Clone)]
pub struct MeshIndex {
    pub index: i32,
    pub count: i32,
}

impl PartialEq for Mesh {
    fn eq(&self, other: &Mesh) -> bool {
        self.vertices == other.vertices
        &&
        self.colors == other.colors
    }
}