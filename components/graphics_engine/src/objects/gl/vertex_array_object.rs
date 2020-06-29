// VAO

use super::VertexBuffer;
use std::ptr;
use std::ffi::c_void;
use crate::objects::gl::ElementArrayBuffer;

use crate::gl;

pub struct VertexArrayObject {
    gl_id: u32
}

impl VertexArrayObject {
    pub fn new() -> VertexArrayObject {
        let mut vao = VertexArrayObject {
            gl_id: 0
        };

        unsafe {
            gl::GenVertexArrays(1, &mut vao.gl_id);
        }

        vao
    }

    pub fn id(&self) -> u32 {
        self.gl_id
    }

    pub fn bind(&self) {
        unsafe {
            gl::BindVertexArray(self.id());
        }
    }

    pub fn unbind(&self) {
        unsafe {
            gl::BindVertexArray(0);
        }
    }

    pub fn bind_to(&self, vbo: &VertexBuffer, ebo: &ElementArrayBuffer) {
        self.bind(); // bind VAO
        vbo.bind(); // bind VBO
        ebo.bind(); // bind EBO

        // 12 XYZ 8 TX TY
        unsafe { // Specify our data location
            let mut offset = 0;
            gl::VertexAttribPointer(0, 3,
                                    gl::FLOAT, gl::FALSE,
                                    5*4,
                                    ptr::null());
            gl::EnableVertexAttribArray(0);
            offset += 3 * 4;

            gl::VertexAttribPointer(1, 2,
                                    gl::FLOAT, gl::FALSE,
                                    5*4,
                                    offset as *const c_void);
            gl::EnableVertexAttribArray(1);
            //offset += 2 * 4;
        }

        self.unbind(); // unbind VAO
        vbo.unbind(); // unbind VBO
        ebo.unbind(); // unbind VBO
    }
}

impl Drop for VertexArrayObject {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteVertexArrays(1, &mut self.gl_id);
        }
    }
}
