use crate::{InputEvent, Window, update::Input};
use colosseum_ecs::{Res, ResMut};

impl Input {
    /// Process any input updates from the window
    pub(in crate::update) fn system(mut self: ResMut<Self>, window: Res<Window>) {
        self.reset();
        while let Some(event) = window.next_input() {
            match event {
                InputEvent::KeyDown { key } => self.set_key_down(key),
                InputEvent::KeyUp { key } => self.set_key_up(key),
                InputEvent::MouseButtonDown { button } => self.set_mouse_button_down(button),
                InputEvent::MouseButtonUp { button } => self.set_mouse_button_up(button),
            }
        }

        self.set_mouse_position(window.mouse_position());
        self.set_mouse_wheel(window.mouse_wheel());
    }
}
