use azer::core::{entry_point::Azer, logger::{info}};

pub mod new_layer;

fn main() {
    let mut app = Azer::new();
    info!("start app");
    app.application().push_layer(Box::new(new_layer::NewLayer::new()));
    app.run();
}
