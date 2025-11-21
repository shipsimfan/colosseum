use crate::math::Quaternion;

impl<T: std::fmt::Display> std::fmt::Display for Quaternion<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        "(".fmt(f)?;
        self.x.fmt(f)?;
        ", ".fmt(f)?;
        self.y.fmt(f)?;
        ", ".fmt(f)?;
        self.z.fmt(f)?;
        ", ".fmt(f)?;
        self.w.fmt(f)?;
        ")".fmt(f)
    }
}
