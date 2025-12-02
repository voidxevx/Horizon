use gl::{types::*};

use crate::rendering::mesh_data::buffer::*;

#[allow(unused)]
pub struct VertexArray {
    pub id: GLuint,
    array_buffer: Buffer,
    index_buffer: Buffer,
}

#[allow(unused)]
impl VertexArray {
    pub fn new(array_buffer: Buffer, index_buffer: Buffer) -> Self {
        array_buffer.bind();
        index_buffer.bind();

        let mut id = 0;
        unsafe {
            gl::GenVertexArrays(1, &mut id);
            gl::BindVertexArray(id);
        }

        array_buffer.unbind();
        index_buffer.unbind();

        Self {
            id: id,
            index_buffer: index_buffer,
            array_buffer: array_buffer,
        }
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
}

impl Drop for VertexArray {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteVertexArrays(1, [self.id].as_ptr());
        }
    }
}