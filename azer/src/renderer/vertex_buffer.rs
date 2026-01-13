use wgpu::util::DeviceExt;

use crate::renderer::mesh;

pub struct VertexBuffer {
    buffer: wgpu::Buffer,
    num_vertices: u32,
}

impl VertexBuffer {

    pub fn new(device: &wgpu::Device, label: Option<String> , vertices: &[mesh::Vertex]) -> Self {
        let buffer = device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: label.as_deref(),
                contents: bytemuck::cast_slice(vertices),
                usage: wgpu::BufferUsages::VERTEX,
            }
        );

        Self { buffer, num_vertices: vertices.len() as u32 }
    }

    pub fn buffer(&self) -> &wgpu::Buffer {
        &self.buffer
    }

    pub fn num_vertices(&self) -> u32 {
        self.num_vertices
    }
}