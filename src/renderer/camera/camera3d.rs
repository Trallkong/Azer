use crate::renderer::camera::Camera;
use glam::{Mat4, Vec3};

pub struct Camera3D {
    proj: Mat4,
    view: Mat4,
    view_proj: Mat4,
    pub position: Vec3,
    pub rotation: Vec3,
    pub fov: f32,
    pub aspect_ratio: f32,
    pub z_near: f32,
    pub z_far: f32,
}

impl Camera3D {
    pub fn new(fov: f32, aspect_ratio: f32, z_near: f32, z_far: f32) -> Self {
        let proj = Mat4::perspective_rh(fov, aspect_ratio, z_near, z_far);
        let position = Vec3::new(0.0,0.0,0.0);
        let view = Mat4::look_at_rh(position, Vec3::new(0.0,0.0,0.0), Vec3::new(0.0,1.0,0.0));
        let rotation = Vec3::new(0.0,0.0,0.0);

        Self {
            proj,
            view,
            view_proj: proj * view,
            position,
            rotation,
            fov,
            aspect_ratio,
            z_near,
            z_far
        }
    }
}

impl Camera for Camera3D {
    fn get_projection_matrix(&self) -> &Mat4 {
        &self.proj
    }

    fn get_view_matrix(&self) -> &Mat4 {
        &self.view
    }

    fn get_view_projection_matrix(&self) -> &Mat4 {
        &self.view_proj
    }

    fn update(&mut self) {
        self.proj = Mat4::perspective_rh(self.fov, self.aspect_ratio, self.z_near, self.z_far);
        self.view = Mat4::look_at_rh(self.position, Vec3::new(0.0,0.0,0.0), Vec3::new(0.0,1.0,0.0));
        self.view_proj = self.proj * self.view;
    }
}