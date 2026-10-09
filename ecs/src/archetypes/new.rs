use crate::Archetypes;
use alexandria::SlotMap;

impl Archetypes {
    /// Create a new, empty set of [`Archetypes`]
    pub fn new() -> Archetypes {
        Archetypes {
            entities: SlotMap::new(),
        }
    }
}
