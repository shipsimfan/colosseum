/// Begins the game engine with the specified game type
#[macro_export]
macro_rules! run {
    ($game: ty, $initial_scene: expr) => {
        fn main() {
            $crate::run::<$game, _>(
                $initial_scene,
                option_env!("COLOSSEUM_GAME_BRANCH"),
                option_env!("COLOSSEUM_GAME_COMMIT"),
                option_env!("COLOSSEUM_GAME_BUILD_TIME"),
            );
        }
    };
}
