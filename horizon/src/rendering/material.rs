use crate::rendering::mesh_data::{shader::{ShaderError, ShaderProgram, ShaderUniform, ShaderUniformTemplate, generate_shader}, texture::Texture};
use std::{collections::HashMap, fs::File, io::{BufRead, BufReader}, path::Path, rc::Rc, sync::Arc};

#[allow(unused)]
pub struct TextureSocket {
    index: i32,
    texture: Texture,
}

#[allow(unused)]
pub struct Material {
    shader_program: ShaderProgram,
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
                            },
                            MaterialParseTarget::TextureSockets => {
                                let texture: Texture = Texture::new();
                                    texture.load(&buffer);
                                texture_sockets.push(TextureSocket { 
                                    index: socket_index, 
                                    texture: texture
                                });
                            }
                            _ => (),
                        }
                    }
                }
            }


            let shader_program: ShaderProgram = generate_shader(shader_path.as_str())?;
            let shader_uniforms = shader_program.get_uniforms();

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

}

#[allow(unused)]
pub struct MaterialInstance {
    parent_material: Arc<Material>,
    uniforms: HashMap<String, ShaderUniform>,
}

// TODO: material instance generation.