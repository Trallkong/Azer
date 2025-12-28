use crate::api::shaders::Shader;
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

            layout(location = 0) in vec2 position;
            layout(set = 0, binding = 0) uniform Data {
                mat4 view_proj;
                mat4 transform;
                vec4 color;
            } ubo;

            layout(location = 0) out vec4 v_Color;

            void main() {
                gl_Position = ubo.view_proj * ubo.transform * vec4(position, 0.0, 1.0);
                v_Color = ubo.color;
            }
        ",
    }
}

mod fs {
    vulkano_shaders::shader! {
        ty: "fragment",
        src: r"
            #version 460

            layout(location = 0) in vec4 v_Color;
            layout(location = 0) out vec4 f_Color;

            void main() {
                f_Color = v_Color;
            }
        "
    }
}

#[derive(Clone, Debug)]
pub struct ShaderCamera2D {
    pub vs: Arc<ShaderModule>,
    pub fs: Arc<ShaderModule>
}

impl Shader for ShaderCamera2D {
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
            fs: fs::load(device.clone())?
        })
    }
}

#[repr(C)]
#[derive(BufferContents, Clone, Copy)]
pub struct Cam2dShaderData {
    pub view_proj: [[f32; 4]; 4],
    pub transform: [[f32; 4]; 4],
    pub color: [f32; 4]
}

impl Default for Cam2dShaderData {
    fn default() -> Self {
        Self {
            view_proj: [[1.0; 4]; 4],
            transform: [[1.0; 4]; 4],
            color: [0.5; 4]
        }
    }
}