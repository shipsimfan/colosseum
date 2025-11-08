use crate::math::Vector2;

impl<T: std::fmt::Display> std::fmt::Display for Vector2<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        "(".fmt(f)?;
        self.x.fmt(f)?;
        ", ".fmt(f)?;
        self.y.fmt(f)?;
        ")".fmt(f)
    }
}
