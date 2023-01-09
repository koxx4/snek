use piston_window::{Button, ButtonArgs, ButtonState, Key};
use crate::snake::Snake;

pub(crate) fn handle_snake_controls(event: ButtonArgs, snake: &mut Snake) {

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