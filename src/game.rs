use crate::Window;
use alexandria::StateTrackingInput;

pub trait Game {
    const INITIAL_TITLE: &'static str;
    type Input: crate::Input = StateTrackingInput;

    fn new(window: &mut Window<Self::Input>) -> Self;

    fn update(&mut self, delta_time: f32, window: &mut Window<Self::Input>);

    fn render(&mut self, window: &mut Window<Self::Input>);
    fn clear_color(&self) -> [f32; 4];
}
