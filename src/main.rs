mod camera_controller_layer;
mod render_layer;

use azer::core::{application::Application, logger};
use log::info;
use std::thread;
use std::time::Duration;
use winit::event_loop::{ControlFlow, EventLoop};

fn main() {
    // 日志模块
    logger::init_logger();
    info!("日志模块初始化成功！");

    // 窗口模块
    let event_loop = EventLoop::new().unwrap();
    event_loop.set_control_flow(ControlFlow::Poll);
    info!("窗口模块初始化成功！");

    let mut app: Application = Application::new();
    app.push_layer(Box::new(camera_controller_layer::NewLayer::new()));
    app.push_layer(Box::new(render_layer::RenderLayer::new()));

    event_loop.run_app(&mut app).unwrap();

    thread::sleep(Duration::from_millis(500));
}
