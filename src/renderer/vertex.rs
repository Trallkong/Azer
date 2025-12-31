use std::sync::Arc;
use vulkano::buffer::{Buffer, BufferContents, BufferCreateInfo, BufferUsage, Subbuffer};
use vulkano::memory::allocator::{AllocationCreateInfo, MemoryTypeFilter, StandardMemoryAllocator};
use vulkano::pipeline::graphics::vertex_input::Vertex;

#[derive(BufferContents, Vertex)]
#[repr(C)]
pub struct Vertex2D {
    #[format(R32G32_SFLOAT)]
    pub position: [f32; 2],
    #[format(R32G32_SFLOAT)]
    pub uv: [f32; 2],
}

pub fn get_vbo_2d(
    vertices: Vec<Vertex2D>,
    memory_allocator: Arc<StandardMemoryAllocator>,
) -> Subbuffer<[Vertex2D]> {
    let vertex_buffer = Buffer::from_iter(
        memory_allocator,
        BufferCreateInfo {
            usage: BufferUsage::VERTEX_BUFFER,
            ..BufferCreateInfo::default()
        },
        AllocationCreateInfo {
            memory_type_filter: MemoryTypeFilter::PREFER_DEVICE | MemoryTypeFilter::HOST_SEQUENTIAL_WRITE,
            ..AllocationCreateInfo::default()
        },
        vertices
    ).expect("Failed to create vertex buffer");

    vertex_buffer
}

pub fn get_vbo_from_size(image_size: (u32, u32), allocator: Arc<StandardMemoryAllocator>) -> Subbuffer<[Vertex2D]> {
    let (width, height) = image_size;
    let vertices = vec![
        Vertex2D { position: [0.0, 0.0], uv: [0.0, 0.0] },
        Vertex2D { position: [width as f32, 0.0], uv: [1.0, 0.0] },
        Vertex2D { position: [width as f32, height as f32], uv: [1.0, 1.0]},
        Vertex2D { position: [0.0, height as f32], uv: [0.0, 1.0]},
    ];
    get_vbo_2d(vertices, allocator.clone())
}

pub fn get_ibo_2d(
    indices: Vec<u32>,
    memory_allocator: Arc<StandardMemoryAllocator>,
) -> Subbuffer<[u32]> {
    Buffer::from_iter(
        memory_allocator.clone(),
        BufferCreateInfo {
            usage: BufferUsage::INDEX_BUFFER,
            ..BufferCreateInfo::default()
        },
        AllocationCreateInfo {
            memory_type_filter: MemoryTypeFilter::PREFER_DEVICE | MemoryTypeFilter::HOST_SEQUENTIAL_WRITE,
            ..AllocationCreateInfo::default()
        },
        indices,
    ).unwrap()
}

pub fn get_vbo_and_ibo_2d(
    vertices: Vec<Vertex2D>,
    indices: Vec<u32>,
    memory_allocator: Arc<StandardMemoryAllocator>,
) -> (Subbuffer<[Vertex2D]>, Subbuffer<[u32]>) {
    let vbo = Buffer::from_iter(
        memory_allocator.clone(),
        BufferCreateInfo {
            usage: BufferUsage::VERTEX_BUFFER,
            ..BufferCreateInfo::default()
        },
        AllocationCreateInfo {
            memory_type_filter: MemoryTypeFilter::PREFER_DEVICE | MemoryTypeFilter::HOST_SEQUENTIAL_WRITE,
            ..AllocationCreateInfo::default()
        },
        vertices
    );

    let ibo = Buffer::from_iter(
        memory_allocator,
        BufferCreateInfo {
            usage: BufferUsage::INDEX_BUFFER,
            ..BufferCreateInfo::default()
        },
        AllocationCreateInfo {
            memory_type_filter: MemoryTypeFilter::PREFER_DEVICE | MemoryTypeFilter::HOST_SEQUENTIAL_WRITE,
            ..AllocationCreateInfo::default()
        },
        indices
    );

    (vbo.expect("Failed to create vertex buffer"), ibo.expect("Failed to create index buffer"))
}