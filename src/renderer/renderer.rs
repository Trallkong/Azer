use crate::api::vulkan_helper;
use crate::core::core::new_scope;
use crate::core::layer_stack::LayerStack;
use crate::renderer::frame_commands::FrameCommands;
use crate::renderer::image_buffer_man::ImageBufferManager;
use crate::renderer::renderer2d::render_helper::get_default_set;
use crate::renderer::renderer2d::render_image::RenderImage;
use crate::renderer::shaders::upgrade_shader::{PushConstants, UpgradeShader};
use crate::renderer::shaders::Shader;
use crate::renderer::shapes::shape_2d::rectangle::Rectangle;
use crate::renderer::shapes::shape_2d::triangle::Triangle;
use crate::renderer::shapes::transform::Transform;
use crate::renderer::shapes::{DrawList, GameObject, Shape};
use crate::renderer::vertex;
use crate::ui::imgui_renderer::ImGuiRenderer;
use glam::Mat4;
use imgui::DrawData;
use log::error;
use std::sync::Arc;
use vulkano::command_buffer::PrimaryAutoCommandBuffer;
use vulkano::descriptor_set::allocator::StandardDescriptorSetAllocator;
use vulkano::descriptor_set::DescriptorSet;
use vulkano::memory::allocator::StandardMemoryAllocator;
use vulkano::pipeline::graphics::viewport::Viewport;
use vulkano::pipeline::{GraphicsPipeline, Pipeline, PipelineBindPoint};
use vulkano::{
    command_buffer::{allocator::StandardCommandBufferAllocator, RenderPassBeginInfo, SubpassBeginInfo, SubpassContents, SubpassEndInfo},
    device::{Device, Queue},
    render_pass::{Framebuffer, RenderPass}
};
use winit::window::Window;

pub struct Allocators {
    pub buffer_allocator: Arc<StandardMemoryAllocator>,
    pub descriptor_set_allocator: Arc<StandardDescriptorSetAllocator>,
    pub command_buffer_allocator: Arc<StandardCommandBufferAllocator>,
}

pub struct Renderer {
    render_image: Box<RenderImage>,

    queue: Arc<Queue>,
    shader: Arc<UpgradeShader>,
    device: Arc<Device>,
    render_pass: Arc<RenderPass>,
    pipeline: Arc<GraphicsPipeline>,
    pub allocators: Allocators,

    draw_list: DrawList,
    view_proj: [[f32;4];4],
    default_set: Arc<DescriptorSet>,
}

impl Renderer {
    pub fn new(
        device: Arc<Device>,
        queue: Arc<Queue>,
        window: Arc<Window>,
        render_pass: Arc<RenderPass>,
        map: &mut ImageBufferManager
    ) -> Self {
        let command_buffer_allocator =
            vulkan_helper::get_cmd_buffer_allocator(device.clone());

        let buffer_allocator =
            vulkan_helper::get_mem_allocator(device.clone());

        let descriptor_set_allocator = vulkan_helper::get_descriptor_set_allocator(device.clone());

        // 准备着色器
        let shader = Arc::new(UpgradeShader::load(device.clone())
            .unwrap_or_else(|e| {
                error!("着色器创建失败: {}", e);
                panic!("着色器创建失败");
            }));

        let viewport = Viewport {
            offset: [0.0,0.0],
            extent: window.inner_size().into(),
            depth_range: 0.0..=1.0
        };

        // 创建管道
        let pipeline = vulkan_helper::get_graphics_pipeline(
            Arc::clone(&device),
            Arc::clone(&render_pass),
            shader.clone(),
            viewport.clone()
        );

        // 创建内存分配集
        let allocators = Allocators {
            buffer_allocator,
            descriptor_set_allocator,
            command_buffer_allocator,
        };

        let default_set = get_default_set(
            allocators.buffer_allocator.clone(),
            device.clone(),
            allocators.descriptor_set_allocator.clone(),
            pipeline.layout().set_layouts()[0].clone(),
            map
        );

        Self {
            render_image: Box::new(RenderImage::new(
                device.clone(),
                pipeline.layout().set_layouts()[0].clone(),
                allocators.buffer_allocator.clone(),
                allocators.descriptor_set_allocator.clone()
            )),
            queue,
            shader,
            device,
            render_pass,
            pipeline,
            allocators,
            draw_list: DrawList::default(),
            view_proj: Mat4::IDENTITY.to_cols_array_2d(),
            default_set,
        }
    }

    pub fn update_camera(&mut self, view_projection_matrix: Mat4) {
        self.view_proj = view_projection_matrix.to_cols_array_2d();
    }

    fn begin(&mut self, frame: &mut FrameCommands, framebuffer: Arc<Framebuffer>, clear_color: [f32; 4], ) {
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
            .bind_pipeline_graphics(self.pipeline.clone())
            .unwrap();
    }

    fn end(&mut self, frame: &mut FrameCommands) {
        frame.builder
            .end_render_pass(SubpassEndInfo::default())
            .unwrap();

        self.draw_list.clear();
    }

    pub fn draw_triangle(&mut self, transform: Transform, color: [f32; 4]) {
        let triangle = Triangle::new();

        let mut mesh = triangle.mesh().get_transformed_mesh(&transform);
        mesh.set_color(color);

        let obj = GameObject {
            vertex_len: mesh.vertices.len() as u32,
            index_count: mesh.indices.len() as u32,
            transform: transform.to_mat4().to_cols_array_2d(),
            set: self.default_set.clone(),
        };

        self.draw_list.vertices.extend(mesh.vertices);
        self.draw_list.indices.extend(mesh.indices);
        self.draw_list.objects.push(new_scope(obj));
    }

    pub fn draw_rectangle(&mut self, transform: Transform, color: [f32; 4]) {
        let rectangle = Rectangle::new();

        let mut mesh = rectangle.mesh().get_transformed_mesh(&transform);
        mesh.set_color(color);

        let obj = GameObject {
            vertex_len: mesh.vertices.len() as u32,
            index_count: mesh.indices.len() as u32,
            transform: transform.to_mat4().to_cols_array_2d(),
            set: self.default_set.clone(),
        };

        self.draw_list.vertices.extend(mesh.vertices);
        self.draw_list.indices.extend(mesh.indices);
        self.draw_list.objects.push(new_scope(obj));
    }

    pub fn draw_image(&mut self, transform: Transform, image_path: &str, map: &mut ImageBufferManager) {
        if let Some(rect) = self.render_image.import_image(image_path, map) {
            let mesh = rect.mesh().get_transformed_mesh(&transform);

            let obj = GameObject {
                vertex_len: mesh.vertices.len() as u32,
                index_count: mesh.indices.len() as u32,
                transform: transform.to_mat4().to_cols_array_2d(),
                set: self.render_image.set_sampler(image_path),
            };

            self.draw_list.vertices.extend(mesh.vertices);
            self.draw_list.indices.extend(mesh.indices);
            self.draw_list.objects.push(new_scope(obj));
        }
    }

    pub fn recreate_pipeline(&mut self, viewport: Viewport) {
        self.pipeline = vulkan_helper::get_graphics_pipeline(
            self.device.clone(),
            self.render_pass.clone(),
            self.shader.clone(),
            viewport
        )
    }

    pub fn draw(&mut self, frame: &mut FrameCommands) {
        let vbo = vertex::get_vbo_2d(self.draw_list.vertices.clone(), self.allocators.buffer_allocator.clone());
        let ibo = vertex::get_ibo_2d(self.draw_list.indices.clone(), self.allocators.buffer_allocator.clone());

        frame.builder
            .bind_vertex_buffers(0, vbo)
            .unwrap()
            .bind_index_buffer(ibo)
            .unwrap();

        let mut vertex_offset = 0;
        let mut index_offset = 0;

        for obj in self.draw_list.objects.iter() {

            unsafe {
                frame.builder
                    .push_constants(self.pipeline.layout().clone(), 0, 
                        PushConstants {
                            view_proj: self.view_proj,
                            transform: obj.transform,
                    })
                    .unwrap()
                    .bind_descriptor_sets(
                        PipelineBindPoint::Graphics,
                        self.pipeline.layout().clone(),
                        0,
                        obj.set.clone()
                    )
                    .unwrap()
                    .draw_indexed(obj.index_count, 1, index_offset, vertex_offset, 0)
                    .unwrap();
            }

            vertex_offset += obj.vertex_len as i32;
            index_offset += obj.index_count;
        }
    }

    pub fn render_frame(
        &mut self,
        frame_buffer: Arc<Framebuffer>,
        clear_color: [f32; 4],
        layer_stack: &mut LayerStack,
        map: &mut ImageBufferManager,
        imgui_renderer: &mut ImGuiRenderer,
        draw_data: &DrawData,
        viewport: Viewport
    ) -> Arc<PrimaryAutoCommandBuffer> {
        let mut frame = FrameCommands::new(self.allocators.command_buffer_allocator.clone(), self.queue.clone());

        self.begin(&mut frame, frame_buffer, clear_color);

        layer_stack.iter_mut().for_each(|layer| {
            layer.on_render(self, map);
        });

        self.draw(&mut frame);

        imgui_renderer.draw(&mut frame, draw_data, viewport);

        self.end(&mut frame);

        // copy_buffer_to_image here!
        map.copy_all_buffer_to_image(&mut frame);
        // --------------------------
        map.clear();

        frame.builder.build().unwrap()
    }
}