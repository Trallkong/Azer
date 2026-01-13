use wgpu::util::DeviceExt;

pub struct IndexBuffer {
    buffer: wgpu::Buffer,
    num_indices: u32,
}

impl IndexBuffer {
    pub fn new(device: &wgpu::Device, label: Option<String> , indices: &[u16]) -> Self {
        let buffer = device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: label.as_deref(),
                contents: bytemuck::cast_slice(indices),
                usage: wgpu::BufferUsages::INDEX,
            }
        );

        Self { buffer, num_indices: indices.len() as u32 }
    }

    pub fn buffer(&self) -> &wgpu::Buffer {
        &self.buffer
    }

    pub fn num_indices(&self) -> u32 {
        self.num_indices
    }
}