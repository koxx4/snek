use piston_window::{G2dTextureContext, Glyphs, TextureSettings};
use piston_window::types::{Color, Scalar, Vec2d};
use rand::{Rng, thread_rng};

pub fn create_font(path: &str, factory: G2dTextureContext) -> Glyphs {

    let glyphs = Glyphs::new(
        path, factory, TextureSettings::new());

    glyphs.unwrap_or_else(|_| panic!("Could not create font from {path}"))
}

pub fn random_solid_color() -> Color {

    let mut rng = thread_rng();

    [rng.gen::<f32>(), rng.gen::<f32>(), rng.gen::<f32>(), 1.0]
}

pub fn random_solid_toned_color() -> Color {

    let mut color = random_solid_color();

    color[0] *= 0.7;
    color[1] *= 0.7;
    color[2] *= 0.7;

    color
}

pub fn random_pos_in_grid(cell_size: Scalar, max_x_cells: usize, max_y_cells: usize) -> Vec2d {

    let mut rng = thread_rng();
    let x_pos = rng.gen_range(0..=max_x_cells) as Scalar * cell_size;
    let y_pos = rng.gen_range(0..=max_y_cells) as Scalar * cell_size;

    [x_pos, y_pos]
}