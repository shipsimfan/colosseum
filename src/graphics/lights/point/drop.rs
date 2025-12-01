use crate::graphics::lights::PointLight;
use std::rc::Rc;

impl Drop for PointLight {
    fn drop(&mut self) {
        // Is this the last reference to the point light, other than from the list?
        if Rc::strong_count(&self.point_light) == 2 {
            // If so, remove it from the list
            let mut point_light_list = self.point_light_list.borrow_mut();
            let mut index = None;
            for (i, point_light) in point_light_list.0.iter().enumerate() {
                if Rc::ptr_eq(point_light, &self.point_light) {
                    index = Some(i);
                }
            }

            if let Some(i) = index {
                point_light_list.0.remove(i);
                point_light_list.1 = true;
            }
        }
    }
}
