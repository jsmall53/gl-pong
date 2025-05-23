use nalgebra_glm as glm;


pub struct OrthographicCamera {
    position: glm::Vec3,
    rotation: f32,
    projection: glm::Mat4,
    view: glm::Mat4,
    view_projection: glm::Mat4,
}



pub struct OrthographicCameraController {
    aspect_ratio: f32,
    zoom_level: f32,
    camera: OrthographicCamera,
    rotation: bool,
    camera_rotation_speed: f32,
    camera_translation_speed: f32,
}



impl OrthographicCamera {
    pub fn new(left: f32, right: f32, top: f32, bottom: f32) -> Self {
        let projection = glm::ortho(left, right, top, bottom, -1.0f32, 1.0f32);
        let mut view = glm::Mat4::identity();
        let view_projection = glm::Mat4::identity();
        Self {
            position: glm::Vec3::zeros(),
            rotation: 0.0f32,
            projection,
            view,
            view_projection,
        }
    }

    pub fn set_projection(&mut self, left: f32, right: f32, top: f32, bottom: f32) {
        self.projection = glm::ortho(left, right, top, bottom, -1.0f32, 1.0f32);
        self.view_projection = self.projection * self.view;
    }

    pub fn recalculate_view(&mut self) {
        let identity = glm::Mat4::identity();

        // translation * rotation
        let transform: glm::Mat4 = glm::translate(&identity, &self.position) *
            glm::rotate(
                &identity, 
                self.rotation.to_radians(),
                &glm::Vec3::new(0.0f32, 0.0f32, 1.0f32)
            );
        self.view = glm::inverse(&transform);
        self.view_projection = self.projection * self.view;
    }

    pub fn get_view_projection(&self) -> &glm::Mat4 {
        &self.view_projection
    }
}



impl OrthographicCameraController {
    pub fn new(aspect_ratio: f32, rotation: bool) -> Self {
        let zoom_level = 1.0f32;
        let camera = OrthographicCamera::new(
            -aspect_ratio * zoom_level,
            aspect_ratio * zoom_level,
            -zoom_level,
            zoom_level,
        );

        Self {
            aspect_ratio,
            zoom_level,
            camera,
            rotation,
            camera_rotation_speed: 180.0f32,
            camera_translation_speed: 5.0f32,
        }
    }

    pub fn get_camera(&self) -> &OrthographicCamera {
        &self.camera
    }

    pub fn update(&mut self, delta: f32) {

    }

    pub fn resize(&mut self, width: f32, height: f32) {
        self.aspect_ratio = width / height;
        self.camera.set_projection(
            -self.aspect_ratio * self.zoom_level,
            self.aspect_ratio * self.zoom_level,
            -self.zoom_level,
            self.zoom_level);
    }
}


