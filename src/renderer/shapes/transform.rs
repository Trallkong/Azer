use glam::{Mat4, Quat, Vec2, Vec3};

#[derive(Copy, Clone)]
pub struct Transform2D {
    pub position: Vec2,
    pub rotation: Quat,
    pub scale: Vec2,
}

impl Default for Transform2D {
    fn default() -> Self {
        Self {
            position: Vec2::new(0.0,0.0),
            rotation: Quat::IDENTITY,
            scale: Vec2::new(1.0,1.0)
        }
    }
}

impl Transform2D {
    pub fn to_mat4(&self) -> Mat4 {
        Mat4::from_scale_rotation_translation(
            Vec3::new(self.scale.x, self.scale.y, 1.0),
            self.rotation,
            Vec3::new(self.position.x, self.position.y, 0.0),
        )
    }
}