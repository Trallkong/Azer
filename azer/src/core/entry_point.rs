/// 这是 Azer 的入口点
/// 如果你要创建一个 Azer 应用程序，使用 Azer::new() 创建一个实例，并调用 run() 方法启动应用程序。

use winit::event_loop;
use crate::core::{application::Application, logger};

/// Azer 运行时
pub struct Azer {
    event_loop: event_loop::EventLoop<()>,
    application: Application,
}

impl Azer {
    /// 创建一个 Azer 实例
    pub fn new() -> Self {
        // 初始化日志
        logger::init_logger();

        // 创建事件循环
        let event_loop = event_loop::EventLoop::new().unwrap();
        event_loop.set_control_flow(event_loop::ControlFlow::Poll);

        // 创建应用实例
        let application = Application::default();

        Self {
            event_loop,
            application,
        }
    }

    /// 获取应用实例
    pub fn application(&mut self) -> &mut Application {
        &mut self.application
    }

    /// 运行应用
    pub fn run(mut self) {
        let _ = self.event_loop.run_app(&mut self.application);
    }
}