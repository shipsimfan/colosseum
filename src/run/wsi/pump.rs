use crate::{Error, Result, debug, run::Wsi};
use alexandria::{Event, EventKind, math::Vector2u};

impl Wsi {
    /// Pump the event loop, handling events as they come in. This will block until an event is received
    pub fn pump(&mut self) -> Result<bool> {
        let event = self.event_pump.wait().map_err(Error::new_inner)?;
        if !self.handle_event(event)? {
            return Ok(false);
        }

        while let Some(event) = self.event_pump.poll().map_err(Error::new_inner)? {
            if !self.handle_event(event)? {
                return Ok(false);
            }
        }

        Ok(true)
    }

    /// Handle a single event
    fn handle_event(&mut self, event: Event<()>) -> Result<bool> {
        match event.kind {
            EventKind::Quit | EventKind::WindowCloseRequest { .. } => {
                debug!(&self.logger, "Received quit event");
                return Ok(false);
            }
            EventKind::WindowResized { id: _, new_size } => {
                self.shared_window
                    .set_size(Vector2u::new(new_size.x as _, new_size.y as _));
            }
            _ => {}
        }

        Ok(true)
    }
}
