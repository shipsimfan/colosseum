/// Define a new sytem
#[macro_export]
macro_rules! system {
    (|$($param: ident: $type: ty),*| $($tt: tt)*) => {
        $crate::system_with_extra_data!(|_extra_data: (), $($param: $type),*| $($tt)*)
    };
}

/// Define a new sytem with extra data
#[macro_export]
macro_rules! system_with_extra_data {
    (|$extra_data_name: ident: $extra_data_type: ty, $($param: ident: $type: ty),*| $($tt: tt)*) => {{
        type __Components<'a> = ($(&'a mut [$type],)*);
        const __TYPE_IDS: &[::std::any::TypeId] = &[$(
            ::std::any::TypeId::of::<$type>(),
        )*];
        const __TYPE_COUNT: usize = __TYPE_IDS.len();

        (
            __TYPE_IDS,
            ::std::boxed::Box::new(
                move |archetypes: &mut [$crate::update::ecs::Archetype], indices: &[usize], $extra_data_name: &mut $extra_data_type| {
                    let archetype_count = indices.len() / (__TYPE_COUNT + 1);
                    for i in 0..archetype_count {
                        let archetype_index = indices[i * (__TYPE_COUNT + 1)];
                        let component_indices =
                            &indices[i * (__TYPE_COUNT + 1) + 1..(i + 1) * (__TYPE_COUNT + 1)];

                        let archetype = &mut archetypes[archetype_index];
                        let [$($param,)*] = archetype.get_disjoint_components_mut::<__TYPE_COUNT>(
                            component_indices.try_into().unwrap(),
                        );
                        let component_set = ($($param.get_all_mut::<$type>(),)*);

                        (|($($param,)*): __Components| $($tt)*)(component_set);
                    }
                },
            ),
        )
    }};
}
