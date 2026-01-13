use crate::renderer::render_context::RenderContext;

pub struct Renderer {
    pub renderer_3d: super::renderer_3d::Renderer3D,
    pub renderer_2d: super::renderer_2d::Renderer2D,
}

impl Renderer {
    pub fn new(context: &RenderContext) -> Self {
        Self {
            renderer_3d: super::renderer_3d::Renderer3D::new(),
            renderer_2d: super::renderer_2d::Renderer2D::new(context),
        }
    }
}

impl Renderer {

    pub(crate) fn begin_render(&self) {
        self.renderer_3d.begin_render();
        self.renderer_2d.begin_render();
    }

    pub(crate) fn end_render(&self) {
        self.renderer_3d.end_render();
        self.renderer_2d.end_render();
    }

    pub fn set_camera(&mut self, vp: glam::Mat4) {
        self.renderer_2d.set_camera(&vp);
    }
}