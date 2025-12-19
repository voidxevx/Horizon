
pub enum DataTypes {
    Float,
    Float2,
    Float3,
    Float4,
}

impl DataTypes {
    pub fn size(&self) -> (i32, u32, usize) {
        match self {
            Self::Float => (1, gl::FLOAT, size_of::<f32>()),
            Self::Float2 => (2, gl::FLOAT, size_of::<f32>() * 2),
            Self::Float3 => (3, gl::FLOAT, size_of::<f32>() * 3),
            Self::Float4 => (4, gl::FLOAT, size_of::<f32>() * 4),
        }
    }
}