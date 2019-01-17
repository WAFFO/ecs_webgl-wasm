
pub struct Mesh {
    pub vertices: Vec<f32>,
    pub colors: Option<Vec<f32>>,
    pub indices: Vec<u16>,
}

#[derive(Debug, Default, Copy, Clone)]
pub struct MeshIndex {
    pub offset: i32,
    pub size: i32,
}
