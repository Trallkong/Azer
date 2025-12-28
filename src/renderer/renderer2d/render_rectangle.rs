use crate::renderer::render_trait::{Render, RenderData};
use crate::renderer::renderer::RendererContext;
use crate::renderer::vertex::{get_vbo_and_ibo_2d, Vertex2D};
use std::sync::Arc;
use vulkano::command_buffer::{AutoCommandBufferBuilder, PrimaryAutoCommandBuffer};
use vulkano::device::Device;
use vulkano::pipeline::{GraphicsPipeline, Pipeline, PipelineBindPoint};
use vulkano::render_pass::RenderPass;
use winit::window::Window;

pub struct RenderRectangle {
    pub data: RenderData
}

impl Render for RenderRectangle {
    fn new(
        device: Arc<Device>,
        window: Arc<Window>,
        render_pass: Arc<RenderPass>,
        renderer_context: Arc<RendererContext>
    ) -> Self {
        let vertices = vec![
          Vertex2D { position: [-0.5, 0.5] },
          Vertex2D { position: [0.5, 0.5] },
          Vertex2D { position: [0.5, -0.5] },
          Vertex2D { position: [-0.5, -0.5] },
        ];

        let indices = vec![0, 1, 2, 2, 3, 0];

        let (vbo, ibo) = get_vbo_and_ibo_2d(
            vertices, indices, renderer_context.buffer_allocator.clone()
        );

        RenderRectangle {
            data: RenderData {
                vbo,
                ibo: Some(ibo),
                window,
                device,
                render_pass,
                descriptor_set: renderer_context.cam_2d_uniform.set.clone(),
            }
        }
    }

    fn draw(&mut self, mut cmd_bf_builder: AutoCommandBufferBuilder<PrimaryAutoCommandBuffer>, pipeline: Arc<GraphicsPipeline>) -> AutoCommandBufferBuilder<PrimaryAutoCommandBuffer> {
        unsafe {
            cmd_bf_builder
                .bind_pipeline_graphics(pipeline.clone())
                .unwrap()
                .bind_descriptor_sets(
                    PipelineBindPoint::Graphics,
                    pipeline.layout().clone(),
                    0,
                    self.data.descriptor_set.clone(),
                )
                .unwrap()
                .bind_vertex_buffers(0, self.data.vbo.clone())
                .unwrap()
                .bind_index_buffer(self.data.ibo.as_ref().unwrap().clone())
                .unwrap()
                .draw_indexed(6, 1, 0, 0, 0)
                .unwrap();
        }

        cmd_bf_builder
    }
}