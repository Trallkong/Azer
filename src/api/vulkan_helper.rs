use crate::api::shaders::Shader;
use crate::renderer::vertex::Vertex2D;
use log::error;
use std::collections::BTreeMap;
use std::sync::Arc;
use vulkano::buffer::{Buffer, BufferContents, BufferCreateInfo, BufferUsage, Subbuffer};
use vulkano::command_buffer::allocator::{StandardCommandBufferAllocator, StandardCommandBufferAllocatorCreateInfo};
use vulkano::command_buffer::{AutoCommandBufferBuilder, CommandBufferUsage, PrimaryAutoCommandBuffer, RenderPassBeginInfo, SubpassBeginInfo, SubpassContents, SubpassEndInfo};
use vulkano::descriptor_set::allocator::{StandardDescriptorSetAllocator, StandardDescriptorSetAllocatorCreateInfo};
use vulkano::descriptor_set::layout::{DescriptorSetLayout, DescriptorSetLayoutBinding, DescriptorSetLayoutCreateInfo, DescriptorType};
use vulkano::descriptor_set::DescriptorSet;
use vulkano::device::physical::PhysicalDevice;
use vulkano::device::{Device, DeviceCreateInfo, DeviceExtensions, Queue, QueueCreateInfo, QueueFlags};
use vulkano::format::Format;
use vulkano::image::view::ImageView;
use vulkano::image::{Image, ImageUsage};
use vulkano::instance::{Instance, InstanceCreateInfo, InstanceExtensions};
use vulkano::memory::allocator::{AllocationCreateInfo, MemoryTypeFilter, StandardMemoryAllocator};
use vulkano::pipeline::graphics::color_blend::{ColorBlendAttachmentState, ColorBlendState};
use vulkano::pipeline::graphics::input_assembly::InputAssemblyState;
use vulkano::pipeline::graphics::multisample::MultisampleState;
use vulkano::pipeline::graphics::rasterization::RasterizationState;
use vulkano::pipeline::graphics::vertex_input::{Vertex, VertexDefinition};
use vulkano::pipeline::graphics::viewport::{Viewport, ViewportState};
use vulkano::pipeline::graphics::GraphicsPipelineCreateInfo;
use vulkano::pipeline::layout::PipelineDescriptorSetLayoutCreateInfo;
use vulkano::pipeline::{GraphicsPipeline, PipelineLayout, PipelineShaderStageCreateInfo};
use vulkano::render_pass::{Framebuffer, FramebufferCreateInfo, RenderPass, Subpass};
use vulkano::swapchain::{PresentMode, Surface, Swapchain, SwapchainCreateInfo};
use vulkano::{single_pass_renderpass, VulkanLibrary};
use winit::window::Window;

/// 获取 VulkanLibrary
pub fn get_library() -> Arc<VulkanLibrary> {
    VulkanLibrary::new()
        .unwrap_or_else(|err| panic!("无法创建VulkanLibrary: {}",err))
}

/// 获取 VulkanInstance
pub fn get_instance(
    lib: Arc<VulkanLibrary>,
    win: Arc<Window>,
) -> Arc<Instance> {
    let required_extensions = Surface::required_extensions(&win)
        .unwrap_or_else(|err| panic!("获取窗口所需扩展失败: {}", err));

    let extensions = InstanceExtensions {
        ..required_extensions
    };

    Instance::new(
        lib,
        InstanceCreateInfo {
            enabled_extensions: extensions,
            ..InstanceCreateInfo::default()
        }
    ).unwrap_or_else(|err| panic!("无法创建Vulkan实例: {}", err))
}

/// 获取 VulkanSurface
pub fn get_surface(
    ins: Arc<Instance>,
    win: Arc<Window>,
) -> Arc<Surface> {
    Surface::from_window(ins, win)
        .unwrap_or_else(|err| panic!("创建Surface失败: {}", err))
}

/// 获取 Device 和 Queue
pub fn get_device_and_queue(
    ins: Arc<Instance>,
) -> (Arc<Device>, Arc<Queue>) {
    // 遍历物理设备搜寻符合要求的设备
    let physical_devices = ins.enumerate_physical_devices()
        .unwrap_or_else(|e| panic!("获取枚举设备失败: {}", e));

    for physical_device in physical_devices {

        // 检测是否支持图形队列
        if let Some(idx) = get_required_queue_family_index(
            physical_device.as_ref(), QueueFlags::GRAPHICS)
        {

            println!("物理设备信息:");
            println!("设备名称: {}", physical_device.properties().device_name);
            println!("设备类型: {:?}", physical_device.properties().device_type);

            println!("该设备支持图形队列，被选择为目标物理设备！");

            let queue_create_info = QueueCreateInfo {
                queue_family_index: idx,
                queues: vec![1.0],
                ..QueueCreateInfo::default()
            };

            let device_extensions = DeviceExtensions {
                khr_swapchain: true,
                ..DeviceExtensions::default()
            };

            let device_create_info = DeviceCreateInfo {
                queue_create_infos: vec![queue_create_info],
                enabled_extensions: device_extensions,
                ..DeviceCreateInfo::default()
            };

            let (device, mut queues) =
                Device::new(physical_device, device_create_info)
                .unwrap_or_else(|err| panic!("创建设备失败: {}",err));

            return (device, queues.next().unwrap());
        }

    };

    panic!("获取设备和队列失败: 无可用物理设备，物理设备需要包含图形队列");
}

/// 获取 SwapChian 和 Images
pub fn get_swapchain_and_images(
    device: Arc<Device>,
    surface: Arc<Surface>,
    win: Arc<Window>,
) -> (Arc<Swapchain>, Vec<Arc<Image>>) {
    let swapchain_create_info = SwapchainCreateInfo {
        image_format: Format::R8G8B8A8_UNORM,
        image_extent: win.inner_size().into(),
        image_usage: ImageUsage::COLOR_ATTACHMENT,
        present_mode: PresentMode::Fifo,
        ..SwapchainCreateInfo::default()
    };

    Swapchain::new(
        Arc::clone(&device),
        Arc::clone(&surface),
        swapchain_create_info
    ).unwrap_or_else(|err| panic!("图像交换链创建失败: {}", err))
}

/// 获取 CommandBufferAllocator
pub fn get_command_buffer_allocator(
    device: Arc<Device>,
) -> StandardCommandBufferAllocator {
    StandardCommandBufferAllocator::new(
        device,
        StandardCommandBufferAllocatorCreateInfo::default()
    )
}

/// 获取 RenderPass
pub fn get_render_pass(
    device: Arc<Device>,
    format: Format,
) -> Arc<RenderPass> {
    let render_pass = single_pass_renderpass!(
        device,
        attachments: {
            foo: {
                format: format,
                samples: 1,
                load_op: Clear,
                store_op: Store,
            },
        },
        pass: {
            color: [foo],
            depth_stencil: {},
        }
    )
        .unwrap_or_else(|err| panic!("创建渲染令牌: {}", err));
    render_pass
}

/// 获取 Framebuffers
pub fn get_framebuffers(
    images: Vec<Arc<Image>>,
    render_pass: Arc<RenderPass>,
) -> Vec<Arc<Framebuffer>> {
    let mut framebuffers: Vec<Arc<Framebuffer>> = Vec::new();

    images.iter().for_each(|image| {
        let view = ImageView::new_default(image.clone()).unwrap();
        let framebuffer = Framebuffer::new(
            Arc::clone(&render_pass),
            FramebufferCreateInfo {
                attachments: vec![view],
                ..FramebufferCreateInfo::default()
            }
        ).unwrap_or_else(|err| panic!("创建帧缓冲区失败: {}", err));

        framebuffers.push(framebuffer);
    });

    framebuffers
}

pub fn get_descriptor_set_layout(device: Arc<Device>, binding: u32) -> Arc<DescriptorSetLayout> {
    let layout_binding = DescriptorSetLayoutBinding::descriptor_type(DescriptorType::UniformBuffer);

    let mut bindings = BTreeMap::new();
    bindings.insert(binding, layout_binding);

    DescriptorSetLayout::new(
        Arc::clone(&device),
        DescriptorSetLayoutCreateInfo {
            bindings,
            ..DescriptorSetLayoutCreateInfo::default()
        }
    ).unwrap_or_else(|e| {
        error!("创建描述符集布局失败: {}", e);
        panic!("无法创建描述符集布局: {:?}", e);
    })
}
pub fn get_uniform_buffer<T>(buffer_data: T, allocator: Arc<StandardMemoryAllocator>,) -> Subbuffer<T>
where T: BufferContents
{
    Buffer::from_data(
        allocator,
        BufferCreateInfo {
            usage: BufferUsage::UNIFORM_BUFFER,
            ..BufferCreateInfo::default()
        },
        AllocationCreateInfo {
            memory_type_filter: MemoryTypeFilter::HOST_SEQUENTIAL_WRITE,
            ..AllocationCreateInfo::default()
        },
        buffer_data
    ).unwrap_or_else(|e| {
        error!("创建uniform缓冲区失败: {}", e);
        panic!("无法创建uniform缓冲区: {:?}", e);
    })
}

pub fn get_descriptor_set_allocator(device: Arc<Device>) -> Arc<StandardDescriptorSetAllocator> {
    Arc::new(StandardDescriptorSetAllocator::new(
        Arc::clone(&device),
        StandardDescriptorSetAllocatorCreateInfo {
            set_count: 1,
            update_after_bind: false,
            ..StandardDescriptorSetAllocatorCreateInfo::default()
        }
    ))
}

pub fn get_descriptor_set<T>(
    uniform_buffer: Subbuffer<T>,
    binding: u32,
    layout: Arc<DescriptorSetLayout>,
    allocator: Arc<StandardDescriptorSetAllocator>
) -> Arc<DescriptorSet>
where T: BufferContents
{
    let write_descriptor_set = vulkano::descriptor_set::WriteDescriptorSet::buffer(binding, uniform_buffer.clone());

    DescriptorSet::new(
        allocator,
        layout,
        [write_descriptor_set],
        []
    ).unwrap()
}

/// 获取 GraphicsPipeline
pub fn get_graphics_pipeline(
    window: Arc<Window>,
    device: Arc<Device>,
    render_pass: Arc<RenderPass>,
    shader: Arc<dyn Shader>
) -> Arc<GraphicsPipeline> {
    let viewport = Viewport {
        offset: [0.0, 0.0],
        extent: window.inner_size().into(),
        depth_range: 0.0..=1.0,
    };

    let vs = shader.vs().entry_point("main").unwrap();
    let fs = shader.fs().entry_point("main").unwrap();

    let vertex_input_state = Vertex2D::per_vertex()
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

    let pipeline = GraphicsPipeline::new(
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
                ColorBlendAttachmentState::default()
            )),
            subpass: Some(subpass.into()),
            ..GraphicsPipelineCreateInfo::layout(layout)
        }
    ).unwrap();

    pipeline
}

pub struct VulkanHelper;

impl VulkanHelper {

    pub fn create_command_buffers(
        device: Arc<Device>,
        queue: Arc<Queue>,
        framebuffers: Vec<Arc<Framebuffer>>,
        pipeline: Arc<GraphicsPipeline>,
        vertex_buffer: Subbuffer<[Vertex2D]>) -> Vec<Arc<PrimaryAutoCommandBuffer>> {

        let allocator = Arc::new(StandardCommandBufferAllocator::new(
            device.clone(),
            StandardCommandBufferAllocatorCreateInfo::default()
        ));

        let mut command_buffers: Vec<Arc<PrimaryAutoCommandBuffer>> = Vec::new();
        framebuffers.into_iter().for_each(|framebuffer| {
            let mut builder =
                AutoCommandBufferBuilder::primary(
                    allocator.clone(),
                    queue.queue_family_index(),
                    CommandBufferUsage::OneTimeSubmit,
                ).unwrap();

            unsafe {
                builder
                    .begin_render_pass(
                        RenderPassBeginInfo {
                            clear_values: vec![Some([0.1, 0.1, 0.1, 1.0].into())],
                            ..RenderPassBeginInfo::framebuffer(framebuffer.clone())
                        },
                        SubpassBeginInfo {
                            contents: SubpassContents::Inline,
                            ..SubpassBeginInfo::default()
                        },
                    )
                    .unwrap()
                    .bind_pipeline_graphics(pipeline.clone())
                    .unwrap()
                    .bind_vertex_buffers(0, vertex_buffer.clone())
                    .unwrap()
                    .draw(
                        3, 1, 0, 0,
                    )
                    .unwrap()
                    .end_render_pass(SubpassEndInfo::default())
                    .unwrap();
            }

            let command_buffer = builder.build().unwrap();

            command_buffers.push(command_buffer);
        });

        command_buffers
    }
}

/// 判断某个物理设备是否符合需求并返回队列索引
///
/// @param physical_device 从枚举获得的物理设备
///
/// @param required_flag 所需设备类型的标识
///
/// @param required_flag 所需设备类型的标识
///
/// @return 设备队列索引（Option包裹）
///
fn get_required_queue_family_index(physical_device: &PhysicalDevice, required_flag: QueueFlags) -> Option<u32> {
    let properties = physical_device.queue_family_properties();
    for (i, properties) in properties.iter().enumerate() {
        if properties.queue_flags.contains(required_flag) {
            return Some(i as u32);
        }
    }
    None
}