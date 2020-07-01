// EBO

use std::ffi::c_void;

use crate::gl;

pub struct ElementArrayBuffer {
    gl_id: u32
}

impl ElementArrayBuffer {
    pub fn new() -> ElementArrayBuffer {
        let mut buff = ElementArrayBuffer {
            gl_id: 0
        };

        unsafe {
            gl::GenBuffers(1, &mut buff.gl_id);
        }

        buff
    }

    pub fn id(&self) -> u32 {
        self.gl_id
    }

    pub fn update_data(&mut self, indices: &[i32]) {
        unsafe {
            self.bind();

            gl::BufferData(gl::ELEMENT_ARRAY_BUFFER,
                           (indices.len() * std::mem::size_of::<i32>()) as _,
                           &indices[0] as *const i32 as *const c_void,
                           gl::STATIC_DRAW
            );

            self.unbind();
        }
    }


    pub fn bind(&self) {
        unsafe {
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, self.id());
        }
    }

    pub fn unbind(&self) {
        unsafe {
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, 0);
        }
    }
}

impl Drop for ElementArrayBuffer {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteBuffers(1, &mut self.gl_id);
        }
    }
}
