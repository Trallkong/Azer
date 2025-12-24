use std::sync::Arc;
use vulkano::buffer::Subbuffer;
use vulkano::command_buffer::{AutoCommandBufferBuilder, PrimaryAutoCommandBuffer};
use vulkano::device::Device;
use vulkano::memory::allocator::StandardMemoryAllocator;
use vulkano::pipeline::GraphicsPipeline;
use vulkano::render_pass::RenderPass;
use winit::window::Window;
use crate::render::vertex::Vertex2D;

pub struct RenderData {
    pub graphics_pipeline: Arc<GraphicsPipeline>,

    pub vbo: Subbuffer<[Vertex2D]>,
    pub ibo: Option<Subbuffer<[u32]>>,

    pub window: Arc<Window>,
    pub render_pass: Arc<RenderPass>,
    pub device: Arc<Device>
}

pub trait Render {
    fn new(device: Arc<Device>, window: Arc<Window>, render_pass: Arc<RenderPass>, memory_allocator: Arc<StandardMemoryAllocator>) -> Self;
    fn draw(&self, cmd_bf_builder: AutoCommandBufferBuilder<PrimaryAutoCommandBuffer>) -> AutoCommandBufferBuilder<PrimaryAutoCommandBuffer>;
    fn recreate_pipeline(&mut self);
}