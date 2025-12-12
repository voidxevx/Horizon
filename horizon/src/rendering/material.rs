use crate::{rendering::mesh_data::{shader::{ShaderError, ShaderProgram, ShaderUniform, ShaderUniformTemplate, ShaderUniformType, generate_shader}, texture::{SOCKET_MAP, Texture}}, tools::math::{matrix::{Mat2, Mat3, Mat4, Matrix}, vector::{Vec2, Vec3, Vec4, Vector}}};
use std::{collections::HashMap, fs::File, io::{BufRead, BufReader}, path::Path, sync::Arc};

#[allow(unused)]
pub struct TextureSocket {
    index: i32,
    texture: Texture,
}

#[allow(unused)]
#[derive(Debug)]
pub struct ShaderLayoutAttrib {
    position: u32,
    item_count: u32,
    item_type: u32,
    normalized: u8,
    offset: u32,
}

#[allow(unused)]
#[derive(Debug)]
pub struct ShaderLayout {
    layout_data: Vec<ShaderLayoutAttrib>,
    stride: u32,
}

#[allow(unused)]
pub struct Material {
    pub shader_program: ShaderProgram,
    uniforms: Vec<ShaderUniformTemplate>,
    textures: Vec<TextureSocket>,
    layout: ShaderLayout,
}

#[allow(unused)]
impl Material {
    pub fn new(file_path: &str) -> Result<Arc<Self>, ShaderError> {
        let path: &Path = Path::new(file_path);
        if path.exists() && path.is_file() {
            let material_file = File::open(path)
                .expect("Error openning material file");
            let reader = BufReader::new(material_file);

            let mut shader_path: String = String::new();
            let mut texture_sockets: Vec<TextureSocket> = Vec::new();

            enum MaterialParseTarget {
                None,
                ShaderSource,
                TextureSockets,
            }

            let mut socket_index = 0;
            let mut target: MaterialParseTarget = MaterialParseTarget::None;
            for line in reader.lines() {
                if line.is_ok() {
                    let buffer: String = line.unwrap();
                    match buffer.trim() {
                        "[source]" => target = MaterialParseTarget::ShaderSource,
                        "[textures]" => target = MaterialParseTarget::TextureSockets,
                        _ => match target {
                            MaterialParseTarget::ShaderSource => {
                                shader_path = buffer.clone();
                                target = MaterialParseTarget::None;
                            },
                            MaterialParseTarget::TextureSockets => {
                                let texture: Texture = Texture::new();
                                    texture.load(&buffer);
                                    texture.set_wrapping(gl::REPEAT);
                                texture_sockets.push(TextureSocket { 
                                    index: socket_index, 
                                    texture: texture
                                });
                                target = MaterialParseTarget::None;
                            }
                            _ => (),
                        }
                    }
                }
            }

            let shader_program: ShaderProgram = generate_shader(shader_path.as_str())?;

            let shader_uniforms = shader_program.get_uniforms();
            let layout = unsafe { Self::get_layout(&shader_program.id) };

            Ok(Arc::new(Self { 
                shader_program: shader_program, 
                uniforms: shader_uniforms,
                textures: texture_sockets,
                layout: layout,
            }))
        }
        else{
            Err(ShaderError::CompilationError(String::from("unable to find material file")))
        }

    }


    #[allow(unsafe_op_in_unsafe_fn)]
    unsafe fn get_layout(shader_id: &u32) -> ShaderLayout {
        let mut layout: ShaderLayout = ShaderLayout { 
            layout_data: Vec::new(),
            stride: 0
        };

        let mut count = 0;
        let mut buffer_size = 0;
        gl::GetProgramiv(*shader_id, gl::ACTIVE_ATTRIBUTES, &mut count);
        gl::GetProgramiv(*shader_id, gl::ACTIVE_ATTRIBUTE_MAX_LENGTH, &mut buffer_size);

        let mut name = vec![0u8; buffer_size as usize];
        let mut name_length = 0;
        let mut attrib_size = 0;
        let mut attrib_type = 0;

        let mut current_offset = 0;

        for i in 0..count {
            gl::GetActiveAttrib(
                *shader_id, 
                i as u32, 
                buffer_size, 
                &mut name_length, 
                &mut attrib_size,
                &mut attrib_type,
                name.as_mut_ptr() as *mut i8
            );


            let mut attrib_item_count: u32 = 0;
            let mut attrib_offset_size: u32 = 0;
            
            match attrib_type {
                gl::FLOAT_VEC2 => {
                    attrib_item_count = 2;
                    attrib_offset_size = 2 * size_of::<f32>() as u32;
                },
                gl::FLOAT_VEC3 => {
                    attrib_item_count = 3;
                    attrib_offset_size = 3 * size_of::<f32>() as u32;
                },
                gl::FLOAT_VEC4 => {
                    attrib_item_count = 4;
                    attrib_offset_size = 4 * size_of::<f32>() as u32;
                },
                gl::FLOAT_MAT2 => {
                    attrib_item_count = 4;
                    attrib_offset_size = 4 * size_of::<f32>() as u32;
                },
                gl::FLOAT_MAT3 => {
                    attrib_item_count = 9;
                    attrib_offset_size = 9 * size_of::<f32>() as u32;
                },
                gl::FLOAT_MAT4 => {
                    attrib_item_count = 16;
                    attrib_offset_size = 16 * size_of::<f32>() as u32;
                },
                gl::INT => {
                    attrib_item_count = 1;
                    attrib_offset_size = 16 * size_of::<i32>() as u32;
                }
                // ... as more data implemented.
                _ => (),
            };


            let layout_attrib: ShaderLayoutAttrib = ShaderLayoutAttrib { 
                position: i as u32,
                item_count: attrib_item_count,
                item_type: attrib_type,
                normalized: gl::FALSE, 
                offset: current_offset,
            };

            current_offset += attrib_offset_size;

            layout.layout_data.push(layout_attrib);
        }

        layout.stride = current_offset;
        println!("{:?}", layout);

        layout
    }

    #[allow(unsafe_op_in_unsafe_fn)]
    pub unsafe fn bind_layout(&self) {
        let layout = &self.layout;

        for attrib in &layout.layout_data {
            println!("binding attrib pointer");
            gl::VertexAttribPointer(
                attrib.position,
                attrib.item_count as i32,
                attrib.item_type,
                attrib.normalized,
                layout.stride as i32,
                attrib.offset as *const _,
            );
            println!("enabling attribute");
            gl::EnableVertexAttribArray(attrib.position);
            println!("bind successful");
        }
    }

    pub fn get_uniforms(&self) -> &Vec<ShaderUniformTemplate> {
        &self.uniforms
    }

    pub fn bind_textures(&self) {
        for socket in &self.textures {
           socket.texture.activate(SOCKET_MAP[&(socket.index as u32)]);
        }
    }

}

#[allow(unused)]
pub struct MaterialInstance {
    parent_material: Arc<Material>,
    uniforms: HashMap<String, ShaderUniform>,
}

#[allow(unused)]
impl MaterialInstance {
    pub fn apply(&self) {
        self.parent_material.shader_program.apply();
        for (name, value) in &self.uniforms {
            self.parent_material.shader_program.set_uniform(name.trim(), value);
        }

        self.parent_material.bind_textures();
    }

    pub fn set_uniform(&mut self, name: &String, value: ShaderUniform) {
        let current_uniform = self.uniforms.get_mut(name)
            .expect("Failed to set uniform value, does not exist");
        *current_uniform = value;
    }

    pub fn get_shader_program(&self) -> &ShaderProgram {
        &self.parent_material.shader_program
    }
}

#[allow(unused)]
pub fn instance_material(parent: Arc<Material>) -> MaterialInstance {
    let mut created_uniforms: HashMap<String, ShaderUniform> = HashMap::new();

    let uniforms = parent.get_uniforms();
    for uniform in uniforms {
        match uniform.val_type {
            ShaderUniformType::FloatUniform => {
                    created_uniforms.insert(uniform.name.clone(), ShaderUniform::FloatUniform(0.0));
                },
            ShaderUniformType::IntUniform => {
                    created_uniforms.insert(uniform.name.clone(), ShaderUniform::IntUniform(0));
                },
            ShaderUniformType::VectorUniform(len) => match len {
                2 => {
                    created_uniforms.insert(uniform.name.clone(), ShaderUniform::VectorUnform(Vector::Length2(Vec2::new([0.0, 0.0]))));
                },
                3 => {
                    created_uniforms.insert(uniform.name.clone(), ShaderUniform::VectorUnform(Vector::Length3(Vec3::new([0.0, 0.0, 0.0,]))));
                },
                4 => {
                    created_uniforms.insert(uniform.name.clone(), ShaderUniform::VectorUnform(Vector::Length4(Vec4::new([0.0, 0.0, 0.0, 0.0]))));
                },
                _ => ()
            },
            ShaderUniformType::MatrixUniform(rows, cols) => {
                match (rows, cols) {
                    (2, 2) => {
                        created_uniforms.insert(uniform.name.clone(), ShaderUniform::MatrixUniform(Matrix::SquareLength2(Mat2::new([
                            1.0, 0.0, 
                            0.0, 1.0
                        ]))));
                    },
                    (3, 3) => {
                        created_uniforms.insert(uniform.name.clone(), ShaderUniform::MatrixUniform(Matrix::SquareLength3(Mat3::new([
                            1.0, 0.0, 0.0, 
                            0.0, 1.0, 0.0, 
                            0.0, 0.0, 1.0
                        ]))));
                    },
                    (4, 4) => {
                        created_uniforms.insert(uniform.name.clone(), ShaderUniform::MatrixUniform(Matrix::SquareLength4(Mat4::new([
                            1.0, 0.0, 0.0, 0.0, 
                            0.0, 1.0, 0.0, 0.0, 
                            0.0, 0.0, 1.0, 0.0, 
                            0.0, 0.0, 0.0, 1.0,
                        ]))));
                    },
                    (_,_) => ()
                }
            },
            _ => (),
        }
    }

    MaterialInstance { parent_material: parent.clone(), uniforms: created_uniforms }
}