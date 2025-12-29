use azer::core::delta_time::DeltaTime;
use azer::core::layer::Layer;
use azer::renderer::camera::camera2d::Camera2D;
use azer::renderer::camera::Camera;
use azer::renderer::renderer::Renderer;
use azer::renderer::shapes::transform::Transform2D;
use glam::{Quat, Vec2};
use log::info;
use winit::event::WindowEvent;
use winit::keyboard::{KeyCode, PhysicalKey};

pub struct NewLayer {
    pub camera: Camera2D,
    pub rotation: Quat,
    pub angle: f32,
}

impl NewLayer {
    pub fn new() -> Self {
        let aspect_ratio = 16.0 / 9.0;
        let zoom = 1.0;
        let pos = Vec2::new(0.0, 0.0);
        Self {
            camera: Camera2D::new(aspect_ratio, zoom, -1.0, 1.0, pos),
            rotation: Quat::IDENTITY,
            angle: 0.0,
        }
    }
}

impl Layer for NewLayer {
    fn on_ready(&mut self, _renderer: &mut Renderer) {
        info!("NewLayer ready");
    }

    fn on_update(&mut self, _delta: &DeltaTime, renderer: &mut Renderer) {
        self.camera.update();
        renderer.update_camera(*self.camera.get_view_projection_matrix());

        self.angle = (self.angle + 0.01) % (2.0 * std::f32::consts::PI);
        self.rotation = Quat::from_rotation_z(self.angle);
    }

    fn on_render(&mut self, renderer: &mut Renderer) {
        // let transform = Transform2D {
        //     position: Vec2::new(0.0, 0.0),
        //     scale: Vec2::new(0.5, 0.5),
        //     rotation: self.rotation,
        // };
        // renderer.draw_triangle(transform, [1.0,0.4,0.6,0.8]);
        //
        // let transform2 = Transform2D {
        //     position: Vec2::new(0.0, 0.5),
        //     scale: Vec2::new(0.5, 0.5),
        //     rotation: self.rotation,
        // };
        //
        // renderer.draw_rectangle(transform2, [0.2,0.3,0.4,1.0]);

        for i in 0..5 {
            for j in 0..5 {
                let transform = Transform2D {
                    position: Vec2::new(i as f32, j as f32),
                    scale: Vec2::new(1.0, 1.0),
                    rotation: Quat::IDENTITY,
                };
                if (i + j) % 2 == 0 {
                    renderer.draw_rectangle(transform, [0.2,0.3,0.4,1.0]);
                } else {
                    renderer.draw_rectangle(transform, [1.0,1.0,1.0,1.0]);
                }
            }
        }
    }

    fn on_physics_update(&mut self, _delta: &DeltaTime) {
        // info!("NewLayer physics update");
    }

    fn on_event(&mut self, event: &WindowEvent) {
        match event {
            WindowEvent::KeyboardInput {
                device_id: _device_id,
                event,
                is_synthetic: _is_synthetic,
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
            },
            WindowEvent::MouseWheel {delta,phase, ..} => {
                if *phase == winit::event::TouchPhase::Started || *phase == winit::event::TouchPhase::Moved {
                    match delta {
                        winit::event::MouseScrollDelta::LineDelta(_, y) => {
                            self.camera.set_zoom(self.camera.zoom() - y * 0.1);
                        },
                        winit::event::MouseScrollDelta::PixelDelta(pos) => {
                            self.camera.set_zoom(self.camera.zoom() - pos.y as f32 * 0.1);
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