use crate::renderer::frame_commands::FrameCommands;
use crate::renderer::image_buffer_man::ImageBufferManager;
use crate::renderer::renderer::Allocators;
use crate::renderer::vertex;
use crate::renderer::vertex::{get_vbo_from_size, Vertex2D};
use std::collections::HashMap;
use std::sync::Arc;
use log::info;
use vulkano::buffer::Subbuffer;
use vulkano::descriptor_set::allocator::StandardDescriptorSetAllocator;
use vulkano::descriptor_set::layout::DescriptorSetLayout;
use vulkano::descriptor_set::{DescriptorSet, WriteDescriptorSet};
use vulkano::device::Device;
use vulkano::format::Format;
use vulkano::image::sampler::{Sampler, SamplerCreateInfo};
use vulkano::image::view::ImageView;
use vulkano::memory::allocator::StandardMemoryAllocator;
use vulkano::pipeline::{PipelineBindPoint, PipelineLayout};
use crate::api::vulkan_helper;

pub struct TextureObject {
    pub image_view: Arc<ImageView>,
    pub vbo: Subbuffer<[Vertex2D]>,
    pub set: Arc<DescriptorSet>,
}

pub struct RenderImage {
    textures: HashMap<String, TextureObject>,
    ibo: Subbuffer<[u32]>,

    set_layout: Arc<DescriptorSetLayout>,
    pipeline_layout: Arc<PipelineLayout>,

    memory_allocator: Arc<StandardMemoryAllocator>,
    set_allocator: Arc<StandardDescriptorSetAllocator>,

    set1: Arc<DescriptorSet>,
    sampler: Arc<Sampler>
}

impl RenderImage {
    pub fn new(allocators: &Allocators, device: Arc<Device>, set_layout: Arc<DescriptorSetLayout>, pipeline_layout: Arc<PipelineLayout>, set1: Arc<DescriptorSet>) -> Self {
        // 矩形通用IBO
        let indices = vec![0, 1, 2, 2, 3, 0];
        let ibo = vertex::get_ibo_2d(indices, allocators.buffer_allocator.clone());

        let sampler = Sampler::new(
            device.clone(),
            SamplerCreateInfo::simple_repeat_linear(),
        ).unwrap();

        Self {
            textures: HashMap::new(),
            ibo,
            set_layout,
            pipeline_layout,
            memory_allocator: allocators.buffer_allocator.clone(),
            set_allocator: allocators.descriptor_set_allocator.clone(),
            set1,
            sampler
        }
    }

    pub fn draw(&mut self, frame: &mut FrameCommands, instance_index: usize, image_path: &str, map: &mut ImageBufferManager) {
        if !self.textures.contains_key(image_path) {
            info!("importing: {}", image_path);
            self.import_image(image_path, map);
        }

        unsafe {
            frame.builder
                .bind_descriptor_sets(
                    PipelineBindPoint::Graphics,
                    self.pipeline_layout.clone(),
                    0,
                    Vec::from([self.set1.clone(), self.textures.get(image_path).unwrap().set.clone()]),
                )
                .unwrap()
                .bind_vertex_buffers(0, [self.textures.get(image_path).unwrap().vbo.clone()])
                .unwrap()
                .bind_index_buffer(self.ibo.clone())
                .unwrap()
                .draw_indexed(6, 1, 0, 0, instance_index as u32)
                .unwrap();
        }
    }

    fn import_image(&mut self, img_path: &str, map: &mut ImageBufferManager) {
        let img = image::open(img_path).unwrap().flipv().to_rgba8();
        let (width, height) = img.dimensions();
        let pixels = img.into_raw();
        assert_eq!(pixels.len(), (width * height * 4) as usize);

        // 创建VBO
        let vbo = get_vbo_from_size((width, height), self.memory_allocator.clone());

        let image = vulkan_helper::get_texture_image_2d((width,height), Format::R8G8B8A8_UNORM, self.memory_allocator.clone());

        let staging = vulkan_helper::get_staging(pixels, self.memory_allocator.clone());

        let image_view = ImageView::new_default(image.clone()).unwrap();

        let set2 = DescriptorSet::new(
            self.set_allocator.clone(),
            self.set_layout.clone(),
            [
                WriteDescriptorSet::image_view_sampler(
                    0,
                    image_view.clone(),
                    self.sampler.clone(),
                ), ],
            []
        ).unwrap();

        let texture = TextureObject {
            image_view,
            vbo,
            set: set2
        };

        self.textures.insert(img_path.to_string(), texture);

        // 提交到全局纹理缓存
        map.add(image.clone(), staging.clone());
    }
}