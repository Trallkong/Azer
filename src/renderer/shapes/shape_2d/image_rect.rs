use crate::renderer::shapes::mesh::{AzerVertex, Mesh};
use crate::renderer::shapes::Shape;
use glam::Vec2;

pub struct ImageRect {
    mesh: Mesh
}

impl ImageRect {
    pub fn new(size: Vec2) -> Self {
        let (w, h) = (size.x, size.y);
        let vertices = vec![
            AzerVertex { position: [-w/2.0, h/2.0, 0.0, 1.0], uv: [0.0, 0.0], color: [1.0, 1.0, 1.0, 1.0] },
            AzerVertex { position: [ w/2.0, h/2.0, 0.0, 1.0], uv: [1.0, 0.0], color: [1.0, 1.0, 1.0, 1.0] },
            AzerVertex { position: [ w/2.0,-h/2.0, 0.0, 1.0], uv: [1.0, 1.0], color: [1.0, 1.0, 1.0, 1.0] },
            AzerVertex { position: [-w/2.0,-h/2.0, 0.0, 1.0], uv: [0.0, 1.0], color: [1.0, 1.0, 1.0, 1.0] },
        ];
        let indices = vec![0,1,2,2,3,0];

        Self {
            mesh: Mesh {
                vertices,
                indices
            }
        }
    }
}

impl Shape for ImageRect {
    fn mesh(&self) -> &Mesh {
        &self.mesh
    }
}