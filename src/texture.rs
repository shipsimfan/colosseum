use crate::{SampleType, Window};
use std::{cell::RefCell, path::Path, rc::Rc};

#[derive(Clone)]
pub struct Texture {
    inner: Rc<RefCell<alexandria::Texture2D>>,
}

impl Texture {
    pub fn load<P: AsRef<Path>, I: alexandria::Input>(
        path: P,
        sample_type: SampleType,
        window: &mut Window<I>,
    ) -> Self {
        let image = ginger::open_image(path).unwrap();

        Texture {
            inner: Rc::new(RefCell::new(
                alexandria::Texture2D::new(&image, 0, sample_type, window.inner()).unwrap(),
            )),
        }
    }

    pub(crate) fn set_active(&self) {
        self.inner.borrow_mut().set_active();
    }

    pub(crate) fn clear_active(&self) {
        self.inner.borrow_mut().clear_active();
    }
}
