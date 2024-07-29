use crate::{logging::LogController, SettingsController};
use graphics::GraphicsState;

mod game_loop;
mod graphics;
mod init;
mod render_context;
mod update_context;

pub use render_context::RenderContext;
pub use update_context::UpdateContext;

/// The state of the whole game engine
pub(crate) struct Colosseum {
    /// The graphics state
    graphics: GraphicsState,

    /// Creates loggers and maintains the logging infrastructure
    log_controller: LogController,

    /// Maintains the settings files
    settings: SettingsController,
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
