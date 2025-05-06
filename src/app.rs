use std::{error::Error, num::NonZeroU32};
use glutin_winit::{DisplayBuilder, GlWindow};
use winit::{application::ApplicationHandler, event_loop::EventLoop, raw_window_handle::HasWindowHandle, window::{Window, WindowAttributes}};
use glutin::{config::GetGlConfig, context::ContextAttributesBuilder, display::GetGlDisplay, prelude::*};
use glutin::context::{PossiblyCurrentContext, NotCurrentContext, ContextApi};
use glutin::surface::{WindowSurface, SwapInterval, Surface};
use glutin::config::{Config, ConfigTemplateBuilder};

const WINDOW_TITLE: &str = "gl-pong";

pub struct App {
    template: ConfigTemplateBuilder,
    renderer: Option<i32>, // TODO; implement renderer as type Renderer...
    app_state: Option<AppState>, //TODO: implement AppState type
    gl_context: Option<PossiblyCurrentContext>,
    gl_display: GlDisplayCreationState,
    exit_state: Result<(), Box<dyn Error>>,
}

impl App {
    pub fn new(
        template: ConfigTemplateBuilder, 
    ) -> Self {
        let display_builder = DisplayBuilder::new().with_window_attributes(Some(window_attributes()));

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

struct AppState {
    gl_surface: Surface<WindowSurface>,
    window: Window
}

enum GlDisplayCreationState {
    Builder(DisplayBuilder), // display was not built yet
    Init, // display has been created and initialized for the application.
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
       let (window, gl_config) = match &self.gl_display {
           GlDisplayCreationState::Builder(display_builder) => {
               let (window, gl_config) = match display_builder.clone().build(
                       event_loop,
                       self.template.clone(),
                       gl_config_picker) {
                   Ok((window, gl_config)) => (window.unwrap(), gl_config),
                   Err(e) => {
                       self.exit_state = Err(e);
                       event_loop.exit();
                       return;
                   }
               };

               println!("Picked a config with {} samples", gl_config.num_samples());
               self.gl_display = GlDisplayCreationState::Init;
               
               self.gl_context = Some(create_gl_context(&window, &gl_config).treat_as_possibly_current());

               (window, gl_config)
           },
           GlDisplayCreationState::Init => {
               // not sure what this does right now?
               println!("Need to recreate window in `resumed`");
               let gl_config = self.gl_context.as_ref().as_ref().unwrap().config();
               match glutin_winit::finalize_window(event_loop, window_attributes(), &gl_config) {
                   Ok(window) => (window, gl_config),
                   Err(e) => {
                       self.exit_state = Err(e.into());
                       event_loop.exit();
                       return;
                   }
               }
           }
       };


       let attrs = window
           .build_surface_attributes(Default::default())
           .expect("Failed to build surface attributes");

        let gl_surface = unsafe {
            gl_config.display().create_window_surface(&gl_config, &attrs).unwrap()
        };

        let gl_context = self.gl_context.as_ref().unwrap();
        gl_context.make_current(&gl_surface);

        // TODO: can initialize renderer here...

        if let Err(res) = gl_surface.set_swap_interval(gl_context, SwapInterval::Wait(NonZeroU32::new(1).unwrap())) {
            eprintln!("Error setting vsync: {res:?}");
        }

        assert!(self.app_state.replace(AppState {gl_surface, window}).is_none());
    }


    fn window_event(
            &mut self,
            event_loop: &winit::event_loop::ActiveEventLoop,
            window_id: winit::window::WindowId,
            event: winit::event::WindowEvent,
        ) {
        
    }
    
    fn about_to_wait(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        
    }
}

fn gl_config_picker(configs: Box<dyn Iterator<Item = Config> + '_>) -> Config {
    configs.reduce(|accum, config| {
        if config.num_samples() > accum.num_samples() {
            config
        } else {
            accum
        }
    }).unwrap()
}

fn create_gl_context(window: &Window, gl_config: &Config) -> NotCurrentContext {
    let raw_window_handle = window.window_handle().ok().map(|h| h.as_raw());

    let context_attributes = ContextAttributesBuilder::new().build(raw_window_handle);

    // Since glutin by default tries to create OpenGL core context, which may not be
    // present we should try gles.
    let fallback_context_attributes = ContextAttributesBuilder::new()
        .with_context_api(ContextApi::Gles(None))
        .build(raw_window_handle);

    let gl_display = gl_config.display();

    unsafe {
        gl_display.create_context(gl_config, &context_attributes). unwrap_or_else(|_| {
            gl_display.create_context(gl_config, &fallback_context_attributes)
                .expect("Failed to create context")
        })
    }
}

fn window_attributes() -> WindowAttributes {
    WindowAttributes::default()
            .with_transparent(false)
            .with_title(WINDOW_TITLE)
}
