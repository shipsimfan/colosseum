use crate::{Archetypes, Resources};

/// A parameter that can be used by a system
pub trait SystemParam {
    /// The type of the system parameter when retrieved
    type Item<'new>;

    /// Retrieves the system parameter from the given resources and archetypes
    fn retrieve<'a>(resources: &'a Resources, archetypes: &'a Archetypes) -> Self::Item<'a>;
}
