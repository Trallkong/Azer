/// 事件包装器
/// 用于包装 winit::event::WindowEvent
/// 筛选并简化鼠标、键盘、窗口等事件
/// 不允许在事件处理过程中修改事件状态
/// 使用 handled 字段来指示事件是否已被处理

pub use winit::keyboard::KeyCode;   // 导出键盘按键枚举
pub use winit::event::MouseButton;  // 导出鼠标按键枚举

use std::{cell::Cell};

// 事件类型
#[derive(Debug, Clone)]
pub struct Event {
    pub kind: EventType,
    pub handled: Cell<bool>
}

impl Event {

    // 默认的创建事件方法，如果你不是从winit拿到的事件而是想自定义一个事件的话，请使用此方法
    pub fn new(kind: EventType) -> Self {
        Event { kind, handled: Cell::new(false) }
    }

    // 从winit::event::WindowEvent创建事件
    pub fn from_winit_event(event: &winit::event::WindowEvent) -> Option<Self> {
        match event {
            winit::event::WindowEvent::CloseRequested => Some(Self::new(EventType::WindowClose)),
            winit::event::WindowEvent::Resized(size) => Some(Self::new(EventType::WindowResize { width: size.width, height: size.height })),
            winit::event::WindowEvent::Focused(focused) => Some(Self::new(EventType::WindowFocus(*focused))),
            winit::event::WindowEvent::KeyboardInput { event: key_event, ..} => {
                let key_code = match key_event.physical_key {
                    winit::keyboard::PhysicalKey::Code(code) => code,
                    winit::keyboard::PhysicalKey::Unidentified(_) => return None,
                };
                match key_event.state {
                    winit::event::ElementState::Pressed => Some(Self::new(EventType::KeyPressed { key_code })),
                    winit::event::ElementState::Released => Some(Self::new(EventType::KeyReleased { key_code })),
                }
            },
            winit::event::WindowEvent::MouseInput { state, button, ..} => {
                match state {
                    winit::event::ElementState::Pressed => Some(Self::new(EventType::MouseButtonPressed { button: *button })),
                    winit::event::ElementState::Released => Some(Self::new(EventType::MouseButtonReleased { button: *button })),
                }
            },
            winit::event::WindowEvent::CursorMoved { position, ..} => {
                let (x, y) = (position.x as f32, position.y as f32);
                Some(Self::new(EventType::MouseMoved { x, y }))
            },
            winit::event::WindowEvent::MouseWheel { delta, .. } => {
                match delta {
                    winit::event::MouseScrollDelta::LineDelta(_x, y) => Some(Self::new(EventType::MouseScrolled { value: *y })),
                    winit::event::MouseScrollDelta::PixelDelta(position) => Some(Self::new(EventType::MouseScrolled { value: position.y as f32 })),
                }
            },
            _ => None,
        }
    }
}

// 事件类别
use bitflags::bitflags;

bitflags! {

    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
    pub struct EventCategory: u8 {
        const KEYBOARD  = 1 << 0;
        const MOUSE     = 1 << 1;
        const WINDOW    = 1 << 2;
        const APP       = 1 << 3;
        const INPUT    = Self::KEYBOARD.bits() | Self::MOUSE.bits();
    }
}

// 事件类型
#[derive(Debug, Clone, Copy)]
pub enum EventType {
    WindowClose,
    WindowResize { width: u32, height: u32 },
    WindowFocus(bool),
    WindowLostFocus,
    WindowMoved,
    AppTick,
    AppUpdate,
    AppRender,
    KeyPressed { key_code: KeyCode },
    KeyReleased { key_code: KeyCode },
    MouseButtonPressed { button: MouseButton },
    MouseButtonReleased { button: MouseButton },
    MouseMoved { x: f32, y: f32 },
    MouseScrolled { value: f32 },
}

impl EventType {
    pub fn category(&self) -> EventCategory {
        match self {
            EventType::KeyPressed { .. } | EventType::KeyReleased { .. } => EventCategory::KEYBOARD,
            EventType::MouseButtonPressed { .. } | EventType::MouseButtonReleased { .. } | EventType::MouseScrolled { .. } => EventCategory::MOUSE,
            EventType::WindowClose | EventType::WindowResize { .. } => EventCategory::WINDOW,
            _ => EventCategory::APP,
        }
    }
}