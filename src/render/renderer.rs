use crate::api::vulkan_helper;
use crate::render::render::Render;
use crate::render::render_rectangle::RenderRectangle;
use crate::render::render_triangle::RenderTriangle;
use glam::Mat4;
use std::sync::{Arc, Mutex};
use vulkano::descriptor_set::allocator::StandardDescriptorSetAllocator;
use vulkano::descriptor_set::layout::DescriptorSetLayout;
use vulkano::memory::allocator::StandardMemoryAllocator;
use vulkano::{
    command_buffer::allocator::{StandardCommandBufferAllocator, StandardCommandBufferAllocatorCreateInfo},
    command_buffer::{AutoCommandBufferBuilder, CommandBufferUsage, PrimaryAutoCommandBuffer, RenderPassBeginInfo, SubpassBeginInfo, SubpassContents, SubpassEndInfo},
    device::{Device, Queue},
    render_pass::{Framebuffer, RenderPass}
};
use vulkano::buffer::Subbuffer;
use vulkano::descriptor_set::DescriptorSet;
use vulkano::pipeline::{GraphicsPipeline, Pipeline};
use winit::window::Window;
use crate::api::shaders::sd_camera2d::Data;
use crate::api::shaders::Shader;

pub struct RendererContext {
    pub buffer_allocator: Arc<StandardMemoryAllocator>,
    pub descriptor_set_allocator: Arc<StandardDescriptorSetAllocator>,
    pub camera_set_layout: Arc<DescriptorSetLayout>,

    pub uniform_buffer: Subbuffer<Data>,
    pub descriptor_set: Arc<DescriptorSet>,
}

pub struct Renderer {
    cmd_bf_builder: Option<AutoCommandBufferBuilder<PrimaryAutoCommandBuffer>>,
    render_triangle: Box<RenderTriangle>,
    render_rectangle: Box<RenderRectangle>,

    allocator: Arc<StandardCommandBufferAllocator>,
    queue: Arc<Queue>,

    renderer_context: Arc<RendererContext>,

    window: Arc<Window>,
    device: Arc<Device>,
    render_pass: Arc<RenderPass>,
    shader: Arc<dyn Shader>,
    pipeline: Arc<GraphicsPipeline>,
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


        let buffer_allocator = Arc::new(StandardMemoryAllocator::new_default(Arc::clone(&device)));
        let descriptor_set_allocator = vulkan_helper::get_descriptor_set_allocator(device.clone());

        let shader = crate::api::shaders::sd_camera2d::ShaderCamera2D::load(Arc::clone(&device)).unwrap();
        let shader = Arc::new(shader);

        let pipeline = vulkan_helper::get_graphics_pipeline(
            Arc::clone(&window),
            Arc::clone(&device),
            Arc::clone(&render_pass),
            shader.clone()
        );

        let camera_set_layout = pipeline.layout().set_layouts().get(0).unwrap().clone();

        let data = Data {
            mvp: Mat4::IDENTITY.to_cols_array_2d(),
            color: [1.0,1.0,1.0,1.0]
        };

        let uniform_buffer = vulkan_helper::get_uniform_buffer(
            data,
            Arc::clone(&buffer_allocator),
        );

        let descriptor_set = vulkan_helper::get_descriptor_set(
            uniform_buffer.clone(),
            0,
            camera_set_layout.clone(),
            descriptor_set_allocator.clone(),
        );

        let renderer_context = RendererContext {
            buffer_allocator,
            descriptor_set_allocator,
            camera_set_layout,
            uniform_buffer,
            descriptor_set,
        };
        let renderer_context = Arc::new(renderer_context);

        Self {
            cmd_bf_builder: Some(builder),
            render_triangle: Box::new(
                RenderTriangle::new(
                    Arc::clone(&device),
                    Arc::clone(&window),
                    Arc::clone(&render_pass),
                    renderer_context.clone()
                ),
            ),
            render_rectangle: Box::new(
              RenderRectangle::new(
                  Arc::clone(&device),
                  Arc::clone(&window),
                  Arc::clone(&render_pass),
                  renderer_context.clone()
              )
            ),
            allocator,
            queue,
            renderer_context,
            window,
            device,
            render_pass,
            shader,
            pipeline,
        }
    }

    pub fn recreate_builder(&mut self) {
        self.cmd_bf_builder = Some(AutoCommandBufferBuilder::primary(
            self.allocator.clone(),
            self.queue.queue_family_index(),
            CommandBufferUsage::MultipleSubmit,
        ).unwrap());
    }

    pub fn set_camera(&mut self, view_projection_matrix: Mat4) {
        self.render_triangle.set_camera(view_projection_matrix);
        self.render_rectangle.set_camera(view_projection_matrix);
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
        self.cmd_bf_builder = Some(self.render_triangle.draw(builder, self.pipeline.clone()));
    }

    pub fn draw_rectangle(&mut self) {
        let builder = self.cmd_bf_builder.take().unwrap();
        self.cmd_bf_builder = Some(self.render_rectangle.draw(builder,  self.pipeline.clone()));
    }

    pub fn recreate_pipeline(&mut self) {
        self.pipeline = vulkan_helper::get_graphics_pipeline(
            self.window.clone(),
            self.device.clone(),
            self.render_pass.clone(),
            self.shader.clone()
        )
    }
}