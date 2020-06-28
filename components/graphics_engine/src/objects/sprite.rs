use image::RgbaImage;
use std::time::Duration;

use super::gl::Texture2D;
use crate::objects::gl::{VertexArrayObject, VertexBuffer, Shader, ElementArrayBuffer};
use crate::{Vertices, RenderPipeline};
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
    texture: Texture2D,

    position: glm::Vec2,
    scale: glm::Vec2,
}

impl Sprite {
    pub fn new(img: RgbaImage) -> Sprite {
        let vbo = VertexBuffer::new(&Sprite::vertices());
        let vao = VertexArrayObject::new();
        let ebo = ElementArrayBuffer::new(&Sprite::indices());

        vao.bind_to(&vbo, &ebo);

        let shader = Shader::new(
            "\
                #version 320 es
                precision mediump float;

                out vec4 FragColor;

                in vec2 TexPos;

                uniform sampler2D sTexture;

                void main()
                {
                    FragColor = texture(sTexture, TexPos);
                }
            ",

            "\
                #version 320 es

                layout (location = 0) in vec3 iPos;
                layout (location = 1) in vec2 iTexPos;

                out vec2 TexPos;

                uniform mat4 iMVP;

                void main()
                {
                    gl_Position = iMVP * vec4(iPos.xyz, 1.0);
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
            texture,

            position: glm::Vec2::new(0.0, 0.0),
            scale: glm::Vec2::new(2.0, 2.0)
        }
    }
}

impl crate::Drawable for Sprite {
    fn update(&self, _: &Duration) { } // Ignored, we dont need that right now

    fn render(&self, pipeline: &RenderPipeline, _delta: &Duration) {
        let wnd_size = pipeline.get_window().size();

        self.texture.bind();
        self.shader.bind();
        self.vao.bind();

        let mut proj = glm::ortho(
            -1.0, wnd_size.0 as f32,
            -1.0, wnd_size.1 as f32,
            -1.0, 1.0
        );

        let mut model = glm::one();
        let model = glm::scale(&mut model, &glm::vec3(self.scale.x, self.scale.y, 1.0));

        let mvp = proj * model;

        self.shader.uniform_mat4f("iMVP", &mvp);

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
           //X      Y     Z       TX   TY
             64.0,  64.0, 0.0,    1.0, 0.0, // top right
             64.0,  0.0,  0.0,    1.0, 1.0, // bottom right
             0.0,   0.0,  0.0,    0.0, 1.0, // bottom left
             0.0,   64.0, 0.0,    0.0, 0.0, // top left
        ]
    }

    fn indices() -> Vec<i32> {
        vec![
            0, 1, 3,
            1, 2, 3
        ]
    }
}
