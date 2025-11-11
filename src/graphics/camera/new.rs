use crate::{
    Result,
    graphics::{Camera, CameraInner, CameraProjection},
    math::Vector2u,
};
use std::{cell::RefCell, rc::Rc};
use win32::d3d11::ID3D11Device;

impl Camera {
    /// Create a new [`Camera`]
    pub(in crate::graphics) fn new(
        camera_list: Rc<RefCell<Vec<Rc<RefCell<CameraInner>>>>>,
        projection: CameraProjection,
        screen_size: Vector2u,
        device: &ID3D11Device,
    ) -> Result<Self> {
        let camera = Rc::new(RefCell::new(CameraInner::new(
            projection,
            screen_size,
            device,
        )?));

        camera_list.borrow_mut().push(camera.clone());

        Ok(Camera {
            camera_list,
            camera,
        })
    }
}
