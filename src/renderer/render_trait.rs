use crate::renderer::frame_commands::FrameCommands;
use crate::renderer::renderer::Allocators;
use crate::renderer::vertex::Vertex2D;
use std::sync::Arc;
use vulkano::buffer::Subbuffer;
use vulkano::device::Device;
use vulkano::render_pass::RenderPass;
use winit::window::Window;

pub struct RenderData {
    pub vbo: Subbuffer<[Vertex2D]>,
    pub ibo: Option<Subbuffer<[u32]>>,
    pub window: Arc<Window>,
    pub render_pass: Arc<RenderPass>,
    pub device: Arc<Device>,
}

pub trait Render {
    fn new(
        device: Arc<Device>,
        window: Arc<Window>,
        render_pass: Arc<RenderPass>,
        allocators: &Allocators,
    ) -> Self;
    fn draw(&mut self, frame: &mut FrameCommands, instance_index: usize);
}