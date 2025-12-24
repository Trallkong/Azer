use std::sync::Arc;
use log::error;
use vulkano::{
    command_buffer::allocator::StandardCommandBufferAllocator,
    command_buffer::PrimaryAutoCommandBuffer,
    device::{Device, Queue},
    format::Format,
    image::Image,
    pipeline::graphics::viewport::Viewport,
    render_pass::{Framebuffer, RenderPass},
    swapchain::{acquire_next_image, Surface, Swapchain, SwapchainCreateInfo, SwapchainPresentInfo},
    sync,
    Validated,
    VulkanError,
    sync::GpuFuture
};
use winit::window::Window;
use crate::core::layer_stack::LayerStack;
use crate::render::renderer::Renderer;
use crate::api::vulkan_helper;

pub struct Vulkan {
    pub device: Arc<Device>,
    pub queue: Arc<Queue>,
    pub surface: Arc<Surface>,
    pub swapchain: Arc<Swapchain>,
    pub swapchain_images: Vec<Arc<Image>>,
    pub render_pass: Arc<RenderPass>,
    pub frame_buffers: Vec<Arc<Framebuffer>>,
    pub command_buffers: Vec<Arc<PrimaryAutoCommandBuffer>>,
    pub window_resized: bool,
    pub recreate_swapchain: bool,
    pub viewport: Viewport,
    pub command_buffer_allocator: Arc<StandardCommandBufferAllocator>,
}

impl Vulkan {
    pub fn new(window: Arc<Window>) -> Vulkan {

        let library = vulkan_helper::get_library();

        let instance = vulkan_helper::get_instance(
            Arc::clone(&library),
            Arc::clone(&window)
        );

        let surface = vulkan_helper::get_surface(
            Arc::clone(&instance),
            Arc::clone(&window)
        );

        let (device, queue) = vulkan_helper::get_device_and_queue(Arc::clone(&instance));

        let (swapchain, images) = vulkan_helper::get_swapchain_and_images(
            Arc::clone(&device),
            Arc::clone(&surface),
            Arc::clone(&window),
        );

        let allocator = Arc::new(vulkan_helper::get_command_buffer_allocator(Arc::clone(&device)));

        let viewport = Viewport {
            extent: window.clone().inner_size().into(),
            ..Viewport::default()
        };

        let render_pass = vulkan_helper::get_render_pass(device.clone(), Format::R8G8B8A8_UNORM);

        let framebuffers: Vec<Arc<Framebuffer>> = vulkan_helper::get_framebuffers(
            images.clone(),
            Arc::clone(&render_pass),
        );

        Vulkan {
            device,
            queue,
            surface,
            swapchain,
            swapchain_images: images,
            render_pass,
            frame_buffers: framebuffers,
            command_buffers: vec![],
            window_resized: false,
            recreate_swapchain: false,
            viewport,
            command_buffer_allocator: allocator,
        }
    }

    pub fn submit(&mut self) {

        let (image_i, suboptimal, acquire_future) =
            match acquire_next_image(self.swapchain.clone(), None)
                .map_err(Validated::unwrap)
            {
                Ok(result) => result,
                Err(VulkanError::OutOfDate) => {
                    self.recreate_swapchain = true;
                    return;
                }
                Err(e) => panic!("获取下一张图像失败: {}",e),
            };

        if suboptimal {
            self.recreate_swapchain = true;
            return;
        }

        let execution = sync::now(self.device.clone())
            .join(acquire_future)
            .then_execute(self.queue.clone(), self.command_buffers[image_i as usize].clone())
            .unwrap()
            .then_swapchain_present(
                self.queue.clone(),
                SwapchainPresentInfo::swapchain_image_index(self.swapchain.clone(), image_i),
            )
            .then_signal_fence_and_flush();

        match execution.map_err(Validated::unwrap) {
            Ok(future) => {
                future.wait(None).unwrap();
            }
            Err(VulkanError::OutOfDate) => {
                self.recreate_swapchain = true;
            }
            Err(e) => {
                error!("failed to flush future: {e}")
            }
        }
    }

    pub fn recreate_swapchain(&mut self, window: Arc<Window>, renderer: &mut Renderer, layer_stack: &mut LayerStack) {
        if self.window_resized || self.recreate_swapchain {
            self.recreate_swapchain = false;

            let new_dimensions = window.clone().inner_size();

            let (new_swapchain, new_images) = self.swapchain
                .recreate(SwapchainCreateInfo{
                    image_extent: new_dimensions.into(),
                    ..self.swapchain.create_info()
                })
                .expect("重建交换链失败！");

            self.swapchain = new_swapchain;
            self.swapchain_images = new_images.clone();
            self.frame_buffers = vulkan_helper::get_framebuffers(new_images, self.render_pass.clone());

            if self.window_resized {
                self.window_resized = false;

                self.viewport.extent = new_dimensions.into();

                renderer.recreate_pipeline();

                self.command_buffers = get_command_buffers(
                    renderer,
                    layer_stack,
                    self.frame_buffers.clone(),
                );
            }
        }
    }
}

/// 创建一组CommandBuffer（Arc包裹）
///
/// @param allocator 命令缓冲区分配器
///
/// @param queue 设备队列
///
/// @param pipeline 可选的图像管线，若为None即是清屏
///
/// @param framebuffers
///
/// @return 一组命令缓冲区（Arc包裹）
///
fn get_command_buffers(
    renderer: &mut Renderer,
    layer_stack: &mut LayerStack,
    framebuffers: Vec<Arc<Framebuffer>>,
) -> Vec<Arc<PrimaryAutoCommandBuffer>> {
    let mut command_buffers: Vec<Arc<PrimaryAutoCommandBuffer>> = Vec::new();

    framebuffers.into_iter().for_each(|framebuffer| {
        renderer.begin(
            framebuffer.clone(),
            [0.1,0.1,0.1,1.0]
        );

        layer_stack.iter_mut().for_each(|layer| {
            layer.on_render(renderer);
        });

        renderer.end();

        let command_buffer = renderer.submit();
        command_buffers.push(command_buffer);
    });

    command_buffers
}