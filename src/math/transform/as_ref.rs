use crate::math::Transform;

impl AsMut<Transform> for &mut Transform {
    fn as_mut(&mut self) -> &mut Transform {
        self
    }
}
