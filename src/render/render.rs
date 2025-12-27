use crate::api::shaders::Shader;
use crate::render::renderer::RendererContext;
use crate::render::vertex::Vertex2D;
use glam::Mat4;
use std::sync::Arc;
use vulkano::buffer::Subbuffer;
use vulkano::command_buffer::{AutoCommandBufferBuilder, PrimaryAutoCommandBuffer};
use vulkano::descriptor_set::layout::DescriptorSetLayout;
use vulkano::device::Device;
use vulkano::pipeline::GraphicsPipeline;
use vulkano::render_pass::RenderPass;
use winit::window::Window;

pub struct RenderData {
    pub vbo: Subbuffer<[Vertex2D]>,
    pub ibo: Option<Subbuffer<[u32]>>,

    pub view_projection_matrix: Mat4,

    pub window: Arc<Window>,
    pub render_pass: Arc<RenderPass>,
    pub device: Arc<Device>,

    pub renderer_context: Arc<RendererContext>,
}

pub trait Render {
    fn new(
        device: Arc<Device>,
        window: Arc<Window>,
        render_pass: Arc<RenderPass>,
        renderer_context: Arc<RendererContext>
    ) -> Self;
    fn draw(&mut self, cmd_bf_builder: AutoCommandBufferBuilder<PrimaryAutoCommandBuffer>, pipeline: Arc<GraphicsPipeline>) -> AutoCommandBufferBuilder<PrimaryAutoCommandBuffer>;

    fn set_camera(&mut self, view_projection_matrix: Mat4);
}