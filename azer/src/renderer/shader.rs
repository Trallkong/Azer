use wgpu::SurfaceConfiguration;

pub struct Shader {
    render_pipeline: wgpu::RenderPipeline
}

impl Shader {

    pub fn new(
        device: &wgpu::Device , 
        label: Option<String>, 
        path: &str, 
        config: &SurfaceConfiguration,
        vbo_layouts: &[wgpu::VertexBufferLayout],
        bind_group_layouts: Option<&[&wgpu::BindGroupLayout]>,
    ) -> Self {
        let source_code = std::fs::read_to_string(path)
            .expect("Failed to read shader file");

        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor { 
            label: label.as_deref(), 
            source: wgpu::ShaderSource::Wgsl(source_code.into()) , 
        });

        let render_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Render Pipeline Layout"),
                bind_group_layouts: bind_group_layouts.unwrap_or(&[]),
                immediate_size: 0,
            });

        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Render Pipeline"),
            layout: Some(&render_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: Some("vs_main"), // 1.
                buffers: vbo_layouts, // 2.
                compilation_options: wgpu::PipelineCompilationOptions::default(),
            },
            fragment: Some(wgpu::FragmentState { // 3.
                module: &shader,
                entry_point: Some("fs_main"),
                targets: &[Some(wgpu::ColorTargetState { // 4.
                    format: config.format,
                    blend: Some(wgpu::BlendState::REPLACE),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
                compilation_options: wgpu::PipelineCompilationOptions::default(),
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList, // 1.
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw, // 2.
                cull_mode: Some(wgpu::Face::Back),
                // Setting this to anything other than Fill requires Features::NON_FILL_POLYGON_MODE
                polygon_mode: wgpu::PolygonMode::Fill,
                // Requires Features::DEPTH_CLIP_CONTROL
                unclipped_depth: false,
                // Requires Features::CONSERVATIVE_RASTERIZATION
                conservative: false,
            },
            depth_stencil: None, // 1.
            multisample: wgpu::MultisampleState {
                count: 1, // 2.
                mask: !0, // 3.
                alpha_to_coverage_enabled: false, // 4.
            },
            multiview_mask: None, // 5.
            cache: None, // 6.
        });

        Self { render_pipeline }
    }

    pub fn get(&self) -> &wgpu::RenderPipeline { &self.render_pipeline }

    pub fn bind(&self, render_pass: &mut wgpu::RenderPass<'_>) {
        render_pass.set_pipeline(&self.render_pipeline);
    }
}