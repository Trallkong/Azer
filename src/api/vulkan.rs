use crate::api::vulkan_helper;
use crate::core::layer_stack::LayerStack;
use crate::renderer::image_buffer_man::ImageBufferManager;
use crate::renderer::renderer::Renderer;
use log::error;
use std::sync::Arc;
use vulkano::{
    command_buffer::allocator::StandardCommandBufferAllocator,
    device::{Device, Queue},
    format::Format,
    image::Image,
    pipeline::graphics::viewport::Viewport,
    render_pass::{Framebuffer, RenderPass},
    swapchain::{acquire_next_image, Surface, Swapchain, SwapchainCreateInfo, SwapchainPresentInfo},
    sync,
    sync::GpuFuture,
    Validated,
    VulkanError
};
use winit::window::Window;

bitflags::bitflags! {
    pub struct RenderDirty: u32 {
        const NONE          = 0;
        const SWAPCHAIN     = 1 << 0;
        const PIPELINE      = 1 << 1;
        const COMMAND_BUF   = 1 << 2;
    }
}
pub struct Vulkan {
    pub device: Arc<Device>,
    pub queue: Arc<Queue>,
    pub surface: Arc<Surface>,
    pub swapchain: Arc<Swapchain>,
    pub swapchain_images: Vec<Arc<Image>>,
    pub render_pass: Arc<RenderPass>,
    pub frame_buffers: Vec<Arc<Framebuffer>>,
    pub viewport: Viewport,
    pub command_buffer_allocator: Arc<StandardCommandBufferAllocator>,

    pub dirty: RenderDirty,
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
            viewport,
            command_buffer_allocator: allocator,
            dirty: RenderDirty::NONE,
        }
    }

    pub fn submit(&mut self, renderer: &mut Renderer, layer_stack: &mut LayerStack, clear_color: [f32; 4], map: &mut ImageBufferManager) {

        let (image_i, suboptimal, acquire_future) =
            match acquire_next_image(self.swapchain.clone(), None)
                .map_err(Validated::unwrap)
            {
                Ok(result) => result,
                Err(VulkanError::OutOfDate) => {
                    self.dirty.insert(RenderDirty::SWAPCHAIN);
                    return;
                }
                Err(e) => panic!("获取下一张图像失败: {}",e),
            };

        if suboptimal {
            self.dirty.insert(RenderDirty::SWAPCHAIN);
            return;
        }

        let command_buffer = renderer.render_frame(
            self.frame_buffers[image_i as usize].clone(),
            clear_color,
            layer_stack,
            map
        );

        let execution = sync::now(self.device.clone())
            .join(acquire_future)
            .then_execute(self.queue.clone(), command_buffer)
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
                self.dirty.insert(RenderDirty::SWAPCHAIN);
            }
            Err(e) => {
                error!("failed to flush future: {e}")
            }
        }
    }

    pub fn recreate_swapchain(&mut self, window: Arc<Window>, renderer: &mut Renderer) {
        if window.is_minimized().unwrap() {
            return;
        }

        if self.dirty.contains(RenderDirty::SWAPCHAIN) {
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

            self.dirty.remove(RenderDirty::SWAPCHAIN);
        }

        if self.dirty.contains(RenderDirty::PIPELINE) {
            renderer.recreate_pipeline();
            self.dirty.remove(RenderDirty::PIPELINE);
        }
    }
}