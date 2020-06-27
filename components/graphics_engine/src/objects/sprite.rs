use image::RgbaImage;
use std::time::Duration;

use super::gl::Texture2D;
use crate::objects::gl::{VertexArrayObject, VertexBuffer, Shader, ElementArrayBuffer};
use crate::Vertices;
use std::ptr::null;

// a High level struct for drawing sprites to the screen
// TODO: Spritebatch for increased performance by having hundreds of sprites
pub struct Sprite {
    /*
    texture: Texture2D,
    material: Material
    */

    // TODO: This MUST be sprite batched
    vbo: VertexBuffer,
    vao: VertexArrayObject,
    ebo: ElementArrayBuffer,

    shader: Shader,
    texture: Texture2D
}

impl Sprite {
    pub fn new(img: RgbaImage) -> Sprite {
        let vbo = VertexBuffer::new(&Sprite::vertices());
        let vao = VertexArrayObject::new();
        let ebo = ElementArrayBuffer::new(&Sprite::indices());

        vao.bind_to(&vbo, &ebo);

        let shader = Shader::new(
            "
                #version 330 core
                out vec4 FragColor;

                in vec2 TexPos;

                uniform sampler2D sTexture;

                void main()
                {
                    FragColor = texture(sTexture, TexPos);
                }
            ",

            "
                #version 330 core
                layout (location = 0) in vec3 iPos;
                layout (location = 1) in vec2 iTexPos;

                out vec2 TexPos;

                void main()
                {
                    gl_Position = vec4(iPos.x, iPos.y, iPos.z, 1.0);
                    TexPos = iTexPos;
                }
            "
        ).unwrap();

        let texture = Texture2D::from(img);

        Sprite {
            vbo,
            vao,
            ebo,
            shader,
            texture
        }
    }
}

impl crate::Drawable for Sprite {
    fn update(&self, _: &Duration) { } // Ignored, we dont need that right now

    fn render(&self, _delta: &Duration) {
        self.texture.bind();
        self.shader.bind();
        self.vao.bind();

        unsafe {
            gl::DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_INT, null());
        }

        self.vao.unbind();
        self.shader.unbind();
        self.texture.unbind();
    }
}

impl Vertices for Sprite {
    fn vertices() -> Vec<f32> {
        vec![
            // Quad
           //X     Y    Z       TX   TY
             1.0,  1.0, 0.0,    1.0, 0.0, // top right
             1.0, -1.0, 0.0,    1.0, 1.0, // bottom right
            -1.0, -1.0, 0.0,    0.0, 1.0, // bottom left
            -1.0,  1.0, 0.0,    0.0, 0.0, // top left
        ]
    }

    fn indices() -> Vec<i32> {
        vec![
            0, 1, 3,
            1, 2, 3
        ]
    }
}
