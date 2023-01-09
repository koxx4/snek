mod snake;
mod apple;
mod game;
mod utils;
mod controls;

use std::ops::Add;
use std::sync::mpsc::channel;
use timer::Timer;
use piston_window::*;
use piston_window::color::{BLACK, RED};
use piston_window::types::{Color, Scalar};
use crate::apple::{Apple, SnakeBlockCount, SnakeCollectibleGrower};
use crate::game::G2DDrawable;
use crate::snake::Snake;

const SNAKE_BLOCK_SIZE: Scalar = 40.0;
const SNAKE_STARTING_BLOCK_COUNT: SnakeBlockCount = 3;
const ARENA_CELLS_WIDTH: usize = 30;
const ARENA_CELLS_HEIGHT: usize = 20;
const ARENA_WIDTH: Scalar = SNAKE_BLOCK_SIZE * ARENA_CELLS_WIDTH as Scalar;
const ARENA_HEIGHT: Scalar = SNAKE_BLOCK_SIZE * ARENA_CELLS_HEIGHT as Scalar;
const ARENA_CENTER_X: Scalar = ARENA_WIDTH * 0.5;
const ARENA_CENTER_Y: Scalar = ARENA_HEIGHT * 0.5;

fn main() {

    let mut clear_color: Color = [0.5, 1.0, 0.5, 1.0];

    let mut window: PistonWindow = WindowSettings::new("Snek 🐍", (ARENA_WIDTH, ARENA_HEIGHT))
        .exit_on_esc(true)
        .build()
        .unwrap_or_else(|e|  panic!("Failed to build PistonWindow: {}", e));

    window.set_lazy(false);
    window.set_max_fps(60);

    let mut texture_ctx = window.create_texture_context();

    let mut default_text_font_glyphs = window.load_font(
        "resources/fonts/SuperMario256.ttf").unwrap();
    default_text_font_glyphs.preload_printable_ascii(30).unwrap();

    let mut snake = Snake::new(
        SNAKE_STARTING_BLOCK_COUNT, SNAKE_BLOCK_SIZE, 10.0);

    let mut is_snake_dead = false;
    let (snake_tick_move_tx, snake_move_tick_rx) = channel::<bool>();

    let tick_timer = Timer::new();
    let guard = tick_timer.schedule_repeating(
        chrono::Duration::milliseconds(150), move || snake_tick_move_tx.send(true).unwrap());

    let apple_pos = utils::random_pos_in_grid(
        SNAKE_BLOCK_SIZE, ARENA_CELLS_WIDTH, ARENA_CELLS_HEIGHT);

    let mut apple = Apple::new_standard_apple(
        &mut texture_ctx,
        apple_pos,
        SNAKE_BLOCK_SIZE);

    while let Some(e) = window.next() {

        e.button(|e| controls::handle_snake_controls(e, &mut snake));

        window.draw_2d(&e, |_context, _graphics, _device| {

            let should_snake_move = snake_move_tick_rx.try_recv().unwrap_or(false);

            if should_snake_move && !is_snake_dead {
                snake.move_in_current_direction();
            }

            if snake.is_head_at_position(&apple.get_position()) {
                snake.grow(apple.on_collect());
                let apple_pos = utils::random_pos_in_grid(40.0, 25, 15);
                apple.move_to(apple_pos);
                clear_color = utils::random_solid_toned_color();
            }

            if snake.is_head_at_any_body_block() {
                is_snake_dead = true;
                clear_color = BLACK;
            }

            clear(clear_color, _graphics);

            {
                let drawables: Vec<&dyn G2DDrawable> = vec![&snake, &apple];

                batch_draw(&drawables, _context, _graphics);
            }

            let score_text = "Current score: ".to_string().add(&snake.get_length_as_str());
            Text::new_color(BLACK, 20)
                .draw(&score_text,
                      &mut default_text_font_glyphs,
                      &_context.draw_state,
                      _context.trans(50.0, 50.0).transform,
                      _graphics)
                .unwrap();

            if is_snake_dead {
                let dead_text = "You're dead! Your score is ".to_string().add(&snake.get_length_as_str());
                Text::new_color(RED, 30)
                    .draw(&dead_text,
                          &mut default_text_font_glyphs,
                          &_context.draw_state,
                          _context.trans(ARENA_CENTER_X - 200.0, ARENA_CENTER_Y).transform,
                          _graphics)
                    .unwrap();
            }

            // Update glyphs before rendering.
            default_text_font_glyphs.factory.encoder.flush(_device);
        });
    }
}

fn batch_draw<>(
    drawable_list: &[&dyn G2DDrawable],
    context: Context,
    graphics: &mut G2d
) {

    for drawable in drawable_list {
        drawable.draw(context, context.transform, graphics)
    }
}