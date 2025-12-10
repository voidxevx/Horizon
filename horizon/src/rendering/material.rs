use crate::{rendering::mesh_data::{shader::{ShaderError, ShaderProgram, ShaderUniform, ShaderUniformTemplate, ShaderUniformType, generate_shader}, texture::{SOCKET_MAP, Texture}}, tools::math::{matrix::{Mat2, Mat3, Mat4, Matrix}, vector::{Vec2, Vec3, Vec4, Vector}}};
use std::{collections::HashMap, fs::File, io::{BufRead, BufReader}, path::Path, sync::Arc};

#[allow(unused)]
pub struct TextureSocket {
    index: i32,
    texture: Texture,
}

#[allow(unused)]
pub struct Material {
    pub shader_program: ShaderProgram,
    uniforms: Vec<ShaderUniformTemplate>,
    textures: Vec<TextureSocket>
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

            println!("uniforms: {:?}", shader_uniforms);

            Ok(Arc::new(Self { 
                shader_program: shader_program, 
                uniforms: shader_uniforms,
                textures: texture_sockets,
            }))
        }
        else{
            Err(ShaderError::CompilationError(String::from("unable to find material file")))
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
}

#[allow(unused)]
pub fn instance_material(parent: Arc<Material>) -> MaterialInstance {
    let mut created_uniforms: HashMap<String, ShaderUniform> = HashMap::new();

    let uniforms = parent.get_uniforms();
    for uniform in uniforms {
        match uniform.val_type {
            ShaderUniformType::FloatUniform => {
                    created_uniforms.insert(uniform.name.clone(), ShaderUniform::FloatUniform(0.0))
                        .expect("unable to create uniform");
                },
            ShaderUniformType::IntUniform => {
                    created_uniforms.insert(uniform.name.clone(), ShaderUniform::IntUniform(0))
                        .expect("unable to create uniform");
                },
            ShaderUniformType::VectorUniform(len) => match len {
                2 => {
                    created_uniforms.insert(uniform.name.clone(), ShaderUniform::VectorUnform(Vector::Length2(Vec2::new([0.0, 0.0]))))
                        .expect("unable to create uniform");
                },
                3 => {
                    created_uniforms.insert(uniform.name.clone(), ShaderUniform::VectorUnform(Vector::Length3(Vec3::new([0.0, 0.0, 0.0,]))))
                        .expect("unable to create uniform");
                },
                4 => {
                    created_uniforms.insert(uniform.name.clone(), ShaderUniform::VectorUnform(Vector::Length4(Vec4::new([0.0, 0.0, 0.0, 0.0]))))
                        .expect("unable to create uniform");
                },
                _ => ()
            },
            ShaderUniformType::MatrixUniform(rows, cols) => {
                match (rows, cols) {
                    (2, 2) => {
                        created_uniforms.insert(uniform.name.clone(), ShaderUniform::MatrixUniform(Matrix::SquareLength2(Mat2::new([
                            1.0, 0.0, 
                            0.0, 1.0
                        ]))))
                            .expect("unable to create uniform");
                    },
                    (3, 3) => {
                        created_uniforms.insert(uniform.name.clone(), ShaderUniform::MatrixUniform(Matrix::SquareLength3(Mat3::new([
                            1.0, 0.0, 0.0, 
                            0.0, 1.0, 0.0, 
                            0.0, 0.0, 1.0
                        ]))))  
                            .expect("unable to create uniform");
                    },
                    (4, 4) => {
                        created_uniforms.insert(uniform.name.clone(), ShaderUniform::MatrixUniform(Matrix::SquareLength4(Mat4::new([
                            1.0, 0.0, 0.0, 0.0, 
                            0.0, 1.0, 0.0, 0.0, 
                            0.0, 0.0, 1.0, 0.0, 
                            0.0, 0.0, 0.0, 1.0,
                        ]))))
                            .expect("unable to create uniform");
                    },
                    (_,_) => ()
                }
            },
            _ => (),
        }
    }

    MaterialInstance { parent_material: parent, uniforms: created_uniforms }
}