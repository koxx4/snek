mod snake;
mod apple;
mod game;

use std::ops::Add;
use std::sync::mpsc::channel;
use timer::Timer;
use piston_window::*;
use piston_window::color::{BLACK, RED};
use piston_window::types::{Color, Scalar, Vec2d};
use rand::{Rng, thread_rng};
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

    let (snake_tick_move_tx, snake_move_tick_rx) = channel::<bool>();
    //let (snake_death_tx, snake_death_rx) = channel::<bool>();

    let mut snake = Snake::new(
        SNAKE_STARTING_BLOCK_COUNT, SNAKE_BLOCK_SIZE, 10.0);

    let mut is_snake_dead = false;

    let tick_timer = Timer::new();
    let guard = tick_timer.schedule_repeating(
        chrono::Duration::milliseconds(150), move || snake_tick_move_tx.send(true).unwrap());

    let mut window: PistonWindow = WindowSettings::new("Snek 🐍", (ARENA_WIDTH, ARENA_HEIGHT))
        .exit_on_esc(true)
        .build()
        .unwrap_or_else(|e|  panic!("Failed to build PistonWindow: {}", e));

    let mut texture_ctx = window.create_texture_context();

    let apple_pos = random_pos_in_grid(
        SNAKE_BLOCK_SIZE, ARENA_CELLS_WIDTH, ARENA_CELLS_HEIGHT);

    let mut apple = Apple::new_standard_apple(
        &mut texture_ctx,
        apple_pos,
        SNAKE_BLOCK_SIZE);

    let mut default_text_font_glyphs = window.load_font(
        "resources/fonts/SuperMario256.ttf").unwrap();
    default_text_font_glyphs.preload_printable_ascii(30).unwrap();

    window.set_lazy(false);
    window.set_max_fps(60);

    while let Some(e) = window.next() {

        e.button(|e| move_snake(e, &mut snake));

        window.draw_2d(&e, |_context, _graphics, _device| {

            let should_snake_move = snake_move_tick_rx.try_recv().unwrap_or(false);

            if should_snake_move && !is_snake_dead {
                snake.move_in_current_direction();
            }

            if snake.is_head_at_position(&apple.get_position()) {
                snake.grow(apple.on_collect());
                let apple_pos = random_pos_in_grid(40.0, 25, 15);
                apple.move_to(apple_pos);
                clear_color = random_solid_color();
            }

            if snake.is_head_at_any_body_block() {
                is_snake_dead = true;
                clear_color = BLACK;
            }

            clear(clear_color, _graphics);
            snake.draw(_context.transform, _graphics);
            apple.draw(_context, _context.transform, _graphics);

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

fn create_font(path: &str, factory: G2dTextureContext) -> Glyphs {

    let glyphs = Glyphs::new(
        path, factory, TextureSettings::new());

    glyphs.unwrap_or_else(|_| panic!("Could not create font from {path}"))
}

fn random_solid_color() -> Color {

    let mut rng = thread_rng();

    [rng.gen::<f32>(), rng.gen::<f32>(), rng.gen::<f32>(), 1.0]
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