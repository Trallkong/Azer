use crate::renderer::frame_commands::FrameCommands;
use crate::renderer::image_buffer_man::ImageBufferManager;
use crate::renderer::renderer::Allocators;
use crate::renderer::vertex::{get_vbo_and_ibo_2d, Vertex2D};
use std::sync::Arc;
use vulkano::buffer::{Buffer, BufferCreateInfo, BufferUsage, Subbuffer};
use vulkano::descriptor_set::layout::DescriptorSetLayout;
use vulkano::descriptor_set::{DescriptorSet, WriteDescriptorSet};
use vulkano::device::Device;
use vulkano::format::Format;
use vulkano::image::sampler::{Sampler, SamplerCreateInfo};
use vulkano::image::view::ImageView;
use vulkano::image::{Image, ImageCreateInfo, ImageType, ImageUsage};
use vulkano::memory::allocator::{AllocationCreateInfo, MemoryTypeFilter};
use vulkano::pipeline::PipelineLayout;

pub struct RenderTriangle {
    vbo: Subbuffer<[Vertex2D]>,
    ibo: Subbuffer<[u32]>,
    set1: Arc<DescriptorSet>,
    set2: Arc<DescriptorSet>,
    pipeline_layout: Arc<PipelineLayout>,
}

impl RenderTriangle {
    pub fn new(
        allocators: &Allocators,
        set1: Arc<DescriptorSet>,
        pipeline_layout: Arc<PipelineLayout>,
        set_layout: Arc<DescriptorSetLayout>,
        device: Arc<Device>,
        map: &mut ImageBufferManager
    ) -> Self {
        let vertices = vec![
            Vertex2D { position: [0.0, 0.5], uv: [0.0, 1.0] },
            Vertex2D { position: [0.5, -0.5], uv: [1.0, 0.0]},
            Vertex2D { position: [-0.5, -0.5], uv: [0.0, 0.0]},
        ];

        let indices = vec![0, 1, 2];

        let (vbo, ibo) = get_vbo_and_ibo_2d(
            vertices,
            indices,
            allocators.buffer_allocator.clone()
        );

        let pixels: [u8;4] = [255, 255, 255, 255];

        let staging = Buffer::from_iter(
            allocators.buffer_allocator.clone(),
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
            allocators.buffer_allocator.clone(),
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
            device,
            SamplerCreateInfo::simple_repeat_linear()
        ).unwrap();

        let set2 = DescriptorSet::new(
            allocators.descriptor_set_allocator.clone(),
            set_layout,
            [WriteDescriptorSet::image_view_sampler(0, image_view, sampler)],
            []
        ).unwrap();

        map.add(image.clone(), staging.clone());

        RenderTriangle { vbo, ibo, set1, set2, pipeline_layout }
    }

    pub fn draw(&mut self, frame: &mut FrameCommands, instance_index: usize) {
        unsafe {
            frame.builder
                .bind_descriptor_sets(
                    vulkano::pipeline::PipelineBindPoint::Graphics,
                    self.pipeline_layout.clone(),
                    0,
                    Vec::from([self.set1.clone(), self.set2.clone()])
                )
                .unwrap()
                .bind_vertex_buffers(0, self.vbo.clone())
                .unwrap()
                .bind_index_buffer(self.ibo.clone())
                .unwrap()
                .draw_indexed(3, 1, 0, 0, instance_index as u32)
                .unwrap();
        }
    }
}