use crate::{Entity, World};
use alexandria::Id;

/// A set of components that can be added to an entity in the ECS system
pub trait ComponentList: 'static {
    /// Add the components in this list to `entity`
    fn add_to_entity(self, entity: Id<Entity>, world: &mut World);
}

macro_rules! impl_component_list {
    ( $($params: ident),+$(,)?) => {
        impl<$($params: 'static + Send + Sync + Sized,)+> ComponentList for ($($params,)+) {
            #[allow(non_snake_case)]
            fn add_to_entity(self, entity: Id<Entity>, world: &mut World) {
                let ($($params,)+) = self;
                $(
                    world.add_component(entity, $params);
                )+
            }
        }
    };
}

impl_component_list!(T1);
impl_component_list!(T1, T2);
impl_component_list!(T1, T2, T3);
impl_component_list!(T1, T2, T3, T4);
impl_component_list!(T1, T2, T3, T4, T5);
impl_component_list!(T1, T2, T3, T4, T5, T6);
impl_component_list!(T1, T2, T3, T4, T5, T6, T7);
impl_component_list!(T1, T2, T3, T4, T5, T6, T7, T8);
impl_component_list!(T1, T2, T3, T4, T5, T6, T7, T8, T9);
impl_component_list!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10);
impl_component_list!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11);
impl_component_list!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12);
impl_component_list!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13);
impl_component_list!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14);
impl_component_list!(
    T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15
);
impl_component_list!(
    T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16
);
