/// 顶点缓冲区元素类型
#[derive(Debug, Clone, Copy)]
pub enum VertexBufferElementType {
    Float,
    Float2,
    Float3,
    Float4,
    Int,
    Int2,
    Int3,
    Int4,
    Bool
}

/// 顶点缓冲区元素类型转换
impl VertexBufferElementType {

    /// 获取元素大小
    pub fn size(&self) -> u32 {
        match self {
            VertexBufferElementType::Float  => 4,
            VertexBufferElementType::Float2 => 4 * 2,
            VertexBufferElementType::Float3 => 4 * 3,
            VertexBufferElementType::Float4 => 4 * 4,
            VertexBufferElementType::Int    => 4,
            VertexBufferElementType::Int2   => 4 * 2,
            VertexBufferElementType::Int3   => 4 * 3,
            VertexBufferElementType::Int4   => 4 * 4,
            VertexBufferElementType::Bool   => 1,
        }
    }

    /// 转换为wgpu::VertexFormat
    fn to_wgpu_format(&self) -> wgpu::VertexFormat {
        match self {
            VertexBufferElementType::Bool   => wgpu::VertexFormat::Uint8,
            VertexBufferElementType::Float  => wgpu::VertexFormat::Float32,
            VertexBufferElementType::Float2 => wgpu::VertexFormat::Float32x2,
            VertexBufferElementType::Float3 => wgpu::VertexFormat::Float32x3,
            VertexBufferElementType::Float4 => wgpu::VertexFormat::Float32x4,
            VertexBufferElementType::Int    => wgpu::VertexFormat::Sint32,
            VertexBufferElementType::Int2   => wgpu::VertexFormat::Sint32x2,
            VertexBufferElementType::Int3   => wgpu::VertexFormat::Sint32x3,
            VertexBufferElementType::Int4   => wgpu::VertexFormat::Sint32x4,
        }
    }
}

/// 顶点缓冲区布局
#[derive(Debug, Clone)]
pub struct VertexBufferLayout {
    pub stride: u32,
    pub layout: Vec<(u32, VertexBufferElementType)>, // (offset, element_type)
    attributes: Vec<wgpu::VertexAttribute>,
}

impl VertexBufferLayout {
    pub fn new() -> Self {
        Self {
            stride: 0,
            layout: vec![],
            attributes: vec![],
        }
    }

    /// 添加元素
    pub fn push(&mut self, element_type: VertexBufferElementType) {
        let offset = self.stride;
        self.layout.push((self.stride, element_type));
        self.attributes.push(wgpu::VertexAttribute { 
            format: element_type.to_wgpu_format(), 
            offset: offset as wgpu::BufferAddress, 
            shader_location: self.attributes.len() as u32
        });
        self.stride += element_type.size();
    }

    /// 转换为wgpu::VertexBufferLayout
    pub fn desc(&self) -> wgpu::VertexBufferLayout<'_> {
        wgpu::VertexBufferLayout {
            array_stride: self.stride as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &self.attributes,
        }
    }
}