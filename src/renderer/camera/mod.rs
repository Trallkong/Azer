use crate::renderer::shaders::sd_camera2d::Cam2dShaderData;
use crate::renderer::shapes::transform::Transform2D;
use glam::{Mat4, Vec3};
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