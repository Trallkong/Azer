use crate::renderer::shaders::Shader;
use std::fmt::Debug;
use std::sync::Arc;
use vulkano::buffer::BufferContents;
use vulkano::device::Device;
use vulkano::shader::ShaderModule;
use vulkano::{Validated, VulkanError};

mod vs {
    vulkano_shaders::shader! {
        ty: "vertex",
        src: r"
            #version 460

            layout(location = 0) in vec4 position;
            layout(location = 1) in vec2 uv;
            layout(location = 2) in vec4 color;

            layout(push_constant) uniform PushConstants {
                mat4 view_proj;
                mat4 transform;
            } pc;

            layout(location = 0) out vec2 v_uv;
            layout(location = 1) out vec4 v_color;

            void main() {
                v_uv = uv;
                v_color = color;
                gl_Position = pc.view_proj * pc.transform * position;
            }
        "
    }
}

mod fs {
    vulkano_shaders::shader! {
        ty: "fragment",
        src: r"
            #version 460

            layout(location = 0) in vec2 v_uv;
            layout(location = 1) in vec4 v_color;

            layout(set = 0, binding = 0) uniform sampler2D tex;

            layout(location = 0) out vec4 f_color;

            void main() {
                f_color = texture(tex, v_uv) * v_color;
            }
        "
    }
}

#[derive(Clone, Debug)]
pub struct UpgradeShader {
    pub vs: Arc<ShaderModule>,
    pub fs: Arc<ShaderModule>,
}

impl Shader for UpgradeShader {
    fn fs(&self) -> &Arc<ShaderModule> {
        &self.fs
    }

    fn vs(&self) -> &Arc<ShaderModule> {
        &self.vs
    }

    fn load(device: Arc<Device>) -> Result<Self, Validated<VulkanError>>
    where
        Self: Sized + Clone + Debug
    {
        Ok(Self {
            vs: vs::load(device.clone())?,
            fs: fs::load(device.clone())?,
        })
    }
}

#[repr(C)]
#[derive(BufferContents, Copy, Clone)]
pub struct PushConstants {
    pub view_proj: [[f32;4];4],
    pub transform: [[f32;4];4],
}