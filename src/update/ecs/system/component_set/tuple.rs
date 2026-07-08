use crate::update::ecs::{Archetype, system::ComponentSet};
use std::any::TypeId;

impl<'a, T1: 'static> ComponentSet<'a> for (&'a [T1],) {
    const COUNT: usize = 1;

    const TYPE_IDS: &'static [TypeId] = &[TypeId::of::<T1>()];

    #[allow(private_interfaces)]
    fn from_archetype(archetype: &'a mut Archetype, indices: &[usize]) -> Self {
        (archetype.get_all_at(indices[0]),)
    }
}

impl<'a, T1: 'static> ComponentSet<'a> for (&'a mut [T1],) {
    const COUNT: usize = 1;

    const TYPE_IDS: &'static [TypeId] = &[TypeId::of::<T1>()];

    #[allow(private_interfaces)]
    fn from_archetype(archetype: &'a mut Archetype, indices: &[usize]) -> Self {
        (archetype.get_all_at_mut(indices[0]),)
    }
}

impl<'a, T1: 'static, T2: 'static> ComponentSet<'a> for (&'a [T1], &'a [T2]) {
    const COUNT: usize = 2;

    const TYPE_IDS: &'static [TypeId] = &[TypeId::of::<T1>(), TypeId::of::<T2>()];

    #[allow(private_interfaces)]
    fn from_archetype(archetype: &'a mut Archetype, indices: &[usize]) -> Self {
        (
            archetype.get_all_at(indices[0]),
            archetype.get_all_at(indices[1]),
        )
    }
}

impl<'a, T1: 'static, T2: 'static> ComponentSet<'a> for (&'a mut [T1], &'a [T2]) {
    const COUNT: usize = 2;

    const TYPE_IDS: &'static [TypeId] = &[TypeId::of::<T1>(), TypeId::of::<T2>()];

    #[allow(private_interfaces)]
    fn from_archetype(archetype: &'a mut Archetype, indices: &[usize]) -> Self {
        let [t1, t2] = archetype.get_disjoint_components_mut([indices[0], indices[1]]);

        (t1.get_all_mut(), t2.get_all())
    }
}

impl<'a, T1: 'static, T2: 'static> ComponentSet<'a> for (&'a [T1], &'a mut [T2]) {
    const COUNT: usize = 2;

    const TYPE_IDS: &'static [TypeId] = &[TypeId::of::<T1>(), TypeId::of::<T2>()];

    #[allow(private_interfaces)]
    fn from_archetype(archetype: &'a mut Archetype, indices: &[usize]) -> Self {
        let [t1, t2] = archetype.get_disjoint_components_mut([indices[0], indices[1]]);

        (t1.get_all(), t2.get_all_mut())
    }
}

impl<'a, T1: 'static, T2: 'static> ComponentSet<'a> for (&'a mut [T1], &'a mut [T2]) {
    const COUNT: usize = 2;

    const TYPE_IDS: &'static [TypeId] = &[TypeId::of::<T1>(), TypeId::of::<T2>()];

    #[allow(private_interfaces)]
    fn from_archetype(archetype: &'a mut Archetype, indices: &[usize]) -> Self {
        let [t1, t2] = archetype.get_disjoint_components_mut([indices[0], indices[1]]);

        (t1.get_all_mut(), t2.get_all_mut())
    }
}
