use glam::Mat4;

pub mod camera2d;

pub trait Camera {
    fn get_projection_matrix(&self) -> &Mat4;
    fn get_view_matrix(&self) -> &Mat4;
    fn get_view_projection_matrix(&self) -> &Mat4;

    fn update(&mut self);
}