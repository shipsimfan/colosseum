/// A type of light
pub(in crate::graphics::managed_objects::lights) trait LightType {
    /// The representation of the light on the GPU
    type GPU: Clone + Copy;

    /// Get the GPU representation of this light
    fn to_gpu(&self) -> Self::GPU;

    /// Update the light, returning if the properties have changed
    fn update(&mut self) -> bool;
}
