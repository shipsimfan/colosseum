use crate::{ClearPhase, PhaseName};

impl<Phase: PhaseName> ClearPhase<Phase> {
    /// Create a new [`ClearPhase`] command
    pub fn new(phase: Phase) -> ClearPhase<Phase> {
        ClearPhase { phase }
    }
}
