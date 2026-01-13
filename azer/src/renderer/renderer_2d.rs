use log::info;
use wgpu::util::DeviceExt;

use crate::renderer::{index_buffer::IndexBuffer, mesh::{CameraUniform, INDICES, VERTICES}, render_command, render_context::RenderContext, shader::Shader, vertex_buffer::VertexBuffer, vertex_buffer_layout};

pub struct Renderer2D {
    pub shader: Shader,
    pub texture: crate::renderer::texture::Texture,
    pub vbo: VertexBuffer,
    pub ibo: IndexBuffer,

    pub camera_bind_group: wgpu::BindGroup,
    pub camera_uniform: CameraUniform,
    pub camera_buffer: wgpu::Buffer,

    pub queue: wgpu::Queue,
}

impl Renderer2D {
    pub(crate) fn new(context: &RenderContext) -> Self {

        let camera_uniform = CameraUniform::new();

        let camera_buffer = context.device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some("Camera Buffer"),
                contents: bytemuck::cast_slice(&[camera_uniform]),
                usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            }
        );

        let camera_bind_group_layout = context.device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::VERTEX,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                }
            ],
            label: Some("camera_bind_group_layout"),
        });

        let camera_bind_group = context.device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &camera_bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: camera_buffer.as_entire_binding(),
                }
            ],
            label: Some("camera_bind_group"),
        });

        let mut vbo_layout = vertex_buffer_layout::VertexBufferLayout::new();
        vbo_layout.push(vertex_buffer_layout::VertexBufferElementType::Float3);
        vbo_layout.push(vertex_buffer_layout::VertexBufferElementType::Float2);

        let texture = crate::renderer::texture::Texture::new(
            "C:\\Users\\Trallkong\\OneDrive\\图片\\本机照片\\pic.png", &context.device, &context.queue);

        let path = "E:\\Projects\\new\\azer\\src\\shaders\\shader.wgsl";
        let shader = Shader::new(
            &context.device, 
            Some(String::from("Shader")), 
            path, 
            &context.config, 
            &[vbo_layout.desc()],
            Some(&[texture.layout(), &camera_bind_group_layout])
        );

        let vertex_buffer = VertexBuffer::new(&context.device, Some(String::from("VertexBuffer")), VERTICES);
        let index_buffer = IndexBuffer::new(&context.device, Some(String::from("IndexBuffer")), INDICES);

        Self {
            shader,
            texture,
            vbo: vertex_buffer,
            ibo: index_buffer,
            camera_bind_group,
            camera_uniform,
            camera_buffer,
            queue: context.queue.clone(),
        }
    }

    pub(crate) fn begin_render(&self) {
        
    }

    pub(crate) fn end_render(&self) {

    }

    pub fn draw_quad(&self, render_pass: &mut wgpu::RenderPass) {
        self.shader.bind(render_pass);
        self.texture.bind(0, render_pass);
        render_pass.set_bind_group(1, &self.camera_bind_group, &[]);
        render_command::draw_indexed(render_pass, &self.vbo, &self.ibo);
    }

    pub(crate) fn set_camera(&mut self, vp: &glam::Mat4) {
        self.camera_uniform.update_view_proj(vp);
        self.queue.write_buffer(&self.camera_buffer, 0, bytemuck::cast_slice(&[self.camera_uniform]));
    }    
}