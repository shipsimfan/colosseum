pub trait Scene {}

pub trait InitialScene: Scene {
    type Context = ();

    fn new(context: Self::Context) -> Box<dyn Scene>;
}
