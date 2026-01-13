use crate::core::{delta_time::DeltaTime, event::Event, input::InputState};

pub trait Layer {
    fn on_attach(&mut self) {}

    fn on_detach(&mut self) {}

    fn update(&mut self, _delta_time: &DeltaTime, _input: &InputState) {}

    fn event(&mut self, _event: &Event) {}

    fn render(&self, _renderer: &mut crate::renderer::renderer::Renderer, _render_pass: &mut crate::core::application::RenderPass) {}
}