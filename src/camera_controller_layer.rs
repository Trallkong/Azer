use azer::core::delta_time::DeltaTime;
use azer::core::event::Event;
use azer::core::input::InputState;
use azer::core::layer::Layer;
use azer::renderer::camera::camera2d::Camera2D;
use azer::renderer::camera::Camera;
use azer::renderer::image_buffer_man::ImageBufferManager;
use azer::renderer::renderer::Renderer;
use glam::Vec2;
use imgui::{Condition, Ui};
use log::info;
use winit::event::{MouseButton, WindowEvent};
use winit::keyboard::KeyCode;

pub struct NewLayer {
    pub camera: Camera2D,
    pub mouse_pos: (f64, f64),
    pub zoom_speed: u32,
    pub drag_speed: u32,
}

impl NewLayer {
    pub fn new() -> Self {
        let aspect_ratio = 16.0 / 9.0;
        let zoom = 1.0;
        let pos = Vec2::new(0.0, 0.0);
        Self {
            camera: Camera2D::new(aspect_ratio, zoom, -1.0, 1.0, pos),
            mouse_pos: (0.0, 0.0),
            zoom_speed: 1,
            drag_speed: 1,
        }
    }
}

impl Layer for NewLayer {
    fn on_ready(&mut self, _renderer: &mut Renderer) {
        info!("NewLayer ready");
    }

    fn on_update(&mut self, delta: &DeltaTime, input: &mut InputState) {
        let move_speed = 10.0;
        if input.is_key_pressed(KeyCode::KeyW) {
            self.camera.position.y += (move_speed * delta.as_seconds()) as f32;
        }
        if input.is_key_pressed(KeyCode::KeyS) {
            self.camera.position.y -= (move_speed * delta.as_seconds()) as f32;
        }
        if input.is_key_pressed(KeyCode::KeyA) {
            self.camera.position.x -= (move_speed * delta.as_seconds()) as f32;
        }
        if input.is_key_pressed(KeyCode::KeyD) {
            self.camera.position.x += (move_speed * delta.as_seconds()) as f32;
        }

        if input.is_mouse_pressed(MouseButton::Left) {
            let cur_pos = input.mouse_pos();
            let rx = cur_pos.0 - self.mouse_pos.0;
            let ry = cur_pos.1 - self.mouse_pos.1;
            self.camera.position.x -= rx as f32 * delta.as_seconds() as f32 * self.camera.zoom * self.drag_speed as f32;
            self.camera.position.y += ry as f32 * delta.as_seconds() as f32 * self.camera.zoom * self.drag_speed as f32;
            self.mouse_pos = cur_pos;
        } else {
            self.mouse_pos = input.mouse_pos();
        }


        self.camera.update();
    }

    fn on_render(&mut self, renderer: &mut Renderer, _map: &mut ImageBufferManager) {
        renderer.update_camera(*self.camera.get_view_projection_matrix());
    }

    fn on_imgui_render(&mut self, ui: &mut Ui) {
        ui.window("相机控制器")
            .size([300.0, 100.0],Condition::FirstUseEver)
            .build(|| {
                ui.slider("缩放速度", 1, 10, &mut self.zoom_speed);
                ui.slider("拖拽力度", 1, 10, &mut self.drag_speed);
            });
    }

    fn on_physics_update(&mut self, _delta: &DeltaTime) {
        // info!("NewLayer physics update");
    }

    fn on_event(&mut self, event: &Event) {
        let window_event = &event.event;

        match window_event {
            WindowEvent::MouseWheel {delta,phase, ..} => {
                if *phase == winit::event::TouchPhase::Started || *phase == winit::event::TouchPhase::Moved {
                    match delta {
                        winit::event::MouseScrollDelta::LineDelta(_, y) => {
                            self.camera.zoom -= y * 0.05 * self.camera.zoom * self.zoom_speed as f32;
                        },
                        winit::event::MouseScrollDelta::PixelDelta(pos) => {
                            self.camera.zoom -= pos.y as f32 * 0.05 * self.camera.zoom * self.zoom_speed as f32;
                        },
                    }
                }
            }
            _ => {}
        }
    }

    fn on_close(&mut self) {
        info!("NewLayer close");
    }
}