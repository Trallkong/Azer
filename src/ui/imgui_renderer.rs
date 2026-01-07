use crate::api::vulkan_helper;
use crate::core::core::print_mem;
use crate::renderer::frame_commands::FrameCommands;
use crate::renderer::image_buffer_man::ImageBufferManager;
use crate::renderer::shaders::Shader;
use imgui::{DrawCmd, DrawIdx};
use log::error;
use smallvec::smallvec;
use std::fmt::Debug;
use std::sync::Arc;
use vulkano::{
    buffer::{Buffer, BufferContents, BufferCreateInfo, BufferUsage, Subbuffer},
    descriptor_set::allocator::StandardDescriptorSetAllocator,
    descriptor_set::{DescriptorSet, WriteDescriptorSet},
    device::Device,
    format::Format,
    image::sampler::{Sampler, SamplerCreateInfo},
    image::view::ImageView,
    memory::allocator::{AllocationCreateInfo, MemoryTypeFilter, StandardMemoryAllocator},
    pipeline::graphics::color_blend::{AttachmentBlend, BlendFactor, BlendOp, ColorBlendAttachmentState, ColorBlendState},
    pipeline::graphics::input_assembly::InputAssemblyState,
    pipeline::graphics::multisample::MultisampleState,
    pipeline::graphics::rasterization::RasterizationState,
    pipeline::graphics::vertex_input::{Vertex, VertexDefinition},
    pipeline::graphics::viewport::{Scissor, Viewport, ViewportState},
    pipeline::graphics::GraphicsPipelineCreateInfo,
    pipeline::layout::PipelineDescriptorSetLayoutCreateInfo,
    pipeline::{DynamicState, GraphicsPipeline, Pipeline, PipelineBindPoint, PipelineLayout, PipelineShaderStageCreateInfo},
    render_pass::{RenderPass, Subpass},
    shader::ShaderModule,
    Validated,
    VulkanError
};
use winit::window::Window;

mod vs {
    vulkano_shaders::shader! {
        ty: "vertex",
        src: r"
            #version 460

            layout(location = 0) in vec2 pos;
            layout(location = 1) in vec2 uv;
            layout(location = 2) in vec4 color;

            layout(push_constant) uniform PushConstants {
                vec2 scale;
                vec2 translate;
            } pc;

            layout(location = 0) out vec2 v_uv;
            layout(location = 1) out vec4 v_color;

            void main() {
                v_uv = uv;
                v_color = color;

                vec2 pos = pos * pc.scale + pc.translate;
                gl_Position = vec4(pos, 0.0, 1.0);
            }
        "
    }
}

mod fs {
    vulkano_shaders::shader! {
        ty: "fragment",
        src: r"
            #version 460

            layout(location = 0) in vec2 v_uv;
            layout(location = 1) in vec4 v_color;

            layout(set = 0, binding = 0) uniform sampler2D font_texture;

            layout(location = 0) out vec4 o_color;

            void main() {
                vec4 tex = texture(font_texture, v_uv);
                o_color = v_color * tex;
            }
        "
    }
}

#[derive(Clone)]
#[derive(Debug)]
pub struct ImGuiShader {
    vs: Arc<ShaderModule>,
    fs: Arc<ShaderModule>,
}

impl Shader for ImGuiShader {
    fn fs(&self) -> &Arc<ShaderModule> {
        &self.fs
    }

    fn vs(&self) -> &Arc<ShaderModule> {
        &self.vs
    }

    fn load(device: Arc<Device>) -> Result<Self, Validated<VulkanError>>
    where
        Self: Sized + Clone + Debug
    {
        Ok(Self {
            vs: vs::load(device.clone())?,
            fs: fs::load(device.clone())?,
        })
    }
}

pub struct ImGuiRenderer {
    pipeline: Arc<GraphicsPipeline>,
    memory_allocator: Arc<StandardMemoryAllocator>,
    set: Arc<DescriptorSet>,
}

#[repr(C)]
#[derive(BufferContents, Vertex)]
pub struct VertexForImGui {
    #[format(R32G32_SFLOAT)]
    pos: [f32; 2],
    #[format(R32G32_SFLOAT)]
    uv: [f32; 2],
    #[format(R8G8B8A8_UNORM)]
    color: [u8; 4]
}

#[repr(C)]
#[derive(BufferContents)]
pub struct PushConstants {
    scale: [f32; 2],
    translate: [f32; 2]
}

impl ImGuiRenderer {
    pub fn new(
        window: Arc<Window>,
        device: Arc<Device>,
        render_pass: Arc<RenderPass>,
        memory_allocator: Arc<StandardMemoryAllocator>,
        set_allocator: Arc<StandardDescriptorSetAllocator>,
        imgui: &mut imgui::Context,
        map: &mut ImageBufferManager
    ) -> Self {
        let shader = Arc::new(ImGuiShader::load(device.clone()).unwrap());

        let pipeline = Self::get_pipeline(
            window,
            shader,
            device.clone(),
            render_pass.clone()
        );

        let fonts = imgui.fonts();
        let atlas = fonts.build_rgba32_texture();

        print_mem("before imgui renderer staging");
        let staging = vulkan_helper::get_staging(atlas.data.into(), memory_allocator.clone());
        print_mem("after imgui renderer staging");

        let image = vulkan_helper::get_texture_image_2d((atlas.width, atlas.height), Format::R8G8B8A8_UNORM ,memory_allocator.clone());
        let view = ImageView::new_default(image.clone()).unwrap();

        map.add(image.clone(), staging);

        let sampler = Sampler::new(
            device.clone(),
            SamplerCreateInfo::simple_repeat_linear()
        ).unwrap();

        let set = DescriptorSet::new(
            set_allocator.clone(),
            pipeline.layout().set_layouts()[0].clone(),
            [WriteDescriptorSet::image_view_sampler(
                0,
                view.clone(),
                sampler.clone()
            )],
            []
        ).unwrap();

        Self {
            pipeline: pipeline.clone(),
            memory_allocator,
            set
        }
    }

    pub fn get_vbo_and_ibi_from_draw_data(
        &mut self,
        draw_data: &imgui::DrawData,
    )-> (Subbuffer<[VertexForImGui]>, Subbuffer<[DrawIdx]>) {
        let mut vertices = Vec::new();
        let mut indices = Vec::new();

        for draw_list in draw_data.draw_lists() {
            for v in draw_list.vtx_buffer() {
                vertices.push(VertexForImGui {
                    pos: [v.pos[0], v.pos[1]],
                    uv: [v.uv[0], v.uv[1]],
                    color: v.col
                })
            }

            for i in draw_list.idx_buffer() {
                indices.push(*i)
            }
        }

        let vertex_buffers = Buffer::from_iter(
            self.memory_allocator.clone(),
            BufferCreateInfo {
                usage: BufferUsage::VERTEX_BUFFER,
                ..BufferCreateInfo::default()
            },
            AllocationCreateInfo {
                memory_type_filter: MemoryTypeFilter::PREFER_DEVICE | MemoryTypeFilter::HOST_SEQUENTIAL_WRITE,
                ..AllocationCreateInfo::default()
            },
            vertices
        ).expect("Failed to create vertex buffer");

        let index_buffers = Buffer::from_iter(
            self.memory_allocator.clone(),
            BufferCreateInfo {
                usage: BufferUsage::INDEX_BUFFER,
                ..BufferCreateInfo::default()
            },
            AllocationCreateInfo {
                memory_type_filter: MemoryTypeFilter::PREFER_DEVICE | MemoryTypeFilter::HOST_SEQUENTIAL_WRITE,
                ..AllocationCreateInfo::default()
            },
            indices,
        ).unwrap();

        (vertex_buffers, index_buffers)
    }

    pub fn draw(&mut self, frame: &mut FrameCommands, draw_data: &imgui::DrawData, viewport: Viewport) {

        if draw_data.total_vtx_count == 0 || draw_data.total_idx_count == 0 {
            return; // 当前帧 ImGui 没有可绘制数据
        }

        let (vbo, ibo) = self.get_vbo_and_ibi_from_draw_data(draw_data);

        let display_size = draw_data.display_size;
        let display_pos = draw_data.display_pos;

        let scale = [
            2.0 / display_size[0],
            2.0 / display_size[1],
        ];

        let translate = [
            -1.0 - display_pos[0] * scale[0],
            -1.0 - display_pos[1] * scale[1],
        ];

        let mut vertex_offset: i32 = 0;
        let mut index_offset: u32  = 0;

        frame.builder
            .bind_pipeline_graphics(self.pipeline.clone())
            .unwrap()
            .bind_descriptor_sets(
                PipelineBindPoint::Graphics,
                self.pipeline.layout().clone(),
                0,
                self.set.clone(),
            )
            .unwrap()
            .push_constants(
                self.pipeline.layout().clone(),
                0,
                PushConstants {
                    scale,
                    translate
                }
            )
            .unwrap()
            .bind_vertex_buffers(0, vbo.clone())
            .unwrap()
            .bind_index_buffer(ibo.clone())
            .unwrap();


        unsafe {
            for draw_list in draw_data.draw_lists() {
                for cmd in draw_list.commands() {
                    match cmd {
                        DrawCmd::Elements { count, cmd_params } => {
                            let clip = cmd_params.clip_rect;

                            let clip_x1 = clip[0] - display_pos[0];
                            let clip_y1 = clip[1] - display_pos[1];
                            let clip_x2 = clip[2] - display_pos[0];
                            let clip_y2 = clip[3] - display_pos[1];

                            let scissor = Scissor {
                                offset: [
                                    clip_x1.max(0.0) as u32,
                                    clip_y1.max(0.0) as u32,
                                ],
                                extent: [
                                    (clip_x2 - clip_x1) as u32,
                                    (clip_y2 - clip_y1) as u32,
                                ],
                            };

                            // 1. scissor
                            frame.builder
                                .set_scissor(
                                    0,
                                    smallvec![scissor]
                                )
                                .unwrap()
                                .set_viewport(0, smallvec![viewport.clone()])
                                .unwrap();

                            // 2. draw
                            frame.builder
                                .draw_indexed(
                                    count as u32,
                                    1,
                                    index_offset,
                                    vertex_offset,
                                    0,
                                )
                                .unwrap();

                            index_offset += count as u32;
                        },
                        _ => {}
                    }
                }
                vertex_offset += draw_list.vtx_buffer().len() as i32;
            }
        }
    }

    fn get_pipeline(
        win: Arc<Window>,
        shader: Arc<dyn Shader>,
        device: Arc<Device>,
        render_pass: Arc<RenderPass>
    ) -> Arc<GraphicsPipeline> {
        let viewport = Viewport {
            offset: [0.0,0.0],
            extent: win.inner_size().into(),
            depth_range: 0.0..=1.0,
        };

        let vs = shader.vs().entry_point("main").unwrap();
        let fs = shader.fs().entry_point("main").unwrap();

        let vertex_input_state = VertexForImGui::per_vertex()
            .definition(&vs)
            .unwrap_or_else(|e| {
                error!("获取顶点输入状态失败: {}", e);
                panic!("获取顶点输入状态失败")
            });

        let stages = [
            PipelineShaderStageCreateInfo::new(vs),
            PipelineShaderStageCreateInfo::new(fs),
        ];

        let layout = PipelineLayout::new(
            device.clone(),
            PipelineDescriptorSetLayoutCreateInfo::from_stages(&stages)
                .into_pipeline_layout_create_info(device.clone()).unwrap()
        ).unwrap();

        let subpass = Subpass::from(render_pass.clone(), 0).unwrap();

        GraphicsPipeline::new(
            device.clone(),
            None,
            GraphicsPipelineCreateInfo {
                stages: stages.into_iter().collect(),
                vertex_input_state: Some(vertex_input_state),
                input_assembly_state: Some(InputAssemblyState::default()),
                viewport_state: Some(ViewportState {
                    viewports: [viewport].into_iter().collect(),
                    ..ViewportState::default()
                }),
                rasterization_state: Some(RasterizationState::default()),
                multisample_state: Some(MultisampleState::default()),
                color_blend_state: Some(ColorBlendState::with_attachment_states(
                    subpass.num_color_attachments(),
                    ColorBlendAttachmentState {
                        blend: Some(AttachmentBlend {
                            src_color_blend_factor: BlendFactor::SrcAlpha,
                            dst_color_blend_factor: BlendFactor::OneMinusSrcAlpha,
                            color_blend_op: BlendOp::Add,
                            src_alpha_blend_factor: BlendFactor::One,
                            dst_alpha_blend_factor: BlendFactor::OneMinusSrcAlpha,
                            alpha_blend_op: BlendOp::Add,
                        }),
                        ..ColorBlendAttachmentState::default()
                    }
                )),
                subpass: Some(subpass.into()),
                depth_stencil_state: None,
                dynamic_state: [DynamicState::Scissor, DynamicState::Viewport].into_iter().collect(),
                ..GraphicsPipelineCreateInfo::layout(layout)
            }
        ).unwrap()
    }
}