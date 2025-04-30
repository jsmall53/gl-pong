pub mod app;
use crate::app::App;
use std::error::Error;
use glutin_winit::DisplayBuilder;
use winit::{application::ApplicationHandler, event_loop::EventLoop, window::WindowAttributes};

use glutin::{config::ConfigTemplateBuilder, context::PossiblyCurrentContext, prelude::*};

const WINDOW_TITLE: &str = "gl-pong";

fn main() -> Result<(), Box<dyn Error>> {
    let config_template = ConfigTemplateBuilder::new().with_alpha_size(8).with_transparency(false);
    let window_attributes = WindowAttributes::default()
        .with_transparent(false)
        .with_title(WINDOW_TITLE);
    let display_builder = DisplayBuilder::new().with_window_attributes(Some(window_attributes));
    let mut app = App::new(config_template, display_builder);

    let event_loop = EventLoop::new().unwrap();
    Ok(event_loop.run_app(&mut app)?)
}

