use crate::{ClearPhase, Command, PhaseName, World};

impl<Phase: PhaseName> Command for ClearPhase<Phase> {
    fn apply(self: Box<Self>, world: &mut World) {
        world.clear_phase(self.phase);
    }
}
