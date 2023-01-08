use piston_window::{Context, G2d, Graphics, ImageSize};
use piston_window::math::Matrix2d;

pub trait G2DDrawable {
    fn draw(&self, context: Context, transform: Matrix2d, graphics: &mut G2d);
}