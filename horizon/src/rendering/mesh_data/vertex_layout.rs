use crate::rendering::mesh_data::shader_types::DataTypes;

pub struct VertexLayout {
    types: Vec<DataTypes>,
    stride: usize,
}

impl VertexLayout {
    pub fn new() -> Self {
        Self {
            types: Vec::new(),
            stride: 0,
        }
    }

    pub fn attrib(mut self, type_: DataTypes) -> Self {
        self.stride += type_.size().2;
        self.types.push(type_);
        self
    }

    pub fn bind_attribute(&self) {
        let mut index = 0;
        let mut current_offset = 0;
        for attrib in &self.types {
            let (comps, type_, offset) = attrib.size();
            unsafe {
                gl::VertexAttribPointer(
                    index,
                    comps,
                    type_,
                    gl::FALSE,
                    self.stride as i32,
                    current_offset as *const _
                );
                gl::EnableVertexAttribArray(index);
            }
            index += 1;
            current_offset += offset;
        }
    }
}