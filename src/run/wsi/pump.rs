use crate::{Error, Result, debug, run::Wsi};
use alexandria::{Event, EventKind};

impl Wsi {
    /// Pump the event loop, handling events as they come in. This will block until an event is received
    pub fn pump(&mut self) -> Result<bool> {
        let event = self
            .event_pump
            .wait()
            .map_err(|error| Error::new_inner(error))?;
        if !self.handle_event(event)? {
            return Ok(false);
        }

        while let Some(event) = self
            .event_pump
            .poll()
            .map_err(|error| Error::new_inner(error))?
        {
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
                Ok(false)
            }
            _ => Ok(true),
        }
    }
}
