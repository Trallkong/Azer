use azer::core::delta_time::DeltaTime;
use azer::core::layer::Layer;
use azer::renderer::camera::camera2d::Camera2D;
use azer::renderer::camera::Camera;
use azer::renderer::frame_commands::FrameCommands;
use azer::renderer::renderer::Renderer;
use glam::Vec2;
use log::info;
use std::sync::{Arc, Mutex};
use winit::event::WindowEvent;
use winit::keyboard::{KeyCode, PhysicalKey};

pub struct NewLayer {
    pub camera: Camera2D
}

impl NewLayer {
    pub fn new() -> Self {
        let aspect_ratio = 16.0 / 9.0;
        let zoom = 1.0;
        let pos = Vec2::new(0.0, 0.0);
        Self {
            camera: Camera2D::new(aspect_ratio, zoom, -1.0, 1.0, pos),
        }
    }
}

impl Layer for NewLayer {
    fn on_ready(&mut self, renderer: &mut Renderer) {
        info!("NewLayer ready");
    }

    fn on_update(&mut self, _delta: &DeltaTime, renderer: &mut Renderer) {
        self.camera.update();
        renderer.update_camera(*self.camera.get_view_projection_matrix());
    }

    fn on_render(&mut self, renderer: &mut Renderer, frame: &mut FrameCommands) {
        renderer.draw_rectangle(frame);
        renderer.draw_triangle(frame);
    }

    fn on_physics_update(&mut self, _delta: &DeltaTime) {
        // info!("NewLayer physics update");
    }

    fn on_event(&mut self, event: &WindowEvent) {
        match event {
            WindowEvent::KeyboardInput {
                device_id,
                event,
                is_synthetic,
            } => {
                let cam_pos = self.camera.position();
                let move_speed = 0.2;

                if event.physical_key == PhysicalKey::Code(KeyCode::KeyA) {
                    info!("camera move left");
                    self.camera.set_position(Vec2::new(cam_pos.x - move_speed, cam_pos.y));
                }

                if event.physical_key == PhysicalKey::Code(KeyCode::KeyD) {
                    info!("camera move right");
                    self.camera.set_position(Vec2::new(cam_pos.x + move_speed, cam_pos.y));
                }

                if event.physical_key == PhysicalKey::Code(KeyCode::KeyW) {
                    info!("camera move up");
                    self.camera.set_position(Vec2::new(cam_pos.x, cam_pos.y + move_speed));
                }

                if event.physical_key == PhysicalKey::Code(KeyCode::KeyS) {
                    info!("camera move down");
                    self.camera.set_position(Vec2::new(cam_pos.x, cam_pos.y - move_speed));
                }
            }
            _ => {}
        }
    }

    fn on_close(&mut self) {
        info!("NewLayer close");
    }
}