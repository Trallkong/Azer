use crate::renderer::frame_commands::FrameCommands;
use std::sync::Arc;
use vulkano::buffer::Subbuffer;
use vulkano::command_buffer::CopyBufferToImageInfo;
use vulkano::image::Image;

pub struct ImageAndBuffer {
    pub image: Arc<Image>,
    pub buffer: Subbuffer<[u8]>
}

#[derive(Default)]
pub struct ImageBufferManager {
    pub images_and_buffers: Vec<ImageAndBuffer>
}

impl ImageBufferManager {

    pub fn add(&mut self, image: Arc<Image>, buffer: Subbuffer<[u8]>) {
        self.images_and_buffers.push(ImageAndBuffer { image, buffer })
    }

    pub fn copy_all_buffer_to_image(&mut self, frame: &mut FrameCommands) {
        self.images_and_buffers.iter().for_each(|x| {
            frame.builder
                .copy_buffer_to_image(CopyBufferToImageInfo::buffer_image(
                    x.buffer.clone(),
                    x.image.clone()
                ))
                .unwrap();
        });
    }
}