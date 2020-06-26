// a low level texture binding for sprite

use image::RgbaImage;

pub struct Texture2D {
    texture: u32
}

impl Texture2D {
    pub fn new(pixels: &[u8], width: i32, height: i32) -> Texture2D {
        let mut texture = Texture2D {
            texture: 0
        };

        unsafe {
            gl::GenTextures(1, &mut texture.texture);
            texture.bind();

            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);

            gl::TexImage2D(gl::TEXTURE_2D, 0, gl::RGBA as i32, width, height,
                0, gl::RGBA, gl::UNSIGNED_BYTE, pixels.as_ptr() as _);

            gl::GenerateMipmap(gl::TEXTURE_2D);

            texture.unbind();
        }

        texture
    }

    pub fn id(&self) -> u32 {
        self.texture
    }

    pub fn bind(&self) {
        unsafe {
            gl::BindTexture(gl::TEXTURE_2D, self.texture);
        }
    }

    pub fn unbind(&self) {
        unsafe {
            gl::BindTexture(gl::TEXTURE_2D, 0);
        }
    }
}

impl From<RgbaImage> for Texture2D {
    fn from(img: RgbaImage) -> Self {
        let width = img.width() as i32;
        let height = img.height() as i32;
        let pixels = img.into_raw();

        Texture2D::new(&pixels, width, height)
    }
}