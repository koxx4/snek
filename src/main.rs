use piston_window::*;
use piston_window::types::Color;
use rand::{Rng, thread_rng};

fn main() {

    let mut clear_color: Color = [0.5, 1.0, 0.5, 1.0];
    let mut rect_dimensions: types::Rectangle = [0.0, 0.0, 100.0, 100.0];

    let mut window: PistonWindow = WindowSettings::new("Hello Piston!", (640, 480))
        .exit_on_esc(true)
        .build()
        .unwrap_or_else(|e|  panic!("Failed to build PistonWindow: {}", e));

    while let Some(e) = window.next() {

        e.button(|e| change_background_color(e, &mut clear_color));
        e.button(|e| move_rect(e, &mut rect_dimensions));

        window.draw_2d(&e, |_context, _graphics, _device| {

            clear(clear_color, _graphics);

            rectangle([1.0, 0.0, 0.0, 1.0], // red
                      rect_dimensions, // rectangle
                      _context.transform, _graphics);
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

fn move_rect(event: ButtonArgs, rectangle_dimensions: &mut types::Rectangle) {

    const MOVE_SPEED: f64 = 10.0;

    if let Button::Keyboard(key) = event.button {

        if event.state == ButtonState::Release {
            return;
        }

        match key {
            Key::Up => rectangle_dimensions[1] -= MOVE_SPEED,
            Key::Down => rectangle_dimensions[1] += MOVE_SPEED,
            Key::Right => rectangle_dimensions[0] += MOVE_SPEED,
            Key::Left => rectangle_dimensions[0] -= MOVE_SPEED,
            _ => {}
        }
    }
}