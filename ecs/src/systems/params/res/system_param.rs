use crate::{Archetypes, Res, Resources, SystemParam};

impl<'res, T: 'static> SystemParam for Res<'res, T> {
    type Item<'new> = Res<'new, T>;

    fn retrieve<'a>(resources: &'a Resources, _: &'a Archetypes) -> Self::Item<'a> {
        resources.get::<T>().unwrap()
    }
}

impl<'res, T: 'static> SystemParam for Option<Res<'res, T>> {
    type Item<'new> = Option<Res<'new, T>>;

    fn retrieve<'a>(resources: &'a Resources, _: &'a Archetypes) -> Self::Item<'a> {
        resources.get::<T>()
    }
}
