use crate::util::Handle;

impl<T: Sized> PartialEq for Handle<T> {
    fn eq(&self, other: &Self) -> bool {
        self.index == other.index && self.generation == other.generation
    }
}

impl<T: Sized> Eq for Handle<T> {}
