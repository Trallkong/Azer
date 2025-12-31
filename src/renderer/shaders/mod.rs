use std::fmt::Debug;
use std::sync::Arc;
use vulkano::device::Device;
use vulkano::shader::ShaderModule;
use vulkano::{Validated, VulkanError};
pub mod upgrade_shader;

pub trait Shader {

    fn fs(&self) -> &Arc<ShaderModule>;
    fn vs(&self) -> &Arc<ShaderModule>;
    fn load(device: Arc<Device>) -> Result<Self, Validated<VulkanError>> where Self: Sized + Clone + Debug;
}