
use super::mesh::{Mesh, MeshIndexed, MeshCompressed};

pub trait Explodable {
    fn explode(self) -> Mesh;
}

impl Explodable for MeshIndexed {
    fn explode(self) -> Mesh {
        let mut mesh = Mesh {
            vertices: Vec::new(),
            colors: Vec::new(),
        };

        let mut count: usize = 0;
        for i in self.indices {
            mesh.vertices.push(self.vertices[( i * 3 + 0 ) as usize]);
            mesh.vertices.push(self.vertices[( i * 3 + 1 ) as usize]);
            mesh.vertices.push(self.vertices[( i * 3 + 2 ) as usize]);
            mesh.colors.push(self.colors[ count / 3 * 4 + 0 ]);
            mesh.colors.push(self.colors[ count / 3 * 4 + 1 ]);
            mesh.colors.push(self.colors[ count / 3 * 4 + 2 ]);
            mesh.colors.push(self.colors[ count / 3 * 4 + 3 ]);
            count += 1;
        };

        mesh
    }
}

impl Explodable for MeshCompressed {
    fn explode(self) -> Mesh {
        let mut mesh = Mesh {
            vertices: Vec::new(),
            colors: Vec::new(),
        };

        for i in self.v_indices {
            mesh.vertices.push(self.vertices[( i * 3 + 0 ) as usize]);
            mesh.vertices.push(self.vertices[( i * 3 + 1 ) as usize]);
            mesh.vertices.push(self.vertices[( i * 3 + 2 ) as usize]);
        };

        for i in self.c_indices {
            for _ in 0..3 {
                mesh.colors.push(self.colors[(i * 4 + 0) as usize]);
                mesh.colors.push(self.colors[(i * 4 + 1) as usize]);
                mesh.colors.push(self.colors[(i * 4 + 2) as usize]);
                mesh.colors.push(self.colors[(i * 4 + 3) as usize]);
            }
        };

        mesh
    }
}