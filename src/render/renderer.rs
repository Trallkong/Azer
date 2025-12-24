use crate::render::render::Render;
use crate::render::render_rectangle::RenderRectangle;
use crate::render::render_triangle::RenderTriangle;
use std::sync::Arc;
use vulkano::memory::allocator::StandardMemoryAllocator;
use vulkano::{
    command_buffer::allocator::{StandardCommandBufferAllocator, StandardCommandBufferAllocatorCreateInfo},
    command_buffer::{AutoCommandBufferBuilder, CommandBufferUsage, PrimaryAutoCommandBuffer, RenderPassBeginInfo, SubpassBeginInfo, SubpassContents, SubpassEndInfo},
    device::{Device, Queue},
    render_pass::{Framebuffer, RenderPass}
};
use winit::window::Window;

pub struct Renderer {
    cmd_bf_builder: Option<AutoCommandBufferBuilder<PrimaryAutoCommandBuffer>>,
    render_triangle: Box<RenderTriangle>,
    render_rectangle: Box<RenderRectangle>,

    allocator: Arc<StandardCommandBufferAllocator>,
    queue: Arc<Queue>,
}

impl Renderer {
    pub fn new(
        device: Arc<Device>,
        queue: Arc<Queue>,
        window: Arc<Window>,
        render_pass: Arc<RenderPass>,
    ) -> Self {

        let allocator = Arc::new(StandardCommandBufferAllocator::new(
            device.clone(),
            StandardCommandBufferAllocatorCreateInfo::default()
        ));

        let builder =
            AutoCommandBufferBuilder::primary(
                allocator.clone(),
                queue.queue_family_index(),
                CommandBufferUsage::MultipleSubmit,
            ).unwrap();


        let memory_allocator = Arc::new(StandardMemoryAllocator::new_default(Arc::clone(&device)));

        Self {
            cmd_bf_builder: Some(builder),
            render_triangle: Box::new(
                RenderTriangle::new(
                    Arc::clone(&device),
                    Arc::clone(&window),
                    Arc::clone(&render_pass),
                    memory_allocator.clone()
                ),
            ),
            render_rectangle: Box::new(
              RenderRectangle::new(
                  Arc::clone(&device),
                  Arc::clone(&window),
                  Arc::clone(&render_pass),
                  memory_allocator.clone()
              )
            ),
            allocator,
            queue,
        }
    }

    pub fn recreate_builder(&mut self) {
        self.cmd_bf_builder = Some(AutoCommandBufferBuilder::primary(
            self.allocator.clone(),
            self.queue.queue_family_index(),
            CommandBufferUsage::MultipleSubmit,
        ).unwrap());
    }

    pub fn begin(
        &mut self,
        framebuffer: Arc<Framebuffer>,
        clear_color: [f32; 4],
    ) {
        let mut builder = self.cmd_bf_builder.take().unwrap();

        builder
            .begin_render_pass(
                RenderPassBeginInfo {
                    clear_values: vec![Some(clear_color.into())],
                    ..RenderPassBeginInfo::framebuffer(framebuffer)
                },
                SubpassBeginInfo {
                    contents: SubpassContents::Inline,
                    ..SubpassBeginInfo::default()
                }
            )
            .unwrap()
        ;

        self.cmd_bf_builder = Some(builder);
    }

    pub fn end(
        &mut self,
    ) {
        let mut builder = self.cmd_bf_builder.take().unwrap();

        builder
            .end_render_pass(SubpassEndInfo::default())
            .unwrap();

        self.cmd_bf_builder = Some(builder);
    }

    pub fn submit(&mut self) -> Arc<PrimaryAutoCommandBuffer> {
        let builder = self.cmd_bf_builder.take().unwrap();
        let command_buffer = builder.build().unwrap();
        self.recreate_builder();
        command_buffer
    }

    pub fn draw_triangle(&mut self) {
        let builder = self.cmd_bf_builder.take().unwrap();
        self.cmd_bf_builder = Some(self.render_triangle.draw(builder));
    }

    pub fn draw_rectangle(&mut self) {
        let builder = self.cmd_bf_builder.take().unwrap();
        self.cmd_bf_builder = Some(self.render_rectangle.draw(builder));
    }

    pub fn recreate_pipeline(&mut self) {
        self.render_triangle.recreate_pipeline();
    }
}