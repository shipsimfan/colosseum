use crate::{InputEvent, Key, Result, UserEvent, debug, run::Wsi};
use alexandria::{Event, EventKind, math::Vector2u};
use colosseum_core::{ThreadManager, new_error};

impl Wsi {
    /// Pump the event loop, handling events as they come in. This will block until an event is received
    pub fn pump(&mut self, thread_manager: &ThreadManager<UserEvent>) -> Result<bool> {
        let event = self
            .event_pump
            .wait()
            .map_err(|error| new_error!("unable to wait for window event - {}", error))?;
        if !self.handle_event(event)? {
            return Ok(false);
        }

        while thread_manager.is_running()
            && let Some(event) = self
                .event_pump
                .poll()
                .map_err(|error| new_error!("unable to poll window event - {}", error))?
        {
            if !self.handle_event(event)? {
                return Ok(false);
            }
        }

        Ok(true)
    }

    /// Handle a single event
    fn handle_event(&mut self, event: Event<UserEvent>) -> Result<bool> {
        match event.kind {
            EventKind::Quit | EventKind::WindowCloseRequest { .. } => {
                debug!(&self.logger, "Received quit event");
                return Ok(false);
            }

            EventKind::WindowMoved { new_position, .. } => {
                self.shared_window.set_position(new_position);
            }
            EventKind::WindowResized { new_size, .. } => {
                if !self.window.is_minimized() {
                    self.shared_window
                        .set_size(Vector2u::new(new_size.x as _, new_size.y as _));
                }
            }
            EventKind::WindowMinimized { .. } => {
                self.shared_window.set_size(Vector2u::new(0, 0));
            }
            EventKind::WindowRestored { .. } => {
                let new_size = self.window.size();
                self.shared_window.set_size(new_size);
                self.shared_window.set_maximized(false);
            }
            EventKind::WindowEnteredFullscreen { .. } => {
                self.shared_window.set_fullscreen(true);
            }
            EventKind::WindowLeftFullscreen { .. } => {
                self.shared_window.set_fullscreen(false);
            }
            EventKind::WindowMaximized { .. } => {
                self.shared_window.set_maximized(true);
            }
            EventKind::WindowGainedFocus { .. } => {
                self.shared_window.set_focused(true);
            }
            EventKind::WindowLostFocus { .. } => {
                self.shared_window.set_focused(false);
            }

            EventKind::KeyDown {
                key_code, key_mod, ..
            } => {
                self.input_sender
                    .send(InputEvent::KeyDown { key: key_code })
                    .ok();

                if key_code == Key::F4 && key_mod.alt() {
                    return Ok(false);
                }
            }
            EventKind::KeyUp { key_code, .. } => {
                self.input_sender
                    .send(InputEvent::KeyUp { key: key_code })
                    .ok();
            }

            EventKind::MouseMove { position, .. } => {
                self.shared_window.set_mouse_position(position);
            }
            EventKind::MouseButtonDown { button, .. } => {
                self.input_sender
                    .send(InputEvent::MouseButtonDown { button })
                    .ok();
            }
            EventKind::MouseButtonUp { button, .. } => {
                self.input_sender
                    .send(InputEvent::MouseButtonUp { button })
                    .ok();
            }
            EventKind::MouseWheel { delta, .. } => {
                self.shared_window.add_mouse_wheel(delta);
            }

            EventKind::User(UserEvent::SetFullscreen) => {
                self.window.set_fullscreen(true).map_err(|error| {
                    new_error!("unable to set window to fullscreen - {}", error)
                })?;
            }
            EventKind::User(UserEvent::UnsetFullscreen) => {
                self.window.set_fullscreen(false).map_err(|error| {
                    new_error!("unable to unset window from fullscreen - {}", error)
                })?;
            }

            _ => {}
        }

        Ok(true)
    }
}
