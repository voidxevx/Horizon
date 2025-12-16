use crate::rendering::mesh_data::shader_data_type::ShaderDataType;

#[allow(unused)]
pub struct VertexLayout {
    attributes: Vec<ShaderDataType>,
    stride: u32,
}

#[allow(unused)]
impl VertexLayout {
    pub fn new(attribs: Vec<ShaderDataType>) -> Self {
        let mut stride: u32 = 0;
        for attrib in &attribs {
            stride += attrib.get_size().1;
        }
        Self { 
            attributes: attribs,
            stride: stride,
        }
    }

    pub fn bind(&self) {
        let mut index: u32 = 0;
        let mut offset: u32 = 0;
        for attrib in &self.attributes {
            let (size, components, type_) = attrib.get_size();
            unsafe 
            {
                gl::VertexAttribPointer(
                    index,
                    size as i32,
                    type_,
                    gl::FALSE,
                    self.stride as i32,
                    offset as *const _,
                )
            }

            offset += size;
            index += 1;
        }
    }
}

