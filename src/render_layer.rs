use azer::core::delta_time::DeltaTime;
use azer::core::event::Event;
use azer::core::input::InputState;
use azer::core::layer::Layer;
use azer::renderer::image_buffer_man::ImageBufferManager;
use azer::renderer::renderer::Renderer;
use azer::renderer::shapes::transform::Transform;
use glam::{Quat, Vec3};
use imgui::Ui;
use log::info;

pub struct RenderLayer {
    pub rotation: Quat,
    pub angle: f32,
}

impl RenderLayer {
    pub fn new() -> Self {
        Self {
            rotation: Quat::IDENTITY,
            angle: 0.0,
        }
    }
}

impl Layer for RenderLayer {
    fn on_ready(&mut self, _renderer: &mut Renderer) {
        info!("RenderLayer ready!")
    }

    fn on_update(&mut self, _delta: &DeltaTime, _input: &mut InputState) {
        self.angle = (self.angle + 0.01) % (2.0 * std::f32::consts::PI);
        self.rotation = Quat::from_rotation_z(self.angle);
    }

    fn on_render(&mut self, renderer: &mut Renderer, map: &mut ImageBufferManager) {
        for i in 0..5 {
            for j in 0..5 {
                let mut transform = Transform::default();
                transform.scale = Vec3::new(1.5,1.5,1.5);
                transform.position = Vec3::new(i as f32, j as f32, 0.0);
                if (i + j) % 2 == 0 {
                    renderer.draw_rectangle(transform, [0.2,0.3,0.4,1.0]);
                } else {
                    renderer.draw_rectangle(transform, [1.0,1.0,1.0,1.0]);
                }
            }
        }

        let mut transform = Transform::default();
        transform.scale = Vec3::new(0.1,0.1, 1.0);
        renderer.draw_image(transform, "E:\\360MoveData\\Users\\w1926\\OneDrive\\图片\\Camera Roll\\Snipaste_2025-08-19_01-57-10.png", map);
    }

    fn on_imgui_render(&mut self, _ui: &mut Ui) {
        
    }

    fn on_physics_update(&mut self, _delta: &DeltaTime) {

    }

    fn on_event(&mut self, _event: &Event) {

    }

    fn on_close(&mut self) {
        info!("render_layer closed")
    }
}