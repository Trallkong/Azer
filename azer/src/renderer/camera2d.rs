pub struct Camera2D {
    pub position: [f32; 2],
    pub zoom: f32,

    aspect_ratio: f32,
    projection: glam::Mat4,
    view: glam::Mat4,
    view_projection: glam::Mat4,
}

impl Camera2D {
    pub fn new(aspect_ratio: f32) -> Self {
        let projection = glam::Mat4::orthographic_rh(-aspect_ratio * 1.0, aspect_ratio * 1.0, -1.0, 1.0, 0.0, 1.0);
        let view = glam::Mat4::IDENTITY;

        Self {
            position: [0.0, 0.0],
            aspect_ratio,
            zoom: 1.0,

            projection,
            view,
            view_projection: projection * view,
        }
    }

    fn update_projection(&mut self)  {
        self.projection = glam::Mat4::orthographic_rh(
            -self.aspect_ratio * self.zoom, 
            self.aspect_ratio * self. zoom, 
            -self.zoom, 
            self.zoom, 
            0.0, 
            1.0,
        );
    }

    fn update_view(&mut self) {
        let view = glam::Mat4::from_translation(glam::Vec3::new(self.position[0], self.position[1], 0.0));
        self.view = view.inverse();
    } 

    pub fn update(&mut self) {
        self.update_projection();
        self.update_view();
        self.view_projection = self.projection * self.view;
    }

    pub fn vp(&self) -> glam::Mat4 {
        self.view_projection
    }
}