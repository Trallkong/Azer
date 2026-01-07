pub use crate::core::delta_time::DeltaTime;
use crate::core::event::Event;
use crate::core::input::InputState;
use crate::renderer::image_buffer_man::ImageBufferManager;
use crate::renderer::renderer::Renderer;
use imgui::Ui;
pub use winit::event::WindowEvent;

pub trait Layer: Send + Sync {
    fn on_ready(&mut self, renderer: &mut Renderer);
    fn on_update(&mut self, delta: &DeltaTime, input: &mut InputState);
    fn on_render(&mut self, renderer: &mut Renderer, map: &mut ImageBufferManager);
    fn on_imgui_render(&mut self, ui: &mut Ui);
    fn on_physics_update(&mut self, delta: &DeltaTime);
    fn on_event(&mut self, event: &Event);
    fn on_close(&mut self);
}