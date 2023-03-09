use crate::Window;

pub trait Scene {
    fn update(&mut self, window: &mut Window) -> Option<Box<dyn Scene>>;

    fn render(&mut self, window: &mut Window);
}

pub trait InitialScene: Scene {
    type Context = ();

    fn new(context: Self::Context, window: &mut Window) -> Box<dyn Scene>;
}
