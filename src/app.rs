use std::error::Error;
use glutin_winit::DisplayBuilder;
use winit::{application::ApplicationHandler, event_loop::EventLoop, window::WindowAttributes};
use glutin::{config::ConfigTemplateBuilder, context::PossiblyCurrentContext, prelude::*};

pub struct App {
    template: ConfigTemplateBuilder,
    renderer: Option<i32>, // TODO; implement renderer as type Renderer...
    app_state: Option<i32>, //TODO: implement AppState type
    gl_context: Option<PossiblyCurrentContext>,
    gl_display: GlDisplayCreationState,
    exit_state: Result<(), Box<dyn Error>>,
}

impl App {
    pub fn new(
        template: ConfigTemplateBuilder, 
        display_builder: DisplayBuilder,
    ) -> Self {
        Self {
            template, 
            gl_display: GlDisplayCreationState::Builder(display_builder),
            app_state: None,
            renderer: None,
            gl_context: None,
            exit_state: Ok(())
        }
    }
}


enum GlDisplayCreationState {
    Builder(DisplayBuilder), // display was not built yet
    Init, // display has been created and initialized for the application.
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        
    }


    fn window_event(
            &mut self,
            event_loop: &winit::event_loop::ActiveEventLoop,
            window_id: winit::window::WindowId,
            event: winit::event::WindowEvent,
        ) {
        
    }
}
