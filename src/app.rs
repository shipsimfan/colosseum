use crate::{Game, Shader, Window};
use std::time::Instant;

pub struct App<G: Game> {
    game: G,
    window: Window<G::Input>,
    default_shader: Shader,
    fixed_tick_timer: Option<f32>,
}

const DEFAULT_WIDTH: usize = 1920;
const DEFAULT_HEIGHT: usize = 1080;

impl<G: Game> App<G> {
    pub fn new() -> ! {
        // Create window & default shader
        let mut window = Window::new(
            G::INITIAL_TITLE,
            DEFAULT_WIDTH,
            DEFAULT_HEIGHT,
            G::INITIAL_FIXED_UPDATE_DELTA_TIME,
            G::ENABLE_DEBUG_LOGGING,
        );
        window.post_init();

        let default_shader = crate::shader::new_default(&mut window);

        // Create game
        let game = G::new(&mut window);

        // Run
        App {
            game,
            window,
            default_shader,
            fixed_tick_timer: G::INITIAL_FIXED_UPDATE_DELTA_TIME.map(|_| 0.0),
        }
        .run();
    }

    fn run(mut self) -> ! {
        let mut last_tick = Instant::now();

        while self.window.inner().poll_events() {
            // Render
            self.window.inner().begin_render(self.game.clear_color());
            self.default_shader.set_active(&mut self.window);

            self.game.render(&mut self.window);

            self.window.inner().end_render().unwrap();

            // Calculate delta time
            let current_tick = Instant::now();
            let delta_time = (current_tick - last_tick).as_secs_f32();
            last_tick = current_tick;

            // Update
            self.game.update(delta_time, &mut self.window);

            // Fixed Update
            match &mut self.fixed_tick_timer {
                Some(fixed_tick_timer) => {
                    *fixed_tick_timer -= delta_time;
                    if *fixed_tick_timer <= 0.0 {
                        self.game.fixed_update(&mut self.window);
                        self.fixed_tick_timer = self.window.fixed_update_time();
                    }
                }
                None => match self.window.fixed_update_time() {
                    Some(_) => {
                        self.game.fixed_update(&mut self.window);
                        self.fixed_tick_timer = self.window.fixed_update_time();
                    }
                    None => {}
                },
            }
        }

        drop(self);

        std::process::exit(0);
    }
}
