use crate::renderer::*;
// use crate::physics::*;
// use crate::input::{InputController, InputState, PongKey, KeyMap};

// use std::collections::HashMap;
// use std::time::{Instant, Duration};
// use std::ops::Deref;
// use std::cell::RefCell;

use winit::event::KeyEvent;
use glow::*;
use glutin::prelude::GlDisplay;
use nalgebra_glm as glm;



pub struct Example2D {
    renderer: Renderer2D,
}



impl Example2D {
    pub fn new<D: GlDisplay>(gl_display: &D, width: i32, height: i32) -> Self {
        unsafe {
            let gl = Context::from_loader_function_cstr(
                |s| gl_display.get_proc_address(s)
            );
            gl.enable(PROGRAM_POINT_SIZE);
            gl.viewport(0, 0, width, height);

            let renderer = Renderer2D::new(gl);

            Self {
                renderer,
            }
        }
    }

    pub fn resize(&mut self, width: i32, height: i32) {
        // no op
    }

    pub fn update(&mut self) {
        self.renderer.begin_scene();
        self.renderer.draw_quad();
        self.renderer.end_scene();
    }
}



