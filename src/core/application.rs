use crate::api::vulkan::{RenderDirty, Vulkan};
use crate::core::delta_time::DeltaTime;
use crate::core::input::InputState;
use crate::core::layer::Layer;
use crate::core::layer_stack::LayerStack;
use crate::renderer::image_buffer_man::ImageBufferManager;
use crate::renderer::renderer::Renderer;
use log::info;
use std::sync::Arc;
use std::time::Instant;
use imgui::{Condition, FontConfig, FontSource};
use winit::application::ApplicationHandler;
use winit::event::WindowEvent;
use winit::event_loop::ActiveEventLoop;
use winit::window::{Window, WindowId};
use crate::core::event::Event;
use crate::ui;
use crate::ui::imgui_renderer::ImGuiRenderer;

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
        map: ImageBufferManager,
        imgui: imgui::Context,
        imgui_renderer: ImGuiRenderer,

        last_time: Instant,
        accumulated_time: f64,
        clear_color: [f32; 4]
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

            let mut map = ImageBufferManager::default();

            // 创建渲染器
            let mut renderer = Renderer::new(
                vulkan.device.clone(),
                vulkan.queue.clone(),
                window.clone(),
                vulkan.render_pass.clone(),
                &mut map
            );

            // 层栈初始化
            layer_stack.iter_mut().for_each(|layer| layer.on_ready(&mut renderer));

            // 初始化输入状态
            let input_state = InputState::default();

            // 初始化 ImGui
            let font_size = 24.0;
            let mut imgui = imgui::Context::create();
            imgui.fonts().add_font(&[FontSource::DefaultFontData {
                config: Some(FontConfig {
                    size_pixels: font_size,
                    ..FontConfig::default()
                })
            }]);

            let imgui_renderer = ImGuiRenderer::new(
                window.clone(),
                vulkan.device.clone(),
                vulkan.render_pass.clone(),
                renderer.allocators.buffer_allocator.clone(),
                renderer.allocators.descriptor_set_allocator.clone(),
                &mut imgui,
                &mut map
            );

            // 切换运行状态
            self.state = AppState::Running {
                window,
                layer_stack,
                vulkan,
                renderer,
                last_time: Instant::now(),
                accumulated_time: 0.0,
                input_state,
                map,
                imgui,
                imgui_renderer,
                clear_color: [0.0,0.0,0.0,1.0]
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
            map,
            imgui,
            imgui_renderer,
            clear_color,
            ..
        } = &mut self.state else {
            return;
        };

        // 系统事件处理
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

                let size = window.inner_size();

                if size.width == 0 || size.height == 0 {
                    return;
                }

                let scale = window.scale_factor() as f32;

                let io = imgui.io_mut();
                io.display_size = [
                    size.width as f32 / scale,
                    size.height as f32 / scale,
                ];

                let ui = imgui.frame();


                ui.window("Azer Core")
                    .size([300.0, 100.0], Condition::FirstUseEver)
                    .build(|| {
                        ui.text("Hello, world!");
                        ui.button("Click me!");
                        ui.color_edit4("Clear Color", clear_color);
                    });

                layer_stack.iter_mut().for_each(|layer| {
                    layer.on_imgui_render(ui);
                });

                let draw_data = imgui.render();

                vulkan.submit(renderer, layer_stack, *clear_color, map, imgui_renderer, draw_data);
            },
            WindowEvent::Resized(_size) => {
                vulkan.dirty.insert(RenderDirty::SWAPCHAIN);
                vulkan.dirty.insert(RenderDirty::PIPELINE);
                vulkan.dirty.insert(RenderDirty::COMMAND_BUF);
            },
            _ => ()
        }

        let mut wrapped_event = Event {
            event: &event,
            handled: false
        };

        // 处理 ImGui 事件
        ui::imgui_winit_support::handle_event(imgui, &mut wrapped_event);

        if wrapped_event.handled {
            return;
        }

        // 分发给各层
        for layer in layer_stack.iter_mut().rev() {
            layer.on_event(&mut wrapped_event);

            if wrapped_event.handled {
                break;
            }
        }

        // 输入事件处理
        if !wrapped_event.handled {
            match &event {
                WindowEvent::KeyboardInput {
                    event, ..
                } => {
                    if let winit::event::ElementState::Pressed = event.state {
                        input_state.update_key(event.physical_key, true);
                    } else {
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
        }
    }

    fn about_to_wait(&mut self, event_loop: &ActiveEventLoop) {
        if event_loop.exiting() {
            return;
        }

        let AppState::Running {
            window,
            layer_stack,
            vulkan,

            last_time,
            accumulated_time,
            input_state,
            ..
        } = &mut self.state else {
            return;
        };

        // 逻辑更新
        let now = Instant::now();
        let dt = now.duration_since(*last_time).as_secs_f64().min(0.25);
        *last_time = now;

        physics_update(layer_stack, dt, accumulated_time);
        update(layer_stack, dt, input_state);

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

const PHYSICS_DELTA: DeltaTime = DeltaTime::new_const(FIXED_PHYSICS_STEP);
pub fn physics_update(layer_stack: &mut LayerStack, duration: f64, accumulated_time: &mut f64) {
    let mut step: usize = 0;
    *accumulated_time += duration;
    while *accumulated_time > FIXED_PHYSICS_STEP && step < MAX_PHYSICS_STEPS {
        step += 1;
        *accumulated_time -= FIXED_PHYSICS_STEP;
        layer_stack.iter_mut().for_each(|layer| {
            layer.on_physics_update(&PHYSICS_DELTA);
        })
    }
}

pub fn update(layer_stack: &mut LayerStack, duration: f64, input: &mut InputState) {
    layer_stack.iter_mut().for_each(|layer| {
        layer.on_update(&DeltaTime::new(duration), input);
    });
}