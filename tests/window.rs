struct Scene {}

#[test]
fn window() -> Result<(), colosseum::Error> {
    let game = colosseum::Game::new::<Scene>(colosseum::GraphicsSettings::default(), ())?;
    Ok(game.run())
}

impl colosseum::InitialScene for Scene {
    fn new(_: Self::Context, _: &mut colosseum::Window) -> Box<dyn colosseum::Scene> {
        Box::new(Scene {})
    }
}

impl colosseum::Scene for Scene {}
