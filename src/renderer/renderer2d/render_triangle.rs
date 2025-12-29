use crate::renderer::frame_commands::FrameCommands;
use crate::renderer::render_trait::{Render, RenderData};
use crate::renderer::renderer::Allocators;
use crate::renderer::vertex::{get_vbo_and_ibo_2d, Vertex2D};
use std::sync::Arc;
use vulkano::device::Device;
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
        allocators: &Allocators,
    ) -> Self {
        let vertices = vec![
            Vertex2D { position: [0.0, 0.5] },
            Vertex2D { position: [0.5, -0.5]},
            Vertex2D { position: [-0.5, -0.5]},
        ];

        let indices = vec![0, 1, 2];

        let (vbo, ibo) = get_vbo_and_ibo_2d(
            vertices,
            indices,
            allocators.buffer_allocator.clone()
        );

        RenderTriangle {
            data: RenderData {
                vbo,
                ibo: Some(ibo),
                window,
                device,
                render_pass,
            }
        }
    }

    fn draw(&mut self, frame: &mut FrameCommands, instance_index: usize) {
        unsafe {
            frame.builder
                .bind_vertex_buffers(0, self.data.vbo.clone())
                .unwrap()
                .bind_index_buffer(self.data.ibo.as_ref().unwrap().clone())
                .unwrap()
                .draw_indexed(3, 1, 0, 0, instance_index as u32)
                .unwrap();
        }
    }
}