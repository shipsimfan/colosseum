/// Begins the game engine with the provided types
#[macro_export]
macro_rules! run {
    ($title: expr, $options: ty, $first_scene: ty) => {
        fn main() {
            $crate::run::<$first_scene, $options>($title);
        }
    };
}
