use std::sync::Arc;

use winit::window::Window;

use crate::{core::layer_stack, renderer::render_context::RenderContext};

#[derive(Default)]
pub struct InitializingData { 
    pub layer_stack: layer_stack::LayerStack,
}

pub struct RunningData {
    pub window: Arc<Window>,
    pub render_context: RenderContext,
    pub layer_stack: layer_stack::LayerStack,
    pub renderer: crate::renderer::renderer::Renderer,
    pub input_state: crate::core::input::InputState,

    pub last_frame_time: std::time::Instant,
}

pub enum AppState {
    Initializing(InitializingData),
    Running(RunningData),
    Stopped,
}

impl Default for AppState {
    fn default() -> Self {
        AppState::Initializing(InitializingData::default())
    }
}