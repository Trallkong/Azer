use crate::renderer::{index_buffer::IndexBuffer, vertex_buffer::VertexBuffer};

pub fn clear<'a>(encoder: &'a mut wgpu::CommandEncoder, view: &'a wgpu::TextureView, clear_color: &[f64;4]) -> wgpu::RenderPass<'a> {
    encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
        label: Some("Render Pass"),
        color_attachments: &[Some(wgpu::RenderPassColorAttachment {
            view: &view,
            resolve_target: None,
            ops: wgpu::Operations {
                load: wgpu::LoadOp::Clear(wgpu::Color {
                    r: clear_color[0],
                    g: clear_color[1],
                    b: clear_color[2],
                    a: clear_color[3],
                }),
                store: wgpu::StoreOp::Store,
            },
            depth_slice: None,
        })],
        depth_stencil_attachment: None,
        occlusion_query_set: None,
        timestamp_writes: None,
        multiview_mask: None,
    })
}

pub fn draw_indexed(render_pass: &mut wgpu::RenderPass<'_>, vbo: &VertexBuffer, ibo: &IndexBuffer) {
    render_pass.set_vertex_buffer(0, vbo.buffer().slice(..));
    render_pass.set_index_buffer(ibo.buffer().slice(..), wgpu::IndexFormat::Uint16);
    render_pass.draw_indexed(0..ibo.num_indices(), 0, 0..1);
}