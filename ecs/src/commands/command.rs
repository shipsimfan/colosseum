use crate::World;

/// A command that can be applied to the ECS world
pub trait Command: 'static {
    /// Apply the command to the ECS world
    fn apply(self: Box<Self>, world: &mut World);
}
