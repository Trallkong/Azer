use crate::api::shaders::Shader;
use crate::api::vulkan_helper;
use crate::render::camera::Camera;
use crate::render::render::{Render, RenderData};
use crate::render::renderer::RendererContext;
use crate::render::vertex::{get_vbo_and_ibo_2d, Vertex2D};
use std::sync::Arc;
use glam::Mat4;
use log::info;
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
                view_projection_matrix: Mat4::IDENTITY,
                renderer_context,
            }
        }
    }

    fn draw(&mut self, mut cmd_bf_builder: AutoCommandBufferBuilder<PrimaryAutoCommandBuffer>, pipeline: Arc<GraphicsPipeline>) -> AutoCommandBufferBuilder<PrimaryAutoCommandBuffer> {

        let view_proj_matrix = self.data.view_projection_matrix;

        let uniform_data = crate::api::shaders::sd_camera2d::Data {
            mvp: view_proj_matrix.to_cols_array_2d(),
            color: [1.0, 0.0, 0.0, 1.0],
        };

        let mut content = self.data.renderer_context.uniform_buffer.write().unwrap();
        *content = uniform_data;

        unsafe {
            cmd_bf_builder
                .bind_pipeline_graphics(pipeline.clone())
                .unwrap()
                .bind_descriptor_sets(
                    PipelineBindPoint::Graphics,
                    pipeline.layout().clone(),
                    0,
                    self.data.renderer_context.descriptor_set.clone(),
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

    fn set_camera(&mut self, view_projection_matrix: Mat4) {
        self.data.view_projection_matrix = view_projection_matrix;
    }
}