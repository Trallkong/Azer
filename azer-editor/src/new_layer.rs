use azer::{core::{event::{EventType, MouseButton}, input, layer::Layer, logger::info}, renderer::camera2d::Camera2D};
use azer::core::event::KeyCode;

pub struct NewLayer {
    camera: Camera2D,
    camera_move_speed: f32,
    last_mouse_pos: (f64, f64),
}

impl NewLayer {
    pub fn new() -> Self {
        Self {
            camera: Camera2D::new(16.0/9.0),
            camera_move_speed: 1.0,
            last_mouse_pos: (0.0, 0.0),
        }
    }
}

impl Layer for NewLayer {
    fn on_attach(&mut self) {
        info!("NewLayer 已添加");
        self.camera.zoom = 1.0;
        self.camera_move_speed = 3.0;
    }

    fn on_detach(&mut self) {}
    
    fn event(&mut self, event: &azer::core::event::Event) {
        if event.kind.category() == azer::core::event::EventCategory::MOUSE {
            if let EventType::MouseScrolled { value } = event.kind {
                self.camera.zoom -= value * 0.1;
                self.camera.zoom = self.camera.zoom.max(0.1);
            }
        }
    }
    
    fn render(&self, renderer: &mut azer::renderer::renderer::Renderer, render_pass: &mut azer::core::application::RenderPass) {
        renderer.set_camera(self.camera.vp());
        renderer.renderer_2d.draw_quad(render_pass);
    }
    
    fn update(&mut self, delta_time: &azer::core::delta_time::DeltaTime, input: &input::InputState) {
        if input.is_key_pressed(KeyCode::KeyW) {
            self.camera.position[1] += self.camera_move_speed * delta_time.as_seconds();
        }
        if input.is_key_pressed(KeyCode::KeyS) {
            self.camera.position[1] -= self.camera_move_speed * delta_time.as_seconds();
        }
        if input.is_key_pressed(KeyCode::KeyA) {
            self.camera.position[0] -= self.camera_move_speed * delta_time.as_seconds();
        }
        if input.is_key_pressed(KeyCode::KeyD) {
            self.camera.position[0] += self.camera_move_speed * delta_time.as_seconds();
        }

        if input.is_mouse_pressed(MouseButton::Left) {
            let (x, y) = input.mouse_pos();
            self.camera.position[0] -= (x - self.last_mouse_pos.0) as f32 * self.camera.zoom * 0.001;
            self.camera.position[1] += (y - self.last_mouse_pos.1) as f32 * self.camera.zoom * 0.001;
            self.last_mouse_pos = (x, y);
        } else {
            self.last_mouse_pos = input.mouse_pos();
        }

        self.camera.update();
    }

}