use crate::{Drawable, RenderPipeline, Vertices, gl};
use std::time::Duration;
use crate::gl_wrap::{Shader, VertexBuffer, VertexArrayObject, ElementArrayBuffer};
use crate::objects::Sprite;
use bitflags::_core::ptr::null;

const SHADER_SPRITE_SRC_FRAG: &str = "
#version 330 core
out vec4 FragColor;

in vec2 TexPos;

uniform sampler2D sTexture;

void main()
{
    FragColor = texture(sTexture, TexPos);
}
";

const SHADER_SPRITE_SRC_VERT: &str = "
#version 330 core
layout (location = 0) in vec3 iPos;
layout (location = 1) in vec2 iTexPos;

out vec2 TexPos;

uniform mat4 iMVP;

void main()
{
    gl_Position = iMVP * vec4(iPos.xyz, 1.0);
    TexPos = iTexPos;
}
";


pub struct SpriteBatch<'a> {
    vbo: VertexBuffer,
    vao: VertexArrayObject,
    ebo: ElementArrayBuffer,

    shader: Shader,
    draw_objects: Vec<&'a Sprite>,

    tmp_data_vertices: Vec<f32>,
    tmp_data_indices: Vec<i32>
}

impl<'a> SpriteBatch<'a> {
    pub fn new(_: &RenderPipeline, custom_shader: Option<Shader>) -> SpriteBatch {
        let shader = custom_shader.unwrap_or(Shader::new(SHADER_SPRITE_SRC_FRAG, SHADER_SPRITE_SRC_VERT).unwrap());

        let vao = VertexArrayObject::new();
        let vbo = VertexBuffer::new();
        let ebo = ElementArrayBuffer::new();

        vao.bind_to(&vbo, &ebo);

        SpriteBatch {
            vbo,
            vao,
            ebo,

            shader,
            draw_objects: Vec::new(),

            tmp_data_vertices: Vec::new(),
            tmp_data_indices: Vec::new()
        }
    }

    pub fn draw(&mut self, sprite: &'a Sprite) {
        self.draw_objects.push(sprite);
    }
}

impl Drawable for SpriteBatch<'_> {
    fn update(&mut self, _: &Duration) {
        unimplemented!()
    }

    fn render(&mut self, _pipeline: &RenderPipeline, _: &Duration) {
        self.shader.bind();
        self.vao.bind();

        for draw_object in &self.draw_objects {
            for vertex in draw_object.vertices() {
                self.tmp_data_vertices.push(vertex);
            }

            for index in draw_object.indices() {
                self.tmp_data_indices.push(index);
            }
        }

        self.vbo.update_data(&self.tmp_data_vertices);

        unsafe {
            gl::DrawElements(gl::TRIANGLES, self.draw_objects.len() as i32 * 6, gl::UNSIGNED_INT, null());
        }

        self.vao.unbind();
        self.shader.unbind();
    }
}