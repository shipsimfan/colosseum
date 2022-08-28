use crate::Window;
use alexandria::StateTrackingInput;

pub trait Game {
    type Input: crate::Input = StateTrackingInput;

    const INITIAL_TITLE: &'static str;
    const INITIAL_FIXED_UPDATE_DELTA_TIME: Option<f32> = None;

    fn new(window: &mut Window<Self::Input>) -> Self;

    fn update(&mut self, delta_time: f32, window: &mut Window<Self::Input>);
    #[allow(unused_variables)]
    fn fixed_update(&mut self, window: &mut Window<Self::Input>) {}

    fn render(&mut self, window: &mut Window<Self::Input>);
    fn clear_color(&self) -> [f32; 4];
}
