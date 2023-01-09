mod snake;
mod apple;
mod game;
mod utils;
mod controls;

use piston_window::*;
use piston_window::color::{BLACK, RED};
use piston_window::types::{Color, Scalar};
use crate::apple::{Apple, SnakeBlockCount, SnakeCollectibleGrower};
use crate::game::G2DDrawable;
use crate::snake::Snake;
use crate::utils::SnakeMoveTickSystem;

const GAME_WINDOW_TITLE: &str = "Snek 🐍";

const SNAKE_BLOCK_SIZE: Scalar = 40.0;
const SNAKE_STARTING_BLOCK_COUNT: SnakeBlockCount = 3;
const ARENA_CELLS_WIDTH: usize = 30;
const ARENA_CELLS_HEIGHT: usize = 20;
const ARENA_WIDTH: Scalar = SNAKE_BLOCK_SIZE * ARENA_CELLS_WIDTH as Scalar;
const ARENA_HEIGHT: Scalar = SNAKE_BLOCK_SIZE * ARENA_CELLS_HEIGHT as Scalar;
const ARENA_CENTER_X: Scalar = ARENA_WIDTH * 0.5;
const ARENA_CENTER_Y: Scalar = ARENA_HEIGHT * 0.5;
static ARENA_BOUNDS: [Scalar; 4] = [0.0, ARENA_WIDTH, 0.0, ARENA_HEIGHT];

fn create_and_init_game_window_with_texture_ctx() -> (PistonWindow, G2dTextureContext) {

    let mut window: PistonWindow = WindowSettings::new(
        GAME_WINDOW_TITLE, (ARENA_WIDTH, ARENA_HEIGHT))
        .exit_on_esc(true)
        .resizable(false)
        .build()
        .unwrap_or_else(|e|  panic!("Failed to build PistonWindow: {}", e));

    window.set_lazy(false);
    window.set_max_fps(60);

    let texture_ctx = window.create_texture_context();

    (window, texture_ctx)
}

fn create_and_preload_super_mario_font(texture_ctx: G2dTextureContext) -> Glyphs {

    let mut glyphs = utils::create_font("resources/fonts/SuperMario256.ttf", texture_ctx);

    glyphs.preload_printable_ascii(20)
        .unwrap_or_else(|e| eprintln!("Could not preload mario font {e}"));

    glyphs
}

fn main() {

    let mut clear_color: Color = [0.5, 1.0, 0.5, 1.0];

    let (mut window, mut texture_ctx) =
        create_and_init_game_window_with_texture_ctx();

    let mut default_text_font_glyphs =
        create_and_preload_super_mario_font(window.create_texture_context());

    let mut snake = Snake::new(
        SNAKE_STARTING_BLOCK_COUNT, SNAKE_BLOCK_SIZE, 10.0);

    let apple_initial_position = utils::random_pos_in_grid(
        SNAKE_BLOCK_SIZE, ARENA_CELLS_WIDTH, ARENA_CELLS_HEIGHT);

    let mut apple = Apple::new_standard_apple(
        &mut texture_ctx,
        apple_initial_position,
        SNAKE_BLOCK_SIZE);

    let mut snake_move_tick_system =
        SnakeMoveTickSystem::new(chrono::Duration::milliseconds(300));

    snake_move_tick_system.start_ticking();

    while let Some(e) = window.next() {

        e.button(|e| controls::handle_snake_controls(e, &mut snake));

        window.draw_2d(&e, |_context, _graphics, _device| {

            clear(clear_color, _graphics);

            handle_game_logic(&mut clear_color, &mut snake, &mut apple, &mut snake_move_tick_system);

            let drawables: Vec<&dyn G2DDrawable> = vec![&snake, &apple];
            batch_draw(&drawables, _context, _graphics);

            let user_score = snake.get_length_as_str();

            draw_current_score_text(&mut default_text_font_glyphs, &user_score, &_context, _graphics);

            if snake.is_dead() {
                draw_death_text(&mut default_text_font_glyphs, &user_score, &_context, _graphics);
            }

            // Update glyphs before rendering.
            default_text_font_glyphs.factory.encoder.flush(_device);
        });
    }
}

fn handle_game_logic(
    clear_color: &mut Color,
    snake: &mut Snake,
    apple: &mut Apple,
    snake_move_tick_system: &mut SnakeMoveTickSystem) {

    let should_snake_move = snake_move_tick_system.is_tick_available();

    if !should_snake_move {
        return;
    }

    snake.move_in_current_direction();

    if snake.is_head_at_position(&apple.get_position()) {
        snake.grow(apple.on_collect());
        let apple_pos = utils::random_pos_in_grid(40.0, 25, 15);
        apple.move_to(apple_pos);
        *clear_color = utils::random_solid_toned_color();
    }

    if snake.is_head_at_any_body_block() || !snake.is_head_in_bounds(&ARENA_BOUNDS) {
        snake.make_dead();
        snake_move_tick_system.stop_ticking();
        *clear_color = BLACK;
    }
}

fn draw_current_score_text(default_text_font_glyphs: &mut Glyphs, score: &str, _context: &Context, _graphics: &mut G2d) {

    let score_text = format!("Current score {score}");

    Text::new_color(BLACK, 20)
        .draw(&score_text,
              default_text_font_glyphs,
              &_context.draw_state,
              _context.trans(50.0, 50.0).transform,
              _graphics)
        .unwrap();
}

fn draw_death_text(
    default_text_font_glyphs: &mut Glyphs,
    score: &str,
    _context: &Context,
    _graphics: &mut G2d) {

    let dead_text = format!("You're dead! Your score is {score}");

    Text::new_color(RED, 30)
        .draw(&dead_text,
              default_text_font_glyphs,
              &_context.draw_state,
              _context.trans(ARENA_CENTER_X - 200.0, ARENA_CENTER_Y).transform,
              _graphics)
        .unwrap();
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