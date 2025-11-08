use crate::{
    Error, Result,
    graphics::DisplayMode,
    math::{Vector2i, Vector2u},
};
use win32::{AdjustWindowRectEx, FALSE, RECT, try_get_last_error};

impl DisplayMode {
    /// Converts client area position and size to window position and size based on display mode
    pub(crate) fn client_to_window(
        &self,
        size: Vector2u,
        position: Vector2i,
    ) -> Result<(Vector2u, Vector2i)> {
        let (style, ex_style) = self.style();

        let mut rect = RECT {
            left: position.x,
            top: position.y,
            right: (size.x as i32) + position.x,
            bottom: (size.y as i32) + position.y,
        };

        try_get_last_error!(AdjustWindowRectEx(&mut rect, style, FALSE, ex_style))
            .map_err(|os| Error::new_inner("unable to convert window coordinates", os))?;

        Ok((
            Vector2u::new((rect.right - rect.left) as _, (rect.bottom - rect.top) as _),
            Vector2i::new(rect.left, rect.top),
        ))
    }
}
