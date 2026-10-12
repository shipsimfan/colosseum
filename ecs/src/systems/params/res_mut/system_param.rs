use crate::{Archetypes, ResMut, Resources, SystemParam};

impl<'res, T: 'static> SystemParam for ResMut<'res, T> {
    type Item<'new> = ResMut<'new, T>;

    fn retrieve<'a>(resources: &'a Resources, _: &'a Archetypes) -> Self::Item<'a> {
        resources.get_mut::<T>().unwrap()
    }
}

impl<'res, T: 'static> SystemParam for Option<ResMut<'res, T>> {
    type Item<'new> = Option<ResMut<'new, T>>;

    fn retrieve<'a>(resources: &'a Resources, _: &'a Archetypes) -> Self::Item<'a> {
        resources.get_mut::<T>()
    }
}
