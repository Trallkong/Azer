use crate::renderer::frame_commands::FrameCommands;
use std::sync::Arc;
use log::info;
use vulkano::buffer::Subbuffer;
use vulkano::command_buffer::CopyBufferToImageInfo;
use vulkano::image::Image;

pub struct ImageAndBuffer {
    pub image: Arc<Image>,
    pub buffer: Subbuffer<[u8]>,
    pub uploaded: bool
}

#[derive(Default)]
pub struct ImageBufferManager {
    pub items: Vec<ImageAndBuffer>,
}

impl ImageBufferManager {

    pub fn add(&mut self, image: Arc<Image>, buffer: Subbuffer<[u8]>) {
        info!("push {}", buffer.size());
        self.items.push(ImageAndBuffer { image, buffer, uploaded: false });
    }

    pub fn copy_all_buffer_to_image(&mut self, frame: &mut FrameCommands) {
        for (i, item) in self.items.iter_mut().enumerate() {
            if item.uploaded {
                continue;
            }

            info!("copying item {i} into gpu image");

            frame.builder
                .copy_buffer_to_image(
                    CopyBufferToImageInfo::buffer_image(
                        item.buffer.clone(),
                        item.image.clone()
                    )
                )
                .expect("copy_buffer_to_image failed");

            item.uploaded = true
        }
    }
}