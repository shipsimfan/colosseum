use crate::graphics::context::{LightType, managed_objects::lights::LightList};
use std::{cell::RefCell, rc::Rc};

impl<T: LightType> LightList<T> {
    /// Push a new light onto the list, returning a reference to the shared list
    pub fn push(&mut self, light: Rc<RefCell<T>>) -> Rc<RefCell<(Vec<Rc<RefCell<T>>>, bool)>> {
        let mut shared_list = self.shared_list.borrow_mut();
        shared_list.0.push(light);
        shared_list.1 = true;
        return self.shared_list.clone();
    }
}
