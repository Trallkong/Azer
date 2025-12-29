use crate::api::vulkan::{RenderDirty, Vulkan};
use crate::core::delta_time::DeltaTime;
use crate::core::layer::Layer;
use crate::core::layer_stack::LayerStack;
use crate::renderer::renderer::Renderer;
use log::info;
use std::sync::Arc;
use std::time::Instant;
use winit::application::ApplicationHandler;
use winit::event::WindowEvent;
use winit::event_loop::ActiveEventLoop;
use winit::window::{Window, WindowId};
use crate::core::input::InputState;

const FIXED_PHYSICS_STEP: f64 = 1.0/60.0; // 固定物理步长
const MAX_PHYSICS_STEPS: usize = 10; // 最大物理步次

enum AppState {
    Uninitialized {
       layer_stack: LayerStack,
    },
    Running {
        window: Arc<Window>,
        layer_stack: LayerStack,
        vulkan: Vulkan,
        renderer: Renderer,
        input_state: InputState,

        last_time: Instant,
        accumulated_time: f64,
    }
}

impl Default for AppState {
    fn default() -> Self {
        AppState::Uninitialized {
            layer_stack: LayerStack::new(),
        }
    }
}

pub struct Application {
    state: AppState,
}

impl ApplicationHandler for Application {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        info!("Resuming application");

        // 获取状态
        let state = std::mem::take(&mut self.state);

        if let AppState::Uninitialized { mut layer_stack } = state {
            // 创建窗口
            let window_attribute = Window::default_attributes()
                .with_title("Azer")
                .with_inner_size(winit::dpi::PhysicalSize::new(1280,720));

            let window = Arc::new(
                event_loop.create_window(window_attribute).unwrap()
            );

            // 创建 vulkan
            let vulkan = Vulkan::new(window.clone());

            // 创建渲染器
            let mut renderer = Renderer::new(
                vulkan.device.clone(),
                vulkan.queue.clone(),
                window.clone(),
                vulkan.render_pass.clone()
            );

            // 层栈初始化
            layer_stack.iter_mut().for_each(|layer| layer.on_ready(&mut renderer));

            // 初始化输入状态
            let input_state = InputState::default();

            // 切换运行状态
            self.state = AppState::Running {
                window,
                layer_stack,
                vulkan,
                renderer,
                last_time: Instant::now(),
                accumulated_time: 0.0,
                input_state,
            }
        }
    }
    fn window_event(&mut self, event_loop: &ActiveEventLoop, _window_id: WindowId, event: WindowEvent) {
        let AppState::Running {
            window,
            layer_stack,
            vulkan,
            renderer,
            input_state,
            ..
        } = &mut self.state else {
            return;
        };

        // 事件监听
        match &event {
            WindowEvent::CloseRequested => {
                info!("检测到点击关闭按钮，开始清理，请不要退出应用！");

                // 清理层栈
                layer_stack.iter_mut().for_each(|layer| layer.on_close());
                layer_stack.clear();

                event_loop.exit(); // 关闭事件循环
                info!("清理完毕！");
                return;
            },
            WindowEvent::RedrawRequested => {
                if vulkan.dirty.contains(RenderDirty::SWAPCHAIN) {
                    vulkan.recreate_swapchain(window.clone(), renderer);
                    vulkan.dirty.remove(RenderDirty::SWAPCHAIN);
                }

                vulkan.submit(renderer, layer_stack, [0.0,0.0,0.0,1.0]);
            },
            WindowEvent::Resized(_size) => {
                vulkan.dirty.insert(RenderDirty::SWAPCHAIN);
                vulkan.dirty.insert(RenderDirty::PIPELINE);
                vulkan.dirty.insert(RenderDirty::COMMAND_BUF);
            },
            WindowEvent::KeyboardInput {
                event, ..
            } => {
                if let winit::event::ElementState::Pressed = event.state {
                    input_state.update_key(event.physical_key, true);
                } else {
                    info!("检测到按键抬起，请不要退出应用！");
                    input_state.update_key(event.physical_key, false);
                }
            },
            WindowEvent::MouseInput {
                state, button, ..
            } => {
                if let winit::event::ElementState::Pressed = state {
                    input_state.update_mouse(*button, true);
                } else {
                    input_state.update_mouse(*button, false);
                }
            },
        WindowEvent::CursorMoved {
            position, ..
        } => {
            input_state.update_mouse_pos(position.x, position.y);
        }
            _ => ()
        }

        // 事件分发
        layer_stack.iter_mut().for_each(|layer| layer.on_event(&event));
    }

    fn about_to_wait(&mut self, event_loop: &ActiveEventLoop) {
        if event_loop.exiting() {
            return;
        }

        let AppState::Running {
            window,
            layer_stack,
            vulkan,
            renderer,
            last_time,
            accumulated_time,
            input_state,
        } = &mut self.state else {
            return;
        };

        // 逻辑更新
        let now = Instant::now();
        let dt = now.duration_since(*last_time).as_secs_f64().min(0.25);
        *last_time = now;

        physics_update(layer_stack, dt, accumulated_time);
        update(layer_stack, renderer,dt, input_state);

        vulkan.dirty.insert(RenderDirty::COMMAND_BUF);

        window.request_redraw();
    }
}

impl Application {
    pub fn new() -> Self {
        Self {
            state: AppState::Uninitialized {
                layer_stack: LayerStack::new(),
            }
        }
    }
    pub fn push_layer(&mut self, layer: Box<dyn Layer>) {
        match &mut self.state {
            AppState::Uninitialized { layer_stack } => {
                layer_stack.push(layer);
            },
            AppState::Running { layer_stack, .. } => {
                layer_stack.push(layer);
            }
        }
    }
}

pub fn physics_update(layer_stack: &mut LayerStack, duration: f64, accumulated_time: &mut f64) {
    let mut step: usize = 0;
    *accumulated_time += duration;
    while *accumulated_time > FIXED_PHYSICS_STEP && step < MAX_PHYSICS_STEPS {
        step += 1;
        *accumulated_time -= FIXED_PHYSICS_STEP;
        layer_stack.iter_mut().for_each(|layer| {
            layer.on_physics_update(&DeltaTime::new(FIXED_PHYSICS_STEP));
        })
    }
}

pub fn update(layer_stack: &mut LayerStack, renderer: &mut Renderer, duration: f64, input: &mut InputState) {
    layer_stack.iter_mut().for_each(|layer| {
        layer.on_update(&DeltaTime::new(duration), renderer, input);
    });
}