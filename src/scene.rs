use crate::Window;

pub trait Scene {}

pub trait InitialScene: Scene {
    type Context = ();

    fn new(context: Self::Context, window: &mut Window) -> Box<dyn Scene>;
}
