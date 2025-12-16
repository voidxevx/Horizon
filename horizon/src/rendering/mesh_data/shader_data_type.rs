#[allow(unused)]
pub enum ShaderDataType {
    Float,
    Float2,
    Float3,
    Int,
    Int2,
    Int3,
    Float2x2,
    Float3x3,
    Float4x4,
}

#[allow(unused)]
impl ShaderDataType {
    pub fn get_size(&self) -> (u32, u32, u32) {
        match self {
            ShaderDataType::Float => (size_of::<f32>() as u32, 1, gl::FLOAT),
            ShaderDataType::Float2 => ((size_of::<f32>() * 2.0 as usize) as u32 , 2, gl::FLOAT),
            ShaderDataType::Float3 => ((size_of::<f32>() * 3.0 as usize) as u32, 3, gl::FLOAT),
            ShaderDataType::Int => (size_of::<i32>() as u32, 1, gl::INT),
            ShaderDataType::Int2 => ((size_of::<i32>() * 2.0 as usize) as u32, 2, gl::INT),
            ShaderDataType::Int3 => ((size_of::<i32>() * 3.0 as usize) as u32, 3, gl::INT),
            ShaderDataType::Float2x2 => ((size_of::<f32>() * 4.0 as usize) as u32, 4, gl::FLOAT),
            ShaderDataType::Float3x3 => ((size_of::<f32>() * 9.0 as usize) as u32, 9, gl::FLOAT),
            ShaderDataType::Float4x4 => ((size_of::<f32>() * 16.0 as usize) as u32, 16, gl::FLOAT),
        }
    }
}