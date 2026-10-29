use crate::{Query, QueryData, QueryFilter};

impl<'world, 'state, Components: QueryData, Filter: QueryFilter> Iterator
    for Query<'world, 'state, Components, Filter>
{
    type Item = Components::Item<'world, 'state>;

    fn next(&mut self) -> Option<Self::Item> {
        let world = unsafe { self.world.get() };
        let (archetype, state) = loop {
            // Check if the current archetype is valid
            let (archetype, state) = match self.state.get(self.archetype) {
                Some((archetype, state)) => (archetype, state),
                None => return None,
            };

            // Check if the current index is valid in the archetype
            if self.index < world.archetypes().archetype_len(archetype) {
                break (archetype, state);
            }

            self.archetype += 1;
            self.index = 0;
        };

        // Return the state for the current entity
        let state = unsafe { Components::query(archetype, self.index, self.world, state) };
        self.index += 1;
        Some(state)
    }
}
