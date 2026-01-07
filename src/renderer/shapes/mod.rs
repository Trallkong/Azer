use std::sync::Arc;

use vulkano::descriptor_set::DescriptorSet;

use crate::core::core::Scope;
use crate::renderer::shapes::mesh::{AzerVertex, Mesh};

pub mod transform;
pub mod shape_2d;
pub mod mesh;

pub trait Shape {
    fn mesh(&self) -> &Mesh;
}

pub struct GameObject {
    pub vertex_len: u32,
    pub index_count: u32,
    pub transform: [[f32; 4]; 4],
    pub set: Arc<DescriptorSet>,
}

pub struct DrawList {
    pub vertices: Vec<AzerVertex>,
    pub indices: Vec<u32>,
    pub objects: Vec<Scope<GameObject>>,
}

impl Default for DrawList {
    fn default() -> Self {
        Self {
            vertices: Vec::new(),
            indices: Vec::new(),
            objects: Vec::new(),
        }
    }
}

impl DrawList {
    pub fn clear(&mut self) {
        self.vertices.clear();
        self.indices.clear();
        self.objects.clear();
    }
}