use vulkano::descriptor_set::DescriptorSet;
use crate::renderer::shaders::Shader;
use crate::api::vulkan_helper;
use crate::core::layer_stack::LayerStack;
use crate::renderer::frame_commands::FrameCommands;
use crate::renderer::render_trait::Render;
use crate::renderer::renderer2d::render_rectangle::RenderRectangle;
use crate::renderer::renderer2d::render_triangle::RenderTriangle;
use crate::renderer::shapes::transform::Transform2D;
use glam::Mat4;
use std::sync::Arc;
use log::error;
use vulkano::command_buffer::PrimaryAutoCommandBuffer;
use vulkano::descriptor_set::allocator::StandardDescriptorSetAllocator;
use vulkano::memory::allocator::{AllocationCreateInfo, MemoryTypeFilter, StandardMemoryAllocator};
use vulkano::pipeline::{GraphicsPipeline, Pipeline, PipelineBindPoint};
use vulkano::{
    command_buffer::allocator::{StandardCommandBufferAllocator, StandardCommandBufferAllocatorCreateInfo},
    command_buffer::{RenderPassBeginInfo, SubpassBeginInfo, SubpassContents, SubpassEndInfo},
    device::{Device, Queue},
    render_pass::{Framebuffer, RenderPass}
};
use vulkano::buffer::{Buffer, BufferCreateInfo, BufferUsage};
use vulkano::descriptor_set::WriteDescriptorSet;
use winit::window::Window;
use crate::renderer::shaders::batch_render_shader::{Instance, Instances, ShaderData};

pub struct Allocators {
    pub buffer_allocator: Arc<StandardMemoryAllocator>,
    pub descriptor_set_allocator: Arc<StandardDescriptorSetAllocator>,
    pub command_buffer_allocator: Arc<StandardCommandBufferAllocator>,
}

pub struct RendererContext {
    pub instance_index: usize,
}

pub struct Renderer {
    render_triangle: Box<RenderTriangle>,
    render_rectangle: Box<RenderRectangle>,
    queue: Arc<Queue>,
    window: Arc<Window>,
    device: Arc<Device>,
    render_pass: Arc<RenderPass>,
    shader: Arc<dyn Shader>,
    pipeline: Arc<GraphicsPipeline>,
    frame_commands: Option<FrameCommands>,
    allocators: Allocators,
    descriptor_set: Arc<DescriptorSet>,
    shader_data: ShaderData,
    renderer_context: RendererContext
}

impl Renderer {
    pub fn new(
        device: Arc<Device>,
        queue: Arc<Queue>,
        window: Arc<Window>,
        render_pass: Arc<RenderPass>,
    ) -> Self {

        let command_buffer_allocator = Arc::new(StandardCommandBufferAllocator::new(
            device.clone(),
            StandardCommandBufferAllocatorCreateInfo::default()
        ));

        let buffer_allocator = Arc::new(StandardMemoryAllocator::new_default(Arc::clone(&device)));
        let descriptor_set_allocator = vulkan_helper::get_descriptor_set_allocator(device.clone());

        // 准备着色器
        let shader = crate::renderer::shaders::batch_render_shader::BatchRenderShader::load(device.clone())
            .unwrap_or_else(|e| {
                error!("着色器创建失败: {}", e);
                panic!("着色器创建失败");
            });
        let shader = Arc::new(shader);

        // 创建数据
        let camera_data = crate::renderer::shaders::batch_render_shader::CameraData {
            view_proj: Mat4::IDENTITY.to_cols_array_2d(),
        };

        let camera_buffer = vulkan_helper::get_uniform_buffer(
            camera_data,
            Arc::clone(&buffer_allocator),
        );

        let instances = Instances {
            instances: [Instance::default(); 100]
        };

        let instances_buffer = Buffer::from_data(
            buffer_allocator.clone(),
            BufferCreateInfo {
                usage: BufferUsage::STORAGE_BUFFER,
                ..BufferCreateInfo::default()
            },
            AllocationCreateInfo {
                memory_type_filter: MemoryTypeFilter::HOST_SEQUENTIAL_WRITE,
                ..AllocationCreateInfo::default()
            },
            instances
        ).unwrap();

        // 创建管道
        let pipeline = vulkan_helper::get_graphics_pipeline(
            Arc::clone(&window),
            Arc::clone(&device),
            Arc::clone(&render_pass),
            shader.clone()
        );

        // 从管道获取描述符集布局
        let set_layout = pipeline.layout().set_layouts().get(0).unwrap().clone();

        // 创建描述符集

        let w1 = WriteDescriptorSet::buffer(0, camera_buffer.clone());
        let w2 = WriteDescriptorSet::buffer(1, instances_buffer.clone());

        let descriptor_set = DescriptorSet::new(
            descriptor_set_allocator.clone(),
            set_layout,
            [w1,w2],
            []
        ).unwrap();

        // 创建内存分配集
        let allocators = Allocators {
            buffer_allocator,
            descriptor_set_allocator,
            command_buffer_allocator,
        };

        // 创建着色器上下文
        let shader_data = ShaderData {
            camera_buffer,
            instances_buffer,
            instance_index: 0,
            set: descriptor_set.clone(),
        };



        Self {
            render_triangle: Box::new(
                RenderTriangle::new(
                    Arc::clone(&device),
                    Arc::clone(&window),
                    Arc::clone(&render_pass),
                    &allocators
                ),
            ),
            render_rectangle: Box::new(
              RenderRectangle::new(
                  Arc::clone(&device),
                  Arc::clone(&window),
                  Arc::clone(&render_pass),
                  &allocators
              )
            ),
            queue,
            window,
            device,
            render_pass,
            shader,
            pipeline,
            frame_commands: None,
            allocators,
            descriptor_set,
            shader_data,
            renderer_context: RendererContext {
                instance_index: 0
            }
        }
    }

    pub fn update_camera(&mut self, view_projection_matrix: Mat4) {
        self.shader_data.update_camera_buffer(view_projection_matrix);
    }

    fn begin(&mut self, frame: &mut FrameCommands, framebuffer: Arc<Framebuffer>, clear_color: [f32; 4], ) {
        self.shader_data.begin_frame();
        self.renderer_context.instance_index = 0;

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
                self.descriptor_set.clone(),
            ).unwrap()
            .bind_pipeline_graphics(self.pipeline.clone())
            .unwrap();
    }

    fn end(&mut self, frame: &mut FrameCommands) {
        frame.builder
            .end_render_pass(SubpassEndInfo::default())
            .unwrap();
    }

    pub fn draw_triangle(&mut self, transform: Transform2D, color: [f32; 4]) {
        let ins = Instance {
            transform: transform.to_mat4().to_cols_array_2d(),
            color,
        };
        self.shader_data.add_instance(ins);

        if let Some(frame) = self.frame_commands.as_mut() {
            self.render_triangle.draw(frame, self.renderer_context.instance_index);
        }

        self.renderer_context.instance_index += 1;
    }

    pub fn draw_rectangle(&mut self, transform: Transform2D, color: [f32; 4]) {
        let ins = Instance {
            transform: transform.to_mat4().to_cols_array_2d(),
            color,
        };
        self.shader_data.add_instance(ins);

        if let Some(frame) = self.frame_commands.as_mut() {
            self.render_rectangle.draw(frame, self.renderer_context.instance_index);
        }

        self.renderer_context.instance_index += 1;
    }

    pub fn recreate_pipeline(&mut self) {
        self.pipeline = vulkan_helper::get_graphics_pipeline(
            self.window.clone(),
            self.device.clone(),
            self.render_pass.clone(),
            self.shader.clone()
        )
    }

    pub fn render_frame(
        &mut self,
        frame_buffer: Arc<Framebuffer>,
        clear_color: [f32; 4],
        layer_stack: &mut LayerStack
    ) -> Arc<PrimaryAutoCommandBuffer> {
        let mut frame = FrameCommands::new(self.allocators.command_buffer_allocator.clone(), self.queue.clone());

        self.begin(&mut frame, frame_buffer, clear_color);

        self.frame_commands = Some(frame);

        layer_stack.iter_mut().for_each(|layer| {
            layer.on_render(self);
        });

        frame = self.frame_commands.take().unwrap();

        self.end(&mut frame);

        frame.builder.build().unwrap()
    }
}