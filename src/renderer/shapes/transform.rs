use glam::{Mat4, Quat, Vec3};

#[derive(Copy, Clone)]
pub struct Transform {
    pub position: Vec3,
    pub rotation: Quat,
    pub scale: Vec3,
}

impl Default for Transform {
    fn default() -> Self {
        Self {
            position: Vec3::new(0.0,0.0, 0.0),
            rotation: Quat::IDENTITY,
            scale: Vec3::new(1.0,1.0, 1.0)
        }
    }
}

impl Transform {
    pub fn to_mat4(&self) -> Mat4 {
        Mat4::from_scale_rotation_translation(
            Vec3::new(self.scale.x, self.scale.y, 1.0),
            self.rotation,
            Vec3::new(self.position.x, self.position.y, 0.0),
        )
    }
}