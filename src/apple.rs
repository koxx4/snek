use piston_window::{Context, Flip, G2d, G2dTexture, G2dTextureContext, Image, Texture, TextureSettings};
use piston_window::math::{Matrix2d, Scalar, Vec2d};
use piston_window::rectangle::square;
use crate::game::G2DDrawable;

const RED_APPLE_TEXTURE_PATH: &str = "resources/textures/apple_red.png";
const GOLDEN_APPLE_TEXTURE_PATH: &str = "resources/textures/apple_gold.png";

pub type SnakeBlockCount = usize;

pub trait SnakeCollectibleGrower {
    fn on_collect(&self) -> SnakeBlockCount;
}

pub struct Apple {
    image: Image,
    image_size: Scalar,
    texture: G2dTexture,
    position: Vec2d,
    snake_blocks_grow_count: SnakeBlockCount
}

impl SnakeCollectibleGrower for Apple {
    fn on_collect(&self) -> SnakeBlockCount {
        self.snake_blocks_grow_count
    }
}

impl G2DDrawable for Apple {
    fn draw(&self, context: Context, transform: Matrix2d, graphics: &mut G2d) {
        self.image.draw(&self.texture, &Default::default(), transform, graphics);
    }
}

impl Apple {

    pub fn new_standard_apple(texture_ctx: &mut G2dTextureContext, position: Vec2d, size: Scalar) -> Apple {

        Apple {
            image: Image::new().rect(square(position[0],position[1], size)),
            image_size: size,
            texture: Texture::from_path(
                texture_ctx,
                RED_APPLE_TEXTURE_PATH,
                Flip::None,
                &TextureSettings::new())
                .expect("Could not load a texture for standard apple!"),
            position,
            snake_blocks_grow_count: 1,
        }
    }

    pub fn new_super_apple(texture_ctx: &mut G2dTextureContext, position: Vec2d, size: Scalar) -> Apple {

        Apple {
            image: Image::new().rect(square(position[0],position[1], size)),
            image_size: size,
            texture: Texture::from_path(
                texture_ctx,
                GOLDEN_APPLE_TEXTURE_PATH,
                Flip::None,
                &TextureSettings::new())
                .expect("Could not load a texture for standard apple!"),
            position,
            snake_blocks_grow_count: 1,
        }
    }

    pub fn get_position(&self) -> Vec2d {
        self.position
    }

    pub fn move_to(&mut self, pos: Vec2d) {
        self.position = pos;
        self.image = self.image.rect(square(pos[0], pos[1], self.image_size));
    }
}
