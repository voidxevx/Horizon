use std::ffi::{CString, NulError};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::{ptr};
use thiserror::Error;
use std::string::FromUtf8Error;
use gl::types::*;
use std::path::Path;

use crate::tools::math::matrix::{Mat4, Matrix};

#[allow(unused)]
#[derive(Debug, Error)]
pub enum ShaderError {
    #[error("Error compiling shader: {0}")]
    CompilationError(String),
    #[error("Error linking shaders: {0}")]
    LinkingError(String),
    #[error("{0}")]
    Utf8Error(#[from] FromUtf8Error),
    #[error("{0}")]
    NulError(#[from] NulError),
}

#[allow(unused)]
pub struct Shader {
    pub id: GLuint,
}

#[allow(unused)]
impl Shader {
    pub fn new(source_code: &str, shader_type: GLenum) -> Result<Self, ShaderError>
    {
        unsafe {
            let source_code = CString::new(source_code)?;
            let shader = Self {
                id: gl::CreateShader(shader_type),
            };

            gl::ShaderSource(shader.id, 1, &source_code.as_ptr(), ptr::null());
            gl::CompileShader(shader.id);

            let mut success: GLint = 0;
            gl::GetShaderiv(shader.id, gl::COMPILE_STATUS, &mut success);

            if success == 1 {
                Ok(shader)
            } else {
                let mut error_log_size: GLint = 0;
                gl::GetShaderiv(shader.id, gl::INFO_LOG_LENGTH, &mut error_log_size);
                let mut error_log: Vec<u8> = Vec::with_capacity(error_log_size as usize);
                gl::GetShaderInfoLog(
                    shader.id,
                    error_log_size,
                    &mut error_log_size,
                    error_log.as_mut_ptr() as *mut _,
                );

                error_log.set_len(error_log_size as usize);
                let log = String::from_utf8(error_log)?;
                Err(ShaderError::CompilationError(log))
            }
        }
    }
}

impl Drop for Shader {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteShader(self.id);
        }
    }
}



#[allow(unused)]
pub struct ShaderProgram {
    pub id: GLuint,
}

#[allow(unused)]
impl ShaderProgram {
    pub fn new(shaders: &[Shader]) -> Result<Self, ShaderError> {
        unsafe {
            let program = Self {
                id: gl::CreateProgram(),
            };

            for shader in shaders {
                gl::AttachShader(program.id, shader.id);
            }

            gl::LinkProgram(program.id);

            let mut success: GLint = 0;
            gl::GetProgramiv(program.id, gl::LINK_STATUS, &mut success);

            if success == 1 {
                Ok(program)
            } else {
                let mut error_log_size: GLint = 0;
                gl::GetProgramiv(program.id, gl::INFO_LOG_LENGTH, &mut error_log_size);
                let mut error_log: Vec<u8> = Vec::with_capacity(error_log_size as usize);
                gl::GetProgramInfoLog(
                    program.id,
                    error_log_size,
                    &mut error_log_size,
                    error_log.as_mut_ptr() as *mut _,
                );

                error_log.set_len(error_log_size as usize);
                let log = String::from_utf8(error_log)?;
                Err(ShaderError::LinkingError(log))
            }
        }
    }

    pub fn apply(&self) {
        unsafe {
            gl::UseProgram(self.id);
        }
    }

    pub fn get_attrib_location(&self, attrib: &str) -> Result<GLuint, NulError> {
        let attrib = CString::new(attrib)?;
        unsafe {
            Ok(gl::GetAttribLocation(self.id, attrib.as_ptr()) as GLuint)
        }
    }

    pub fn set_int_uniform(&self, name: &str, value: i32) -> Result<(), ShaderError> {
        unsafe {
            self.apply();
            let uniform = CString::new(name)?;
            gl::Uniform1i(gl::GetUniformLocation(self.id, uniform.as_ptr()), value);
            Ok(())
        }
    }

    pub fn set_mat_uniform(&self, name: &str, value: Matrix) -> Result<(), ShaderError> {
        unsafe {
            match value {
                Matrix::SquareLength2(mat2) => {
                    Ok(())
                },
                Matrix::SquareLength3(mat3) => {
                    Ok(())
                }
                Matrix::SquareLength4(mat4) => {
                    self.apply();
                    let uniform: CString = CString::new(name)?;
                    gl::UniformMatrix4fv(
                        gl::GetUniformLocation(self.id, uniform.as_ptr()),
                        1, 
                        gl::FALSE,
                        mat4.data.as_ptr()
                    );
                    Ok(())
                }
            }
        }
    }
}

#[macro_export]
macro_rules! set_attribute {
    ($vbo:ident, $pos:tt, $t:ident :: $field:tt) => { unsafe {
        let dummy = core::mem::MaybeUninit::<$t>::uninit();
        let dummy_ptr = dummy.as_ptr();
        let member_ptr = core::ptr::addr_of!((*dummy_ptr).$field);
        const fn size_of_raw<T>(_: *const T) -> usize {
            core::mem::size_of::<T>()
        }
        let member_offset = member_ptr as i32 - dummy_ptr as i32;
        $vbo.set_attribute::<$t>(
            $pos,
            (size_of_raw(member_ptr) / core::mem::size_of::<f32>()) as i32,
            member_offset,
        )
    }};
}

impl Drop for ShaderProgram {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteProgram(self.id);
        }
    }
}


#[allow(unused)]
enum ShaderTarget {
    None,
    Version,
    Vertex,
    Fragment,
}

#[allow(unused)]
pub fn generate_shader(file_path: &str) -> Result<ShaderProgram, ShaderError> {
    let path = Path::new(file_path);
    if path.exists() && path.is_file()
    {
        let shader_file = File::open(path)
            .expect("Error openning shader file");
        let mut reader = BufReader::new(shader_file);

        // buffers for shader source code.
        let mut vertex_shader_src: String = String::new();
        let mut fragment_shader_src: String = String::new();
        let mut shader_version: String = String::new();

        // file buffer & source target
        let mut target: ShaderTarget = ShaderTarget::None;
        for line in reader.lines() {
            if line.is_ok()
            {
                let buffer: String = line.unwrap();
                match buffer.trim() {
                    "[version]" => target = ShaderTarget::Version,
                    "[vertex]" => target = ShaderTarget::Vertex,
                    "[fragment]" => target = ShaderTarget::Fragment,
                    _ => {
                        match target {
                            ShaderTarget::Version => {
                                shader_version = buffer.clone();
                                vertex_shader_src.push_str("#version ");
                                vertex_shader_src.push_str(&buffer.clone());
                                vertex_shader_src.push('\n');

                                fragment_shader_src.push_str("#version ");
                                fragment_shader_src.push_str(&buffer.clone());
                                fragment_shader_src.push('\n');
                                target = ShaderTarget::None;
                            },
                            ShaderTarget::Vertex => {
                                vertex_shader_src.push_str(&buffer.clone());
                                vertex_shader_src.push('\n');
                            },
                            ShaderTarget::Fragment => {
                                fragment_shader_src.push_str(&buffer.clone());
                                fragment_shader_src.push('\n');
                            },
                            _ => ()
                        }
                    }
                }
            }
        }

        let vertex_shader: Shader = Shader::new(&vertex_shader_src, gl::VERTEX_SHADER)
            .expect("Error creating vertex shader");
        let fragment_shader: Shader = Shader::new(&fragment_shader_src, gl::FRAGMENT_SHADER)
            .expect("Error creating fragment shader");

        Ok(ShaderProgram::new(&[vertex_shader, fragment_shader])
            .expect("Error creating shader program"))

    } else {
        Err(ShaderError::LinkingError(String::from("Invalid File")))
    }
}