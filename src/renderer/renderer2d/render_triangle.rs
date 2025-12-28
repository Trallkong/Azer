use crate::renderer::frame_commands::FrameCommands;
use crate::renderer::render_trait::{Render, RenderData};
use crate::renderer::renderer::RendererContext;
use crate::renderer::vertex::{get_vbo_2d, Vertex2D};
use std::sync::Arc;
use vulkano::device::Device;
use vulkano::pipeline::GraphicsPipeline;
use vulkano::render_pass::RenderPass;
use winit::window::Window;

pub struct RenderTriangle {
    pub data: RenderData,
}

impl Render for RenderTriangle {
    fn new(
        device: Arc<Device>,
        window: Arc<Window>,
        render_pass: Arc<RenderPass>,
        renderer_context: Arc<RendererContext>
    ) -> Self {
        let vertices = vec![
            Vertex2D { position: [0.0, 0.5] },
            Vertex2D { position: [0.5, -0.5]},
            Vertex2D { position: [-0.5, -0.5]},
        ];

        let vbo = get_vbo_2d(
            vertices,
            renderer_context.buffer_allocator.clone()
        );

        RenderTriangle {
            data: RenderData {
                vbo,
                ibo: None,
                window,
                device,
                render_pass,
                descriptor_set: renderer_context.cam_2d_uniform.set.clone(),
            }
        }
    }

    fn draw(&mut self, frame: &mut FrameCommands, pipeline: Arc<GraphicsPipeline>) {
        unsafe {
            &mut frame.builder
                .bind_pipeline_graphics(pipeline.clone())
                .unwrap()
                .bind_vertex_buffers(0, self.data.vbo.clone())
                .unwrap()
                .draw(3, 1, 0, 0)
                .unwrap();
        }
    }
}