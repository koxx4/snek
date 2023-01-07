mod snake;
mod apple;

use std::path::Path;
use piston_window::*;
use piston_window::math::Scalar;
use piston_window::rectangle::square;
use piston_window::types::Color;
use rand::{Rng, thread_rng};
use crate::snake::Snake;

fn main() {

    let mut clear_color: Color = [0.5, 1.0, 0.5, 1.0];

    let snake_block_size: Scalar = 40.0;
    let mut snake: Snake = Snake::new(8, snake_block_size, 10.0);

    let mut window: PistonWindow = WindowSettings::new("Snek", (30.0 * snake_block_size, 20.0 * snake_block_size))
        .exit_on_esc(true)
        .build()
        .unwrap_or_else(|e|  panic!("Failed to build PistonWindow: {}", e));

    while let Some(e) = window.next() {

        e.button(|e| change_background_color(e, &mut clear_color));
        e.button(|e| move_snake(e, &mut snake));

        window.draw_2d(&e, |_context, _graphics, _device| {

            clear(clear_color, _graphics);
            snake.draw(_context.transform, _graphics);
        });
    }
}

fn change_background_color(event: ButtonArgs, bkg_color: &mut Color) {

    if event.state == ButtonState::Release {
        return;
    }

    if let Button::Keyboard(key) = event.button {

        if key == Key::Up {

            let mut rng = thread_rng();

            bkg_color[0] = rng.gen::<f32>();
            bkg_color[1] = rng.gen::<f32>();
            bkg_color[2] = rng.gen::<f32>();
        }
    }
}

fn move_snake(event: ButtonArgs, snake: &mut Snake) {

    if let Button::Keyboard(key) = event.button {

        if event.state == ButtonState::Release {
            return;
        }

        match key {
            Key::Up => snake.move_up(),
            Key::Down => snake.move_down(),
            Key::Right => snake.move_right(),
            Key::Left => snake.move_left(),
            _ => {}
        }
    }
}