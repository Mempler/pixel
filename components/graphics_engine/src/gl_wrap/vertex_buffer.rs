// VBO

use std::ffi::c_void;

use crate::gl;

pub struct VertexBuffer {
    gl_id: u32
}

impl VertexBuffer {
    pub fn new() -> VertexBuffer {
        let mut buff = VertexBuffer {
            gl_id: 0
        };

        unsafe {
            gl::GenBuffers(1, &mut buff.gl_id);

            /*
            // Bind so we can use our VBO
            buff.bind();

            // Upload our vertices to our GPU
            gl::BufferData(gl::ARRAY_BUFFER,
                           (vertices.len() * std::mem::size_of::<f32>()) as _,
                            &vertices[0] as *const f32 as *const c_void,
                            gl::DYNAMIC_DRAW
            );

            // We dont need it anymore, lets unbind it for now! we can always rebind it later
            buff.unbind();
            */
        }

        buff
    }

    pub fn id(&self) -> u32 {
        self.gl_id
    }

    pub fn update_data(&mut self, vertices: &[f32]) {
        unsafe {
            self.bind();

            gl::BufferData(gl::ARRAY_BUFFER,
                           (vertices.len() * std::mem::size_of::<f32>()) as _,
                           &vertices[0] as *const f32 as *const c_void,
                           gl::DYNAMIC_DRAW
            );

            self.unbind();
        }
    }

    pub fn bind(&self) {
        unsafe {
            gl::BindBuffer(gl::ARRAY_BUFFER, self.id());
        }
    }

    pub fn unbind(&self) {
        unsafe {
            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
        }
    }
}

impl Drop for VertexBuffer {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteBuffers(1, &mut self.gl_id);
        }
    }
}
