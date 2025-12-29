use std::collections::HashSet;
use winit::event::MouseButton;
use winit::keyboard::KeyCode;

#[derive(Default)]
pub struct InputState {
    pressed_keys: HashSet<KeyCode>,
    mouse_buttons: HashSet<MouseButton>,
    mouse_position: (f64, f64),
}

impl InputState {
    pub fn is_key_pressed(&self, key: KeyCode) -> bool {
        self.pressed_keys.contains(&key)
    }

    pub fn is_mouse_pressed(&self, button: MouseButton) -> bool {
        self.mouse_buttons.contains(&button)
    }

    pub fn mouse_pos(&self) -> (f64, f64) {
        self.mouse_position
    }

    // 更新按键状态
    pub fn update_key(&mut self, key: winit::keyboard::PhysicalKey, pressed: bool) {
        if let winit::keyboard::PhysicalKey::Code(key_code) = key {
            if pressed {
                self.pressed_keys.insert(key_code);
            } else {
                self.pressed_keys.remove(&key_code);
            }
        }
    }

    // 更新鼠标状态
    pub fn update_mouse(&mut self, button: MouseButton, pressed: bool) {
        if pressed {
            self.mouse_buttons.insert(button);
        } else {
            self.mouse_buttons.remove(&button);
        }
    }

    pub fn update_mouse_pos(&mut self, x: f64, y: f64) {
        self.mouse_position = (x, y);
    }
}