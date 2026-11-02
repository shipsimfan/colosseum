use crate::{Query, QueryData, QueryFilter};

impl<'world, 'state, Components: QueryData, Filter: QueryFilter>
    Query<'world, 'state, Components, Filter>
{
    /// Returns the number of entities matching the query
    pub fn len(&self) -> usize {
        let archetypes = unsafe { self.world.get() }.archetypes();
        let mut count = 0;
        for archetype in self.state.archetypes() {
            count += archetypes.archetype_len(archetype);
        }
        count
    }
}
