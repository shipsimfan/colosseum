use crate::{Commands, PhaseName};

mod command;
mod new;

/// A command to clear all systems in a specific phase
pub struct ClearPhase<Phase: PhaseName> {
    /// The phase to be cleared
    phase: Phase,
}

impl Commands {
    /// Add a command to clear all systems in a specific phase
    pub fn clear_phase<Phase: PhaseName>(&mut self, phase: Phase) {
        self.queue(ClearPhase::new(phase));
    }
}
