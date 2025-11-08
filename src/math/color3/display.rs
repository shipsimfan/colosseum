use crate::math::Color3;

impl<T: std::fmt::Display> std::fmt::Display for Color3<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        "(".fmt(f)?;
        self.r.fmt(f)?;
        ", ".fmt(f)?;
        self.g.fmt(f)?;
        ", ".fmt(f)?;
        self.b.fmt(f)?;
        ")".fmt(f)
    }
}
