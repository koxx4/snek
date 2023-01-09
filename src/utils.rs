use std::sync::{Arc, Mutex};
use std::sync::mpsc::{channel, Receiver, Sender};
use piston_window::{G2dTextureContext, Glyphs, TextureSettings};
use piston_window::types::{Color, Scalar, Vec2d};
use rand::{Rng, thread_rng};
use timer::{Guard, Timer};

pub struct SnakeMoveTickSystem {
    tick_tx: Arc<Mutex<Sender<bool>>>,
    tick_rx: Receiver<bool>,
    tick_timer: Timer,
    tick_timer_guard: Option<Guard>,
    tick_duration: chrono::Duration,
    stopped: Arc<Mutex<bool>>
}

impl SnakeMoveTickSystem {

    pub fn new(tick_duration: chrono::Duration) -> SnakeMoveTickSystem {

        let (tick_tx, tick_rx) = channel::<bool>();

        SnakeMoveTickSystem {
            tick_tx: Arc::new(Mutex::new(tick_tx)),
            tick_rx,
            tick_timer: Timer::new(),
            tick_timer_guard: None,
            tick_duration,
            stopped: Arc::new(Mutex::new(true))
        }
    }

    pub fn start_ticking(&mut self) {

        *self.stopped.lock().expect("Could not start ticking!") = false;

        let tick_duration= self.tick_duration;
        let stop_ticking_flag_pointer = Arc::clone(&self.stopped);
        let tx_pointer = Arc::clone(&self.tick_tx);

        let guard = self.tick_timer
            .schedule_repeating(tick_duration, move || {

                let is_stopped = *stop_ticking_flag_pointer.lock().unwrap();

                if !is_stopped {
                    tx_pointer.lock().unwrap().send(true).unwrap();
                }
            });

        self.tick_timer_guard = Some(guard);
    }

    pub fn is_tick_available(&self) -> bool {
        self.tick_rx.try_recv().unwrap_or(false)
    }

    pub fn stop_ticking(&mut self) {
        *self.stopped.lock().expect("Could not start ticking!") = true;
    }
}


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

