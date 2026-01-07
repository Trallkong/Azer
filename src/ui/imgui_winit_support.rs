use crate::core::event::Event;
use winit::event::ElementState::Pressed;
use winit::event::WindowEvent;

pub fn handle_event(imgui: &mut imgui::Context, event: &mut Event) {
    if event.handled {
        return;
    }

    let window_event = &event.event;
    let io = imgui.io_mut();

    match window_event {
        WindowEvent::CursorMoved {
            position, ..
        } => {
            io.mouse_pos = [position.x as f32, position.y as f32];
            if io.want_capture_mouse {
                event.handled = true;
            }
        },
        WindowEvent::MouseInput {
            button, state,  ..
        } => {
            let pressed = state.is_pressed();
            match button {
                winit::event::MouseButton::Left => io.mouse_down[0] = pressed,
                winit::event::MouseButton::Right => io.mouse_down[1] = pressed,
                winit::event::MouseButton::Middle => io.mouse_down[2] = pressed,
                _ => {}
            }
            // 如果 ImGui 想要捕获鼠标（点击了 UI），拦截该事件，不传给 Layer
            if io.want_capture_mouse {
                event.handled = true;
            }
        },
        WindowEvent::KeyboardInput {
            event: key_event, ..
        } => {
            if let winit::keyboard::PhysicalKey::Code(keycode) = key_event.physical_key {
                // 注意：ImGui 的 keys_down 通常需要映射到其特定的索引，
                // 确保映射逻辑与 imgui-rs 的枚举一致
                io.keys_down[keycode as usize] = key_event.state == Pressed;
            }
            // 如果 ImGui 正在输入文字（焦点在输入框），拦截键盘事件
            if io.want_capture_keyboard {
                event.handled = true;
            }
        },
        _ => {}
    }
}