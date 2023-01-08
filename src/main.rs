mod snake;
mod apple;
mod game;

use std::sync::mpsc::channel;
use timer::Timer;
use piston_window::*;
use piston_window::math::{Scalar, Vec2d};
use piston_window::types::Color;
use rand::{Rng, thread_rng};
use crate::apple::{Apple, SnakeCollectibleGrower};
use crate::game::G2DDrawable;
use crate::snake::Snake;

fn main() {

    let mut clear_color: Color = [0.5, 1.0, 0.5, 1.0];

    let (sender, receiver) = channel();

    let snake_block_size: Scalar = 40.0;
    let mut snake = Snake::new(3, snake_block_size, 10.0);

    let tick_timer = Timer::new();
    let guard = tick_timer.schedule_repeating(
        chrono::Duration::milliseconds(300), move || sender.send(true).unwrap());

    let mut window: PistonWindow = WindowSettings::new("Snek", (30.0 * snake_block_size, 20.0 * snake_block_size))
        .exit_on_esc(true)
        .build()
        .unwrap_or_else(|e|  panic!("Failed to build PistonWindow: {}", e));

    let mut texture_ctx = window.create_texture_context();
    let apple_pos = [10.0 * snake_block_size, 10.0 * snake_block_size];

    let mut apple = Apple::new_standard_apple(
        &mut texture_ctx,
        apple_pos,
        snake_block_size);

    while let Some(e) = window.next() {

        e.button(|e| move_snake(e, &mut snake));

        window.draw_2d(&e, |_context, _graphics, _device| {

            let should_snake_move = receiver.try_recv().unwrap_or(false);

            if should_snake_move {
                snake.move_in_current_direction();
            }

            if snake.is_head_at_position(&apple.get_position()) {
                snake.grow(apple.on_collect());
                let apple_pos = random_pos_in_grid(40.0, 25, 15);
                apple.move_to(apple_pos);
                clear_color = random_solid_color();
            }

            clear(clear_color, _graphics);
            snake.draw(_context.transform, _graphics);
            apple.draw(_context, _context.transform, _graphics);
        });
    }
}

fn random_solid_color() -> Color {

    let mut rng = thread_rng();
    let mut color: Color = [rng.gen::<f32>(), rng.gen::<f32>(), rng.gen::<f32>(), 1.0];

    color
}

fn random_pos_in_grid(cell_size: Scalar, max_x_cells: usize, max_y_cells: usize) -> Vec2d {

    let mut rng = thread_rng();
    let x_pos = rng.gen_range(0..=max_x_cells) as Scalar * cell_size;
    let y_pos = rng.gen_range(0..=max_y_cells) as Scalar * cell_size;

    [x_pos, y_pos]
}

fn move_snake(event: ButtonArgs, snake: &mut Snake) {

    if let Button::Keyboard(key) = event.button {

        if event.state == ButtonState::Release {
            return;
        }

        match key {
            Key::Up => snake.change_dir_to_up(),
            Key::Down => snake.change_dir_to_down(),
            Key::Right => snake.change_dir_to_right(),
            Key::Left => snake.change_dir_to_left(),
            _ => {}
        }
    }
}