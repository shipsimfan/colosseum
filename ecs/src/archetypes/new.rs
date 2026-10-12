use crate::Archetypes;
use alexandria::SlotMap;

impl Archetypes {
    /// Create a new, empty set of [`Archetypes`]
    pub(crate) fn new() -> Archetypes {
        Archetypes {
            entities: SlotMap::new(),
        }
    }
}
