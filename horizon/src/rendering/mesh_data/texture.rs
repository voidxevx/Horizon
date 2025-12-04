use gl::types::*;
use image::EncodableLayout;
use std::path::Path;
use image::error::ImageError;

#[allow(unused)]
pub struct Texture {
    pub id: GLuint,
}

#[allow(unused)]
impl Texture {
    pub fn new() -> Self {
        unsafe {
            let mut id: GLuint = 0;
            gl::GenTextures(1, &mut id);
            Self { id }
        }
    }

    pub fn bind(&self) {
        unsafe {
            gl::BindTexture(gl::TEXTURE_2D, self.id);
        }
    }

    pub fn load(&self, path: &str) -> Result<(), ImageError> {
        unsafe
        {
            self.bind();;
            let image_path: &Path = Path::new(path);
            let img = image::open(image_path)?.into_rgba8();
                gl::TexImage2D(gl::TEXTURE_2D, 
                0, 
                gl::RGBA as i32, 
                img.width() as i32, 
                img.height() as i32, 
                0, gl::RGBA,
                gl::UNSIGNED_BYTE,
                img.as_bytes().as_ptr() as *const _
            );
            gl::GenerateMipmap(gl::TEXTURE_2D);
            Ok(())
        }
    }

    pub fn activate(&self, unit: GLuint) {
        unsafe 
        {
            gl::ActiveTexture(unit);
            self.bind();
        }
    }

    pub fn set_wrapping(&self, mode: GLuint) {
        self.bind();
        unsafe {
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, mode as GLint);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, mode as GLint);
        }
    }
}

impl Drop for Texture {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteTextures(1, [self.id].as_ptr());
        }
    }
}