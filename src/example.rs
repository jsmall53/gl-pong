use crate::renderer::{camera::OrthographicCameraController, *};
use crate::core::input::{InputController,InputState};
use crate::core::FrameCounter;

use winit::event::KeyEvent;
use glow::*;
use glutin::prelude::GlDisplay;
use nalgebra_glm as glm;



pub struct Example2D {
    renderer: Renderer2D,
    camera: camera::OrthographicCameraController,
    input: InputController,
    frame_counter: FrameCounter,
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
                input: InputController::new(),
                frame_counter: FrameCounter::new(),
            }
        }
    }

    pub fn resize(&mut self, width: i32, height: i32) {
        self.renderer.resize(width, height);
    }

    pub fn update(&mut self) {
        // Always handle frame counter and input at the beginning of update.
        let delta = self.update_frames();
        let input_state = self.input.state();
        //****************************************

        self.camera.update(delta, &input_state);
        self.renderer.begin_scene(self.camera.get_camera());

        let mut pos = glm::Vec3::new(-0.5, -0.5, 0.5);
        let size = glm::Vec2::new(0.8, 0.4);
        let color = glm::Vec4::new(0.8, 0.2, 0.2, 1.0);
        self.renderer.draw_quad_ez(&pos, &size, color);

        pos.x = 0.5;
        pos.y = 0.5;
        self.renderer.draw_quad_ez(&pos, &size, color);

        pos.x = 0.0;
        pos.y = 0.0;
        self.renderer.draw_quad_ez(&pos, &size, color);
 
        self.renderer.end_scene();
    }

    pub fn handle_keyboard(&mut self, event: KeyEvent) {
        self.input.handle_keyboard(event)
    }

    fn update_frames(&mut self) -> f32 {
        let delta = self.frame_counter.increment();
        match self.frame_counter.fps() {
            Some(fps) => println!("{:.2} fps", fps),
            None => { }
        }
        delta
    }
}



