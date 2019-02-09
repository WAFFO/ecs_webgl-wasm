#[derive(Debug)]
pub struct Mesh {
    pub vertices: Vec<f32>,
    pub colors: Vec<f32>,
    pub normals: Vec<f32>,
}

#[derive(Debug)]
pub struct MeshIndexed {
    pub vertices: Vec<f32>,
    pub colors: Vec<f32>,
    pub indices: Vec<u16>,
}

#[derive(Debug)]
pub struct MeshCompressed {
    pub vertices: Vec<f32>,
    pub colors: Vec<f32>,
    pub v_indices: Vec<u16>,
    pub c_indices: Vec<u16>,
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

impl Mesh {
    #[allow(non_snake_case)]
    pub fn set_normals(&mut self) {
        if self.vertices.len() % 9 != 0 {
            panic!("Mesh has {} vertices, this isn't a multiple of 9!",self.vertices.len());
        }
        if !self.normals.is_empty() {
            self.normals = Vec::new();
        }
        let mut i: usize = 0;
        while i < self.vertices.len() {
            let P = (
                i,
                i + 3,
                i + 6,
            );
            let U = (
                self.vertices[P.1  ] - self.vertices[P.0  ],
                self.vertices[P.1+1] - self.vertices[P.0+1],
                self.vertices[P.1+2] - self.vertices[P.0+2],
            );
            let V = (
                self.vertices[P.2  ] - self.vertices[P.0  ],
                self.vertices[P.2+1] - self.vertices[P.0+1],
                self.vertices[P.2+2] - self.vertices[P.0+2],
            );
            let N = (
                (U.1 * V.2) - (U.2 * V.1),
                (U.2 * V.0) - (U.0 * V.2),
                (U.0 * V.1) - (U.1 * V.0),
            );
            let len = (N.0*N.0 + N.1*N.1 + N.2*N.2).sqrt();
            for _ in 0..3 {
                self.normals.push(N.0 / len);
                self.normals.push(N.1 / len);
                self.normals.push(N.2 / len);
            }
            i += 9;
        }
    }
}