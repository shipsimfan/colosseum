use crate::{
    info,
    logging::{LogController, Logger},
};
use alexandria::{Instance, Window};

mod event_logger;
mod game_loop;
mod init;
mod render_context;
mod update_context;

pub use render_context::RenderContext;
pub use update_context::UpdateContext;

/// The state of the whole game engine
pub(crate) struct Colosseum {
    /// The graphics instance
    instance: Instance,

    /// The window the game is displayed in
    window: Box<Window>,

    /// The logger for graphics information
    graphics_logger: Logger,

    /// Creates loggers and maintains the logging infrastructure
    log_controller: LogController,
}

impl Colosseum {
    /// Get the [`UpdateContext`] for this state
    pub(crate) fn update_context(&mut self) -> UpdateContext {
        UpdateContext::new(self)
    }

    /// Get the [`RenderContext`] for this state
    pub(crate) fn render_context(&mut self) -> RenderContext {
        RenderContext::new(self)
    }
}

impl Drop for Colosseum {
    fn drop(&mut self) {
        info!(self.graphics_logger, "Shutting down");
    }
}
