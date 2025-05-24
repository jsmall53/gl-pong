pub mod core;
pub mod app;
pub mod renderer;
pub mod game;
pub mod physics;

use crate::app::App;



use std::error::Error;
use winit::{application::ApplicationHandler, event_loop::EventLoop, window::WindowAttributes};

use glutin::{config::ConfigTemplateBuilder, context::PossiblyCurrentContext, prelude::*};


fn main() -> Result<(), Box<dyn Error>> {
    let config_template = ConfigTemplateBuilder::new().with_alpha_size(8).with_transparency(false);
    let mut app = App::new(config_template);

    let event_loop = EventLoop::new().unwrap();
    Ok(event_loop.run_app(&mut app)?)
}

