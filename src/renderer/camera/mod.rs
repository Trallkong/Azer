use crate::api::shaders::sd_camera2d::Cam2dShaderData;
use glam::Mat4;
use std::sync::Arc;
use vulkano::buffer::Subbuffer;
use vulkano::descriptor_set::DescriptorSet;

pub mod camera2d;

pub trait Camera {
    fn get_projection_matrix(&self) -> &Mat4;
    fn get_view_matrix(&self) -> &Mat4;
    fn get_view_projection_matrix(&self) -> &Mat4;

    fn update(&mut self);
}

pub struct Camera2dUniform {
    pub buffer: Subbuffer<Cam2dShaderData>,
    pub set: Arc<DescriptorSet>
}

impl Camera2dUniform {
    pub fn update(&self, view_proj_matrix: Mat4) {
        self.buffer.write().unwrap().view_proj = view_proj_matrix.to_cols_array_2d();
    }
}