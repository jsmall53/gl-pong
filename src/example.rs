use crate::renderer::{camera::OrthographicCameraController, *};
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
    camera: camera::OrthographicCameraController,
}



impl Example2D {
    pub fn new<D: GlDisplay>(gl_display: &D, width: i32, height: i32) -> Self {
        unsafe {
            let gl = Context::from_loader_function_cstr(
                |s| gl_display.get_proc_address(s)
            );
            gl.enable(PROGRAM_POINT_SIZE);
            gl.viewport(0, 0, width, height);

            let renderer = Renderer2D::new(gl, width, height);
            let camera = OrthographicCameraController::new(width as f32 / height as f32, false);
            Self {
                renderer,
                camera, 
            }
        }
    }

    pub fn resize(&mut self, width: i32, height: i32) {
        self.renderer.resize(width, height);
    }

    pub fn update(&mut self) {
        println!("begin_scene");
        self.renderer.begin_scene(self.camera.get_camera());

        println!("done.");
        println!("init quad data");
        let pos = glm::Vec3::new(0.5, 0.5, 1.0);
        let size = glm::Vec2::new(0.5, 0.5);
        let color = glm::Vec4::new(0.8, 0.2, 0.2, 1.0);
        println!("done.");
        println!("draw_quad_ez");
        self.renderer.draw_quad_ez(&pos, &size, color);
        println!("done.");
        println!("end_scene");
        self.renderer.end_scene();
        println!("done.");
    }
}



