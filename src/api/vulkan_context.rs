use std::sync::Arc;
use vulkano::command_buffer::allocator::StandardCommandBufferAllocator;
use vulkano::command_buffer::{AutoCommandBufferBuilder, CommandBuffer, PrimaryAutoCommandBuffer};
use vulkano::device::Device;
use vulkano::image::Image;
use vulkano::memory::allocator::StandardMemoryAllocator;
use vulkano::swapchain::Swapchain;

struct VulkanContext {
    pub device: Arc<Device>,
    pub swapchain: Arc<Swapchain>,
    pub images: Vec<Arc<Image>>,
    pub memory_allocator: Arc<StandardMemoryAllocator>,
    pub cmd_bf_allocator: Arc<StandardCommandBufferAllocator>,
    pub cmd_bf_builder: Arc<AutoCommandBufferBuilder<PrimaryAutoCommandBuffer>>,
    pub command_buffers: Vec<Arc<CommandBuffer>>,
}