use std::any::Any;

/// A trait for dynamically checking equality between objects of the same type
pub trait DynEq: 'static + Any {
    /// Checks if this object is equal to another object of the same type
    fn dyn_eq(&self, other: &dyn DynEq) -> bool;
}

impl<T: 'static + Eq> DynEq for T {
    fn dyn_eq(&self, other: &dyn DynEq) -> bool {
        if let Some(other) = (other as &dyn Any).downcast_ref::<T>() {
            self == other
        } else {
            false
        }
    }
}
