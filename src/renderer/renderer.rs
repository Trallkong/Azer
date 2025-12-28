use crate::api::shaders::sd_camera2d::Cam2dShaderData;
use crate::api::shaders::Shader;
use crate::api::vulkan_helper;
use crate::renderer::camera::Camera2dUniform;
use crate::renderer::frame_commands::FrameCommands;
use crate::renderer::render_trait::Render;
use crate::renderer::renderer2d::render_rectangle::RenderRectangle;
use crate::renderer::renderer2d::render_triangle::RenderTriangle;
use glam::Mat4;
use std::sync::Arc;
use vulkano::descriptor_set::allocator::StandardDescriptorSetAllocator;
use vulkano::descriptor_set::layout::DescriptorSetLayout;
use vulkano::memory::allocator::StandardMemoryAllocator;
use vulkano::pipeline::{GraphicsPipeline, Pipeline, PipelineBindPoint};
use vulkano::{
    command_buffer::allocator::{StandardCommandBufferAllocator, StandardCommandBufferAllocatorCreateInfo},
    command_buffer::{RenderPassBeginInfo, SubpassBeginInfo, SubpassContents, SubpassEndInfo},
    device::{Device, Queue},
    render_pass::{Framebuffer, RenderPass}
};
use winit::window::Window;

#[derive(Clone)]
pub struct RendererContext {
    pub buffer_allocator: Arc<StandardMemoryAllocator>,
    pub descriptor_set_allocator: Arc<StandardDescriptorSetAllocator>,
    pub cam_2d_uniform: Arc<Camera2dUniform>,
}

pub struct Renderer {
    render_triangle: Box<RenderTriangle>,
    render_rectangle: Box<RenderRectangle>,

    pub allocator: Arc<StandardCommandBufferAllocator>,
    queue: Arc<Queue>,

    renderer_context: Arc<RendererContext>,

    window: Arc<Window>,
    device: Arc<Device>,
    render_pass: Arc<RenderPass>,
    shader: Arc<dyn Shader>,
    pipeline: Arc<GraphicsPipeline>,
    descriptor_set_layout: Arc<DescriptorSetLayout>,
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

        let set_layout = pipeline.layout().set_layouts().get(0).unwrap().clone();

        let data = Cam2dShaderData::default();

        let uniform_buffer = vulkan_helper::get_uniform_buffer(
            data,
            Arc::clone(&buffer_allocator),
        );

        let descriptor_set = vulkan_helper::get_descriptor_set(
            uniform_buffer.clone(),
            0,
            set_layout.clone(),
            descriptor_set_allocator.clone(),
        );

        let renderer_context = RendererContext {
            buffer_allocator,
            descriptor_set_allocator,
            cam_2d_uniform: Arc::new(Camera2dUniform {
                buffer: uniform_buffer,
                set: descriptor_set,
            })
        };

        let renderer_context = Arc::new(renderer_context);

        Self {
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
            descriptor_set_layout: set_layout,
        }
    }

    pub fn update_camera(&mut self, view_projection_matrix: Mat4) {
        self.renderer_context.cam_2d_uniform.update(view_projection_matrix);
    }

    pub fn begin(&mut self, frame: &mut FrameCommands, framebuffer: Arc<Framebuffer>, clear_color: [f32; 4], ) {
        frame.builder
            .begin_render_pass(
                RenderPassBeginInfo {
                    clear_values: vec![Some(clear_color.into())],
                    ..RenderPassBeginInfo::framebuffer(framebuffer)
                },
                SubpassBeginInfo {
                    contents: SubpassContents::Inline,
                    ..SubpassBeginInfo::default()
                }
            ).unwrap()
            .bind_descriptor_sets(
                PipelineBindPoint::Graphics,
                self.pipeline.layout().clone(),
                0,
                self.renderer_context.cam_2d_uniform.set.clone(),
            ).unwrap();
    }

    pub fn end(&mut self, frame: &mut FrameCommands) {
        frame.builder
            .end_render_pass(SubpassEndInfo::default())
            .unwrap();
    }

    pub fn draw_triangle(&mut self, frame: &mut FrameCommands) {
        self.render_triangle.draw(frame, self.pipeline.clone())
    }

    pub fn draw_rectangle(&mut self, frame: &mut FrameCommands) {
        self.render_rectangle.draw(frame, self.pipeline.clone())
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