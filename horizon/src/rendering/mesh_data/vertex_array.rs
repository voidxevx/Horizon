use std::sync::Arc;

use gl::{types::*};

use crate::{rendering::{material::Material}, tools::math::vector::{Vec3, Vec2}}; 

#[allow(unused)]
pub struct VertexArray {
    pub id: GLuint,
}

#[allow(unused)]
impl VertexArray {
    pub fn new() -> Arc<Self> {

        let mut id = 0;
        unsafe {
            gl::GenVertexArrays(1, &mut id);
            gl::BindVertexArray(id);
        }

        Arc::new(Self {
                    id: id,
                })
    }

    pub fn bind(&self) {
        unsafe {
            gl::BindVertexArray(self.id);
        }
    }

    pub fn unbind() {
        unsafe {
            gl::BindVertexArray(0);
        }
    }

    pub fn set_attribute<V: Sized>(
        &self,
        attrib_pos: GLuint,
        components: GLint,
        offset: GLint,
    ) {
        self.bind();
        unsafe {
            gl::VertexAttribPointer(
                attrib_pos,
                components, 
                gl::FLOAT, 
                gl::FALSE, 
                std::mem::size_of::<V>() as GLint,
                offset as *const _,
            );
            gl::EnableVertexAttribArray(attrib_pos);
        }
    }

}

impl Drop for VertexArray {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteVertexArrays(1, [self.id].as_ptr());
        }
    }
}