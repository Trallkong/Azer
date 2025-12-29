pub use crate::core::delta_time::DeltaTime;
use crate::renderer::renderer::Renderer;
pub use winit::event::WindowEvent;
use crate::core::input::InputState;

pub trait Layer: Send + Sync {
    fn on_ready(&mut self, renderer: &mut Renderer);
    fn on_update(&mut self, delta: &DeltaTime, renderer: &mut Renderer, input: &mut InputState);
    fn on_render(&mut self, renderer: &mut Renderer);
    fn on_physics_update(&mut self, delta: &DeltaTime);
    fn on_event(&mut self, event: &WindowEvent);
    fn on_close(&mut self);
}