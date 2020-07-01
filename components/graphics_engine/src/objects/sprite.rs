use crate::Vertices;
use crate::gl_wrap::Texture2D;
use crate::glm::Vec4;

bitflags! {
    pub struct FlipDirection: u8 {
        const NONE   = 0 << 0;
        const FLIP_X = 0 << 1;
        const FLIP_Y = 0 << 2;

        const BOTH = Self::FLIP_X.bits | Self::FLIP_Y.bits;
    }
}


pub struct Sprite {
    width: f32,
    height: f32,

    colour: glm::Vec4,
    model_matrix: glm::Mat4,
    texture: Texture2D,

    tile_offset: glm::Vec2,
    flip_direction: FlipDirection,

    depth: f32
}

impl Sprite {
    pub fn new(tex: Texture2D) -> Sprite {
        Sprite {
            width: tex.width() as f32,
            height: tex.height() as f32,

            colour: crate::colour::WHITE.into(),
            model_matrix: glm::one(),
            texture: tex,

            tile_offset: glm::zero(),
            flip_direction: FlipDirection::NONE,

            depth: 0.0
        }
    }

    pub fn width(&self) -> f32 {
        self.width
    }
    pub fn height(&self) -> f32 {
        self.height
    }

    pub fn colour(&self) -> glm::Vec4 {
        self.colour.clone()
    }
    pub fn model_matrix(&self) -> &glm::Mat4 {
        &self.model_matrix
    }
}

impl Vertices for Sprite {
    fn vertices(&self) -> Vec<f32> {
        let flip_x = self.flip_direction.contains(FlipDirection::FLIP_X) as i32 as f32;
        let flip_y = self.flip_direction.contains(FlipDirection::FLIP_Y) as i32 as f32;

        vec![
            // Sprite
            // X          Y             Z              TX   TY      R               G               B               A                  Flipped X    Flipped Y     Texture Width                Texture Height                      Texture Offset X,    Texture Offset Y
            self.width,   self.height,  self.depth,    1.0, 0.0,    self.colour[0], self.colour[1], self.colour[2], self.colour[3],    flip_x,      flip_y,       self.texture.width() as f32, self.texture.height() as f32,       self.tile_offset[0], self.tile_offset[1], // top right
            self.width,           0.0,  self.depth,    1.0, 1.0,    self.colour[0], self.colour[1], self.colour[2], self.colour[3],    flip_x,      flip_y,       self.texture.width() as f32, self.texture.height() as f32,       self.tile_offset[0], self.tile_offset[1], // bottom right
            0.0,                  0.0,  self.depth,    0.0, 1.0,    self.colour[0], self.colour[1], self.colour[2], self.colour[3],    flip_x,      flip_y,       self.texture.width() as f32, self.texture.height() as f32,       self.tile_offset[0], self.tile_offset[1], // bottom left
            0.0,          self.height,  self.depth,    0.0, 0.0,    self.colour[0], self.colour[1], self.colour[2], self.colour[3],    flip_x,      flip_y,       self.texture.width() as f32, self.texture.height() as f32,       self.tile_offset[0], self.tile_offset[1], // top left
        ]
    }

    fn indices(&self) -> Vec<i32> {
        vec![
            0, 1, 3,
            1, 2, 3
        ]
    }
}
