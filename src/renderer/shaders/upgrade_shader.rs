use crate::renderer::shaders::Shader;
use glam::Mat4;
use std::fmt::Debug;
use std::sync::Arc;
use vulkano::buffer::{BufferContents, Subbuffer};
use vulkano::descriptor_set::DescriptorSet;
use vulkano::device::Device;
use vulkano::shader::ShaderModule;
use vulkano::{Validated, VulkanError};

mod vs {
    vulkano_shaders::shader! {
        ty: "vertex",
        src: r"
            #version 460

            layout(location = 0) in vec2 position;
            layout(location = 1) in vec2 uv;

            layout(set = 0, binding = 0) uniform CameraData {
                mat4 view_proj;
            } camera;

            struct Instance {
                mat4 transform;
                vec4 color;
            };

            layout(set = 0, binding = 1) buffer Instances {
                Instance instances[1000];
            } ds;

            layout(location = 0) out vec2 v_uv;
            layout(location = 1) out vec4 v_color;

            void main() {
                mat4 model = ds.instances[gl_InstanceIndex].transform;
                v_uv = uv;
                v_color = ds.instances[gl_InstanceIndex].color;
                gl_Position = camera.view_proj * model * vec4(position, 0.0, 1.0);
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

            layout(set = 1, binding = 0) uniform sampler2D tex;

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
pub struct CameraData {
    pub view_proj: [[f32;4];4],
}

impl Default for CameraData {
    fn default() -> Self {
        CameraData {
            view_proj: [[1.0;4];4],
        }
    }
}

#[repr(C)]
#[derive(BufferContents, Copy, Clone)]
pub struct Instance {
    pub transform: [[f32;4];4],
    pub color: [f32;4],
}

impl Default for Instance {
    fn default() -> Self {
        Instance {
            transform: [[1.0;4];4],
            color: [1.0;4],
        }
    }
}

#[repr(C)]
#[derive(BufferContents, Copy, Clone)]
pub struct Instances {
    pub instances: [Instance; 1000],
}

impl Default for Instances {
    fn default() -> Self {
        Instances {
            instances: [Instance::default(); 1000],
        }
    }
}

pub struct ShaderData {
    pub camera_buffer: Subbuffer<CameraData>,
    pub instances_buffer: Subbuffer<Instances>,
    pub instance_index: usize,
    pub set: Arc<DescriptorSet>
}

impl ShaderData {
    pub fn update_camera_buffer(&mut self, view_proj: Mat4) {
        self.camera_buffer.write().unwrap().view_proj = view_proj.to_cols_array_2d();
    }

    pub fn update_instances_buffer(&mut self, index: usize ,instance: Instance) {
        self.instances_buffer.write().unwrap().instances[index] = instance;
    }

    pub fn add_instance(&mut self, instance: Instance) {
        self.instances_buffer.write().unwrap().instances[self.instance_index] = instance;
        self.instance_index += 1;
    }

    pub fn begin_frame(&mut self) {
        self.instance_index = 0;
    }
}