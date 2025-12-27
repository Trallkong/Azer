use glam::{Mat4, Vec2, Vec3};
use crate::render::camera::Camera;

#[derive(Clone)]
pub struct Camera2D {
    aspect_ratio: f32,
    zoom: f32,
    near: f32,
    far: f32,
    position: Vec2,

    view_matrix: Mat4,
    projection_matrix: Mat4,
    view_projection_matrix: Mat4,
}

impl Camera2D {
    pub fn new(aspect_ratio: f32, zoom: f32, near: f32, far: f32, position: Vec2) -> Self {

        let projection_matrix = Self::get_projection_matrix(aspect_ratio, zoom, near, far);
        let view_matrix = Self::get_view_matrix(position);

        Self {
            aspect_ratio,
            zoom,
            near,
            far,
            position,

            view_matrix,
            projection_matrix,
            view_projection_matrix: projection_matrix * view_matrix,
        }
    }

    pub fn set_position(&mut self, position: Vec2) {
        self.position = position;
    }

    pub fn position(&self) -> Vec2 {
        self.position
    }

    pub fn set_aspect_ratio(&mut self, aspect_ratio: f32) {
        self.aspect_ratio = aspect_ratio;
    }
    pub fn set_zoom(&mut self, zoom: f32) {
        self.zoom = zoom;
    }

    fn get_projection_matrix(aspect_ratio: f32, zoom: f32, near: f32, far: f32) -> Mat4 {
        let mut m = Mat4::orthographic_rh(-aspect_ratio * zoom, aspect_ratio * zoom, -zoom, zoom, near, far);
        m.y_axis.y *= -1.0;
        m
    }

    fn get_view_matrix(position: Vec2) -> Mat4 {
        Mat4::from_translation(Vec3::new(position.x, position.y, 0.0)).inverse()
    }
}

impl Camera for Camera2D {
    fn get_projection_matrix(&self) -> &Mat4 {
        &self.projection_matrix
    }

    fn get_view_matrix(&self) -> &Mat4 {
        &self.view_matrix
    }

    fn get_view_projection_matrix(&self) -> &Mat4 {
        &self.view_projection_matrix
    }

    fn update(&mut self) {
        self.projection_matrix = Self::get_projection_matrix(self.aspect_ratio, self.zoom, self.near, self.far);
        self.view_matrix = Self::get_view_matrix(self.position);
        self.view_projection_matrix = self.projection_matrix * self.view_matrix;
    }
}