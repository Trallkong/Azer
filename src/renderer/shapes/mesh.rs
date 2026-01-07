use crate::renderer::shapes::transform::Transform;
use glam::Vec4;
use vulkano::buffer::BufferContents;
use vulkano::pipeline::graphics::vertex_input::Vertex;

#[repr(C)]
#[derive(BufferContents, Vertex, Clone, Debug)]
pub struct AzerVertex {
    #[format(R32G32B32A32_SFLOAT)]
    pub position: [f32; 4],
    #[format(R32G32_SFLOAT)]
    pub uv: [f32; 2],
    #[format(R32G32B32A32_SFLOAT)]
    pub color: [f32; 4]
}


#[derive(Debug, Clone)]
pub struct Mesh {
    pub vertices: Vec<AzerVertex>,
    pub indices: Vec<u32>,
}

impl Mesh {
    pub fn get_transformed_mesh(&self, transform: &Transform) -> Mesh {
        let transmat = transform.to_mat4();
        let new_vertices: Vec<AzerVertex> = self.vertices.iter().map(|vertex| {
            let pos = transmat * Vec4::from_array(vertex.position);
            let mut new_vertex = vertex.clone();
            new_vertex.position = pos.to_array();
            new_vertex
        }).collect();

        Mesh {
            vertices: new_vertices,
            indices: self.indices.clone()
        }
    }

    pub fn set_color(&mut self, color: [f32; 4]) {
        self.vertices.iter_mut().for_each(|vertex| {
            vertex.color = color
        })
    }
}