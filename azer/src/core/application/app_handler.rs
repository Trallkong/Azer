use std::sync::Arc;

use pollster::block_on;
use winit::{application::ApplicationHandler, event_loop, window::Window};

use crate::{core::{application::{Application, app_state::{AppState, InitializingData, RunningData}}, input::InputState}, renderer::{render_context::RenderContext, renderer::Renderer}};

impl ApplicationHandler for Application {
    /// 应用启动
    fn resumed(&mut self, event_loop: &event_loop::ActiveEventLoop) {

        let state = std::mem::take(&mut self.state);
        let AppState::Initializing(InitializingData {
            layer_stack
        }) = state else {
            panic!("Invalid state")
        };

        let window_attrib = Window::default_attributes()
            .with_title("Azer")
            .with_inner_size(winit::dpi::LogicalSize::new(1270, 720));

        let window = event_loop.create_window(window_attrib).unwrap();
        let window = Arc::new(window);

        let render_context = block_on(RenderContext::new(window.clone())).unwrap();
        let renderer = Renderer::new(&render_context);

        self.state = AppState::Running(RunningData { 
            window: window.clone(), 
            render_context, 
            layer_stack, 
            renderer, 
            input_state: InputState::default(),
            last_frame_time: std::time::Instant::now()
        });
    }

    /// 应用事件
    fn window_event(
        &mut self,
        event_loop: &event_loop::ActiveEventLoop,
        _window_id: winit::window::WindowId,
        event: winit::event::WindowEvent,
    ) {
        match &event {
            winit::event::WindowEvent::CloseRequested => {
                event_loop.exit();
            },
            winit::event::WindowEvent::Resized(size) => {
                self.resize(size.width, size.height);
            },
            winit::event::WindowEvent::RedrawRequested => {
                self.update();
                match self.render() {
                    Ok(_) => {},
                    Err(wgpu::SurfaceError::Lost | wgpu::SurfaceError::Outdated) => {
                        if let AppState::Running(data) = &mut self.state {
                            let window = data.window.clone();
                            let size = window.inner_size();
                            self.resize(size.width, size.height);
                        } else {
                            panic!("Invalid state")
                        }
                    }
                    Err(e) => {
                        log::error!("Unable to render {}", e);
                    }
                }
            },
            _ => ()
        }

        if let AppState::Running(data) = &mut self.state {
           update_input_state(&mut data.input_state, &event);
        }

        let my_event = crate::core::event::Event::from_winit_event(&event);
        if let Some(my_event) = my_event {
            self.event(&my_event);
        }
    }
}


pub fn update_input_state(input_state: &mut InputState, event: &winit::event::WindowEvent) {
    match event {
        winit::event::WindowEvent::KeyboardInput { event: key_event, .. } => {
            input_state.update_key(key_event.physical_key, key_event.state.is_pressed());
        },
        winit::event::WindowEvent::MouseInput { state, button, .. } => {
            input_state.update_mouse(*button, state.is_pressed());
        },
        winit::event::WindowEvent::CursorMoved { position, .. } => {
            input_state.update_mouse_pos(position.x, position.y);
        },
        _ => {}
    }
}