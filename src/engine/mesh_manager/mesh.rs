
pub struct Mesh {
    pub vertices: Vec<f32>,
    pub colors: Option<Vec<f32>>,
    pub indices: Vec<u16>,
}

#[derive(Defualt, Copy, Clone)]
pub struct MeshIndex {
    pub offset: u32,
    pub size: u32,
}

pub fn new