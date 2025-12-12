use crate::rendering::{material::MaterialInstance, mesh_data::vertex_array::VertexArray};

#[allow(unused)]
pub struct StaticMesh {
    vertex_array: VertexArray,
    material_instance: MaterialInstance,
}

#[allow(unused)]
impl StaticMesh {
    pub fn new(va: VertexArray, mi: MaterialInstance) -> Self {
        unsafe {
            let mut attrib_count: i32 = 0;
            let shader_id = mi.get_shader_program().id;
            gl::GetProgramiv(shader_id, gl::ACTIVE_ATTRIBUTES, &mut attrib_count);

            for i in 0..attrib_count {
                let mut name_length = 0;
                let mut array_size = 0;
                let mut attrib_type = 0;
                let mut name: Vec<u8> = vec![0u8; 256];

                gl::GetActiveAttrib(
                    shader_id, 
                    i as u32, 
                    256, 
                    &mut name_length,
                    &mut array_size, 
                    &mut attrib_type, 
                    name.as_mut_ptr() as *mut i8
                );

                let attrib_name: String = String::from_utf8_lossy(&name[..name_length as usize]).to_string();


                // TODO: bind layouts from gathered metadata. 
            }
        }
        Self {
            vertex_array: va,
            material_instance: mi,
        }
    }
}