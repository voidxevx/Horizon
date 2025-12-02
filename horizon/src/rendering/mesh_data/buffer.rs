use gl::types::*;

#[allow(unused)]
pub struct Buffer {
    pub id: GLuint,
    target: GLuint,
}

#[allow(unused)]
impl Buffer {
    pub fn new(target: GLuint) -> Self {
        let mut id: GLuint = 0;
        unsafe {gl::GenBuffers(1, &mut id); }
        Self { id, target }
    }

    pub fn bind(&self) {
        unsafe {
            gl::BindBuffer(self.target, self.id);
        }
    }

    pub fn unbind(&self) {
        unsafe {
            gl::BindBuffer(self.target, 0);
        }
    }

    pub fn buffer_data<D>(&self, data: &[D], usage: GLuint) {
        unsafe {
            self.bind();
            let (_, data_bytes, _) = data.align_to::<u8>();
            gl::BufferData(
                self.target,
                data_bytes.len() as GLsizeiptr,
                data_bytes.as_ptr() as *const _,
                usage
            );
        }
    }
}

impl Drop for Buffer {
    fn drop(&mut self) {
        unsafe {
            self.unbind();
            gl::DeleteBuffers(1, [self.id].as_ptr());
        }
    }
}

#[allow(unused)]
pub struct MutliBuffer {
    ids: Vec<GLuint>,
    targets: Vec<GLuint>,
}

#[allow(unused)]
impl MutliBuffer {
    pub fn new() -> Self {
        Self {
            ids: Vec::new(),
            targets: Vec::new(),
        }
    }

    pub fn create(&mut self, target: GLuint) {
        let mut id: GLuint = 0;
        unsafe { gl::GenBuffers(1, &mut id); }
        self.ids.push(id);
        self.targets.push(target);
    }

    pub fn bind(&self, buffer: usize) {
        if let Some(target) = self.targets.get(buffer) 
        && let Some(id) = self.ids.get(buffer)
        {
            unsafe {
                gl::BindBuffer(*target, *id);
            }
        }
    }

    pub fn unbind(&self, buffer: usize) {
        if let Some(target) = self.targets.get(buffer) {
            unsafe {
                gl::BindBuffer(*target, 0);
            }
        }
    }

    pub fn set_buffer_data<D>(&self, buffer: usize, data: &[D], usage: GLuint) {
        unsafe {
            self.bind(buffer);
            let (_, data_bytes, _) = data.align_to::<u8>();
            if let Some(target) = self.targets.get(buffer) {
                gl::BufferData(
                    *target,
                    data_bytes.len() as GLsizeiptr,
                    data_bytes.as_ptr() as *const _,
                    usage,
                )
            }
        }
    }

}

impl Drop for MutliBuffer {
    fn drop(&mut self) {
        for i in 0..self.ids.len() {
            let id = self.ids[i];
            self.unbind(i);
            unsafe {
                gl::DeleteBuffers(1, [id].as_ptr())
            }
        }
    }
}