use crate::renderer::shapes::mesh::{AzerVertex, Mesh};
use crate::renderer::shapes::Shape;

pub struct Triangle {
    mesh: Mesh
}


impl Triangle {
    pub fn new() -> Self {
        let vertices = vec![
            AzerVertex { position: [-0.5, 0.5, 0.0, 1.0], uv: [0.0, 0.0], color: [1.0, 1.0, 1.0, 1.0]},
            AzerVertex { position: [ 0.5, 0.5, 0.0, 1.0], uv: [1.0, 0.0], color: [1.0, 1.0, 1.0, 1.0]},
            AzerVertex { position: [ 0.0,-0.5, 0.0, 1.0], uv: [1.0, 1.0], color: [1.0, 1.0, 1.0, 1.0]},
        ];

        let indices = vec![0, 1, 2];

        Self {
            mesh: Mesh {
                vertices,
                indices
            }
        }
    }
}

impl Shape for Triangle {
    fn mesh(&self) -> &Mesh {
        &self.mesh
    }
}