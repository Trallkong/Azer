use crate::api::vulkan_helper;
use crate::render::render::{Render, RenderData};
use crate::render::vertex::{get_vbo_and_ibo_2d, Vertex2D};
use std::sync::Arc;
use vulkano::command_buffer::{AutoCommandBufferBuilder, PrimaryAutoCommandBuffer};
use vulkano::device::Device;
use vulkano::memory::allocator::StandardMemoryAllocator;
use vulkano::render_pass::RenderPass;
use winit::window::Window;

pub struct RenderRectangle {
    pub data: RenderData
}

impl Render for RenderRectangle {
    fn new(device: Arc<Device>, window: Arc<Window>, render_pass: Arc<RenderPass>, memory_allocator: Arc<StandardMemoryAllocator>) -> Self {
        let vertices = vec![
          Vertex2D { position: [-0.5, 0.5] },
          Vertex2D { position: [0.5, 0.5] },
          Vertex2D { position: [0.5, -0.5] },
          Vertex2D { position: [-0.5, -0.5] },
        ];

        let indices = vec![0, 1, 2, 2, 3, 0];

        let (vbo, ibo) = get_vbo_and_ibo_2d(
            vertices, indices, memory_allocator
        );

        let pipeline = vulkan_helper::get_graphics_pipeline(
            Arc::clone(&window),
            Arc::clone(&device),
            Arc::clone(&render_pass)
        );

        RenderRectangle {
            data: RenderData {
                graphics_pipeline: pipeline,
                vbo,
                ibo: Some(ibo),
                window,
                device,
                render_pass,
            }
        }
    }

    fn draw(&self, mut cmd_bf_builder: AutoCommandBufferBuilder<PrimaryAutoCommandBuffer>) -> AutoCommandBufferBuilder<PrimaryAutoCommandBuffer> {
        unsafe {
            cmd_bf_builder
                .bind_pipeline_graphics(Arc::clone(&self.data.graphics_pipeline))
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

    fn recreate_pipeline(&mut self) {
        self.data.graphics_pipeline = vulkan_helper::get_graphics_pipeline(
            Arc::clone(&self.data.window),
            Arc::clone(&self.data.device),
            Arc::clone(&self.data.render_pass)
        );
    }
}