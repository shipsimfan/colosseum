use crate::{MessageThread, input::InputButtonEvent, warning};

impl MessageThread {
    /// Process inputs in the queue
    pub fn process_inputs<Input: crate::input::Input>(&self, input: &mut Input) {
        // Process button events
        for _ in 0..64 {
            let event = match self.button_events.try_recv() {
                Ok(event) => event,
                Err(_) => break,
            };

            let mut id = None;
            for (handle, potiential_id) in &self.handle_to_device_id_map {
                if *handle == event.device() {
                    id = Some(*potiential_id);
                    break;
                }
            }

            let id = match id {
                Some(id) => id,
                None => {
                    warning!(self.logger, "received input event for unregistered device");
                    continue;
                }
            };

            input.button_event(InputButtonEvent::new(id, event.button(), event.pressed()));
        }
    }
}
