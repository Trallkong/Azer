use winit::event::WindowEvent;

#[derive(Debug, Clone)]
pub struct Event<'a> {
    pub event: &'a WindowEvent,
    pub handled: bool,
}