use crate::api::vulkan_helper;
use crate::core::core::{new_ref, Ref};
use crate::renderer::image_buffer_man::ImageBufferManager;
use crate::renderer::shapes::shape_2d::image_rect::ImageRect;
use glam::Vec2;
use log::{error, info};
use std::collections::HashMap;
use std::sync::Arc;
use vulkano::descriptor_set::allocator::StandardDescriptorSetAllocator;
use vulkano::descriptor_set::layout::DescriptorSetLayout;
use vulkano::descriptor_set::{DescriptorSet, WriteDescriptorSet};
use vulkano::device::Device;
use vulkano::format::Format;
use vulkano::image::sampler::{Sampler, SamplerCreateInfo};
use vulkano::image::view::ImageView;
use vulkano::memory::allocator::StandardMemoryAllocator;

pub struct ImageObject {
    pub set: Arc<DescriptorSet>,
    pub width: u32,
    pub height: u32
}

pub struct RenderImage {
    images: HashMap<String, Ref<ImageObject>>,

    set_layout: Arc<DescriptorSetLayout>,

    memory_allocator: Arc<StandardMemoryAllocator>,
    set_allocator: Arc<StandardDescriptorSetAllocator>,

    sampler: Arc<Sampler>
}

impl RenderImage {
    pub fn new(
        device: Arc<Device>,
        set_layout: Arc<DescriptorSetLayout>,
        memory_allocator: Arc<StandardMemoryAllocator>,
        set_allocator: Arc<StandardDescriptorSetAllocator>,
    ) -> Self {
        let sampler = Sampler::new(
            device.clone(),
            SamplerCreateInfo::simple_repeat_linear(),
        ).unwrap();

        Self {
            images: HashMap::new(),
            set_layout: set_layout.clone(),
            memory_allocator: memory_allocator.clone(),
            set_allocator: set_allocator.clone(),
            sampler
        }
    }

    pub fn import_image(&mut self, img_path: &str, map: &mut ImageBufferManager) -> Option<ImageRect> {
        if let Some(obj) = self.images.get(img_path) {
            let w = obj.borrow().width as f32;
            let h = obj.borrow().height as f32;
            return Some(ImageRect::new(Vec2::new(w, h)));
        }

        let img = image::open(img_path);

        if img.is_err() {
            error!("failed to open image: {}", img_path);
            return None;
        }

        let img = img.unwrap();

        let img = img.to_rgba8();
        let (width, height) = img.dimensions();

        let pixels = img.into_raw();
        assert_eq!(pixels.len(), (width * height * 4) as usize);

        info!("importing: {}", img_path);

        let image = vulkan_helper::get_texture_image_2d((width,height), Format::R8G8B8A8_UNORM, self.memory_allocator.clone());

        let staging = vulkan_helper::get_staging(pixels, self.memory_allocator.clone());

        let image_view = ImageView::new_default(image.clone()).unwrap();

        let set = DescriptorSet::new(
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

        let obj = ImageObject {
            set,
            width,
            height
        };

        self.images.insert(img_path.to_string(), new_ref(obj));

        // 提交到全局纹理缓存
        map.add(image.clone(), staging);

        Some(ImageRect::new(Vec2::new(width as f32, height as f32)))
    }

    pub fn set_sampler(&self, img_path: &str) -> Arc<DescriptorSet> {
        self.images.get(img_path).unwrap().borrow().set.clone()
    }
}