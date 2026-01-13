use log::error;

/// 应用程序核心逻辑
/// 使用状态机模式，对于每个状态的定义存放在 app_state 模块中，
/// 应用程序对 winit 的实现存放在 app_handler 模块中

use crate::{core::{application::app_state::AppState, layer}, renderer::render_command};

pub mod app_state;
pub mod app_handler;

pub use wgpu::RenderPass;

#[derive(Default)]
pub struct Application {
    pub state: AppState,
}

impl Application {

    pub fn resize(&mut self, width: u32, height: u32) {
        if width == 0 || height == 0 {
            return;
        }

        if let AppState::Running(data) = &mut self.state {
            data.render_context.config.width = width;
            data.render_context.config.height = height;
            data.render_context.surface.configure(&data.render_context.device, &data.render_context.config);
            data.render_context.is_surface_configured = true;
        }
    }

    pub fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
        if let AppState::Running(data) = &mut self.state {
            // Rendering logic here
            data.window.request_redraw();

            if !data.render_context.is_surface_configured {
                return Ok(());
            }

            let output = data.render_context.surface.get_current_texture()?;

            let view = output.texture.create_view(&wgpu::TextureViewDescriptor::default());
            let mut encoder = data.render_context.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            });

            {
                data.renderer.begin_render();

                let mut render_pass = render_command::clear(&mut encoder, &view, &[0.1, 0.2, 0.3, 1.0]);

                data.layer_stack.layers().for_each(|layer| {
                    layer.render(&mut data.renderer, &mut render_pass);
                });

                data.renderer.end_render();
            }

            data.render_context.queue.submit(std::iter::once(encoder.finish()));
            output.present();

            Ok(())
        } else {
            error!("请先初始化应用程序");
            Err(wgpu::SurfaceError::Lost)
        }
    }

    pub fn update(&mut self) {
        if let AppState::Running(data) = &mut self.state {
            let current_frame_time = std::time::Instant::now();
            let delta_time = current_frame_time.duration_since(data.last_frame_time).as_secs_f32();
            data.last_frame_time = current_frame_time;
            data.layer_stack.layers_mut().for_each(|layer| layer.update(&crate::core::delta_time::DeltaTime::new(delta_time), &data.input_state));
        }
    }

    pub fn push_layer(&mut self, mut layer: Box<dyn layer::Layer>) {
        match &mut self.state {
            AppState::Initializing(data) => {
                layer.on_attach();
                data.layer_stack.push_layer(layer);
            },
            AppState::Running(data) => {
                layer.on_attach();
                data.layer_stack.push_layer(layer);
            },
            AppState::Stopped => todo!(),
        }
    }

    pub fn event(&mut self, event: &crate::core::event::Event) {
        match &mut self.state {
            AppState::Initializing(data) => {
                data.layer_stack.layers_mut().rev().for_each(|layer| {
                    layer.event(event);
                    if event.handled.get() {
                        return;
                    }
                });
            },
            AppState::Running(data) => {
                data.layer_stack.layers_mut().rev().for_each(|layer| {
                    layer.event(event);
                    if event.handled.get() {
                        return;
                    }
                });
            },
            AppState::Stopped => todo!(),
        }
    }
}