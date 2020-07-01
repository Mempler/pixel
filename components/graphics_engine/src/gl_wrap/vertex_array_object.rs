// VAO

use std::ptr;
use std::ffi::c_void;

use super::{ VertexBuffer, ElementArrayBuffer };
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
            // XYZ
            gl::VertexAttribPointer(0, 3, // TODO: do not hardcode all this. let the developer decide.
                                    gl::FLOAT, gl::FALSE,
                                    15*4,
                                    ptr::null());
            gl::EnableVertexAttribArray(0);
            offset += 3 * 4;

            // Texture XY
            gl::VertexAttribPointer(1, 2,
                                    gl::FLOAT, gl::FALSE,
                                    15*4,
                                    offset as *const c_void);
            gl::EnableVertexAttribArray(1);
            offset += 2 * 4;

            // RGBA
            gl::VertexAttribPointer(2, 4,
                                    gl::FLOAT, gl::FALSE,
                                    15*4,
                                    offset as *const c_void);
            gl::EnableVertexAttribArray(2);
            offset += 4 * 4;

            // Flip XY
            gl::VertexAttribPointer(3, 2,
                                    gl::FLOAT, gl::FALSE,
                                    15*4,
                                    offset as *const c_void);
            gl::EnableVertexAttribArray(3);
            offset += 2 * 4;

            // Texture Width, Height
            gl::VertexAttribPointer(4, 2,
                                    gl::FLOAT, gl::FALSE,
                                    15*4,
                                    offset as *const c_void);
            gl::EnableVertexAttribArray(4);
            offset += 2 * 4;

            // Texture Offset XY
            gl::VertexAttribPointer(5, 2,
                                    gl::FLOAT, gl::FALSE,
                                    15*4,
                                    offset as *const c_void);
            gl::EnableVertexAttribArray(5);
            offset += 2 * 4;
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
