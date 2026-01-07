use crate::renderer::image_buffer_man::ImageBufferManager;
use std::sync::Arc;
use vulkano::buffer::{Buffer, BufferCreateInfo, BufferUsage};
use vulkano::descriptor_set::allocator::StandardDescriptorSetAllocator;
use vulkano::descriptor_set::layout::DescriptorSetLayout;
use vulkano::descriptor_set::{DescriptorSet, WriteDescriptorSet};
use vulkano::device::Device;
use vulkano::format::Format;
use vulkano::image::sampler::{Sampler, SamplerCreateInfo};
use vulkano::image::view::ImageView;
use vulkano::image::{Image, ImageCreateInfo, ImageType, ImageUsage};
use vulkano::memory::allocator::{AllocationCreateInfo, MemoryTypeFilter, StandardMemoryAllocator};

pub fn get_default_set(
    memory_allocator: Arc<StandardMemoryAllocator>,
    device: Arc<Device>,
    set_allocator: Arc<StandardDescriptorSetAllocator>,
    set_layout: Arc<DescriptorSetLayout>,
    map: &mut ImageBufferManager
) -> Arc<DescriptorSet> {

    let pixels: [u8;4] = [255, 255, 255, 255];

    let staging = Buffer::from_iter(
        memory_allocator.clone(),
        BufferCreateInfo {
            usage: BufferUsage::TRANSFER_SRC,
            ..BufferCreateInfo::default()
        },
        AllocationCreateInfo {
            memory_type_filter: MemoryTypeFilter::PREFER_HOST | MemoryTypeFilter::HOST_SEQUENTIAL_WRITE,
            ..AllocationCreateInfo::default()
        },
        pixels,
    ).unwrap();

    let image = Image::new(
        memory_allocator.clone(),
        ImageCreateInfo {
            image_type: ImageType::Dim2d,
            format: Format::R8G8B8A8_UNORM,
            extent: [1,1,1],
            usage: ImageUsage::TRANSFER_DST | ImageUsage::SAMPLED,
            ..ImageCreateInfo::default()
        },
        AllocationCreateInfo {
            memory_type_filter: MemoryTypeFilter::PREFER_DEVICE,
            ..AllocationCreateInfo::default()
        },
    ).unwrap();

    let image_view = ImageView::new_default(image.clone()).unwrap();

    let sampler = Sampler::new(
        device.clone(),
        SamplerCreateInfo::simple_repeat_linear()
    ).unwrap();

    map.add(image, staging);

    DescriptorSet::new(
        set_allocator.clone(),
        set_layout.clone(),
        [WriteDescriptorSet::image_view_sampler(0, image_view, sampler)],
        []
    ).unwrap()
}