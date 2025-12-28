use log::error;
use std::sync::Arc;
use vulkano::command_buffer::allocator::StandardCommandBufferAllocator;
use vulkano::command_buffer::{AutoCommandBufferBuilder, CommandBufferUsage, PrimaryAutoCommandBuffer};
use vulkano::device::Queue;

pub struct FrameCommands {
    pub builder: AutoCommandBufferBuilder<PrimaryAutoCommandBuffer>,
}

impl FrameCommands {
    pub fn new(
        allocator: Arc<StandardCommandBufferAllocator>,
        queue: Arc<Queue>
    ) -> Self {
        let builder = AutoCommandBufferBuilder::primary(
            allocator,
            queue.queue_family_index(),
            CommandBufferUsage::OneTimeSubmit,
        ).unwrap_or_else(|e| {
            error!("Failed to create command buffer builder: {:?}", e);
            panic!("Failed to create command buffer builder");
        });

        Self {
            builder
        }
    }
}