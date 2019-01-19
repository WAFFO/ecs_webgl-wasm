
use super::mesh::{Mesh, MeshIndexed};

impl MeshIndexed {
    pub fn explode(self) -> Mesh {
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