use crate::update::ecs::archetype::Components;

impl std::fmt::Debug for Components {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.type_name.fmt(f)
    }
}
