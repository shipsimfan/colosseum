/// Begins the game engine with the provided types
#[macro_export]
macro_rules! run {
    ($game: ty) => {
        fn main() {
            $crate::run::<$game>();
        }
    };
}
