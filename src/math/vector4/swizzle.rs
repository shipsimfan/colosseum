use crate::math::{Vector2, Vector3, Vector4};

impl<T> Vector4<T> {
    /// Gets this [`Vector4`] as a [`Vector4`] with values `(x, y, z, w)`
    pub const fn xyzw(self) -> Vector4<T> {
        self
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(x, y, w, z)`
    pub fn xywz(self) -> Vector4<T> {
        Vector4::new(self.x, self.y, self.w, self.z)
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(x, z, y, w)`
    pub fn xzyw(self) -> Vector4<T> {
        Vector4::new(self.x, self.z, self.y, self.w)
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(x, z, w, y)`
    pub fn xzwy(self) -> Vector4<T> {
        Vector4::new(self.x, self.z, self.w, self.y)
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(x, w, y, z)`
    pub fn xwyz(self) -> Vector4<T> {
        Vector4::new(self.x, self.w, self.y, self.z)
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(x, w, z, y)`
    pub fn xwzy(self) -> Vector4<T> {
        Vector4::new(self.x, self.w, self.z, self.y)
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(y, x, z, w)`
    pub fn yxzw(self) -> Vector4<T> {
        Vector4::new(self.y, self.x, self.z, self.w)
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(y, x, w, z)`
    pub fn yxwz(self) -> Vector4<T> {
        Vector4::new(self.y, self.x, self.w, self.z)
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(y, z, x, w)`
    pub fn yzxw(self) -> Vector4<T> {
        Vector4::new(self.y, self.z, self.x, self.w)
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(y, z, w, x)`
    pub fn yzwx(self) -> Vector4<T> {
        Vector4::new(self.y, self.z, self.w, self.x)
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(y, w, x, z)`
    pub fn ywxz(self) -> Vector4<T> {
        Vector4::new(self.y, self.w, self.x, self.z)
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(y, w, z, x)`
    pub fn ywzx(self) -> Vector4<T> {
        Vector4::new(self.y, self.w, self.z, self.x)
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(z, x, y, w)`
    pub fn zxyw(self) -> Vector4<T> {
        Vector4::new(self.z, self.x, self.y, self.w)
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(z, x, w, y)`
    pub fn zxwy(self) -> Vector4<T> {
        Vector4::new(self.z, self.x, self.w, self.y)
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(z, y, x, w)`
    pub fn zyxw(self) -> Vector4<T> {
        Vector4::new(self.z, self.y, self.x, self.w)
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(z, y, w, x)`
    pub fn zywx(self) -> Vector4<T> {
        Vector4::new(self.z, self.y, self.w, self.x)
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(z, w, x, y)`
    pub fn zwxy(self) -> Vector4<T> {
        Vector4::new(self.z, self.w, self.x, self.y)
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(z, w, y, x)`
    pub fn zwyx(self) -> Vector4<T> {
        Vector4::new(self.z, self.w, self.y, self.x)
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(w, x, y, z)`
    pub fn wxyz(self) -> Vector4<T> {
        Vector4::new(self.w, self.x, self.y, self.z)
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(w, x, z, y)`
    pub fn wxzy(self) -> Vector4<T> {
        Vector4::new(self.w, self.x, self.z, self.y)
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(w, y, x, z)`
    pub fn wyxz(self) -> Vector4<T> {
        Vector4::new(self.w, self.y, self.x, self.z)
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(w, y, z, x)`
    pub fn wyzx(self) -> Vector4<T> {
        Vector4::new(self.w, self.y, self.z, self.x)
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(w, z, x, y)`
    pub fn wzxy(self) -> Vector4<T> {
        Vector4::new(self.w, self.z, self.x, self.y)
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(w, z, y, x)`
    pub fn wzyx(self) -> Vector4<T> {
        Vector4::new(self.w, self.z, self.y, self.x)
    }

    /// Gets this [`Vector4`] as a [`Vector3`] with values `(x, y, z)`
    pub fn xyz(self) -> Vector3<T> {
        Vector3::new(self.x, self.y, self.z)
    }

    /// Gets this [`Vector4`] as a [`Vector3`] with values `(x, y, w)`
    pub fn xyw(self) -> Vector3<T> {
        Vector3::new(self.x, self.y, self.w)
    }

    /// Gets this [`Vector4`] as a [`Vector3`] with values `(x, z, y)`
    pub fn xzy(self) -> Vector3<T> {
        Vector3::new(self.x, self.z, self.y)
    }

    /// Gets this [`Vector4`] as a [`Vector3`] with values `(x, z, w)`
    pub fn xzw(self) -> Vector3<T> {
        Vector3::new(self.x, self.z, self.w)
    }

    /// Gets this [`Vector4`] as a [`Vector3`] with values `(x, w, y)`
    pub fn xwy(self) -> Vector3<T> {
        Vector3::new(self.x, self.w, self.y)
    }

    /// Gets this [`Vector4`] as a [`Vector3`] with values `(x, w, z)`
    pub fn xwz(self) -> Vector3<T> {
        Vector3::new(self.x, self.w, self.z)
    }

    /// Gets this [`Vector4`] as a [`Vector3`] with values `(y, x, z)`
    pub fn yxz(self) -> Vector3<T> {
        Vector3::new(self.y, self.x, self.z)
    }

    /// Gets this [`Vector4`] as a [`Vector3`] with values `(y, x, w)`
    pub fn yxw(self) -> Vector3<T> {
        Vector3::new(self.y, self.x, self.w)
    }

    /// Gets this [`Vector4`] as a [`Vector3`] with values `(y, z, x)`
    pub fn yzx(self) -> Vector3<T> {
        Vector3::new(self.y, self.z, self.x)
    }

    /// Gets this [`Vector4`] as a [`Vector3`] with values `(y, z, w)`
    pub fn yzw(self) -> Vector3<T> {
        Vector3::new(self.y, self.z, self.w)
    }

    /// Gets this [`Vector4`] as a [`Vector3`] with values `(y, w, x)`
    pub fn ywx(self) -> Vector3<T> {
        Vector3::new(self.y, self.w, self.x)
    }

    /// Gets this [`Vector4`] as a [`Vector3`] with values `(y, w, z)`
    pub fn ywz(self) -> Vector3<T> {
        Vector3::new(self.y, self.w, self.z)
    }

    /// Gets this [`Vector4`] as a [`Vector3`] with values `(z, x, y)`
    pub fn zxy(self) -> Vector3<T> {
        Vector3::new(self.z, self.x, self.y)
    }

    /// Gets this [`Vector4`] as a [`Vector3`] with values `(z, x, w)`
    pub fn zxw(self) -> Vector3<T> {
        Vector3::new(self.z, self.x, self.w)
    }

    /// Gets this [`Vector4`] as a [`Vector3`] with values `(z, y, x)`
    pub fn zyx(self) -> Vector3<T> {
        Vector3::new(self.z, self.y, self.x)
    }

    /// Gets this [`Vector4`] as a [`Vector3`] with values `(z, y, w)`
    pub fn zyw(self) -> Vector3<T> {
        Vector3::new(self.z, self.y, self.w)
    }

    /// Gets this [`Vector4`] as a [`Vector3`] with values `(z, w, x)`
    pub fn zwx(self) -> Vector3<T> {
        Vector3::new(self.z, self.w, self.x)
    }

    /// Gets this [`Vector4`] as a [`Vector3`] with values `(z, w, y)`
    pub fn zwy(self) -> Vector3<T> {
        Vector3::new(self.z, self.w, self.y)
    }

    /// Gets this [`Vector4`] as a [`Vector3`] with values `(w, x, y)`
    pub fn wxy(self) -> Vector3<T> {
        Vector3::new(self.w, self.x, self.y)
    }

    /// Gets this [`Vector4`] as a [`Vector3`] with values `(w, x, z)`
    pub fn wxz(self) -> Vector3<T> {
        Vector3::new(self.w, self.x, self.z)
    }

    /// Gets this [`Vector4`] as a [`Vector3`] with values `(w, y, x)`
    pub fn wyx(self) -> Vector3<T> {
        Vector3::new(self.w, self.y, self.x)
    }

    /// Gets this [`Vector4`] as a [`Vector3`] with values `(w, y, z)`
    pub fn wyz(self) -> Vector3<T> {
        Vector3::new(self.w, self.y, self.z)
    }

    /// Gets this [`Vector4`] as a [`Vector3`] with values `(w, z, x)`
    pub fn wzx(self) -> Vector3<T> {
        Vector3::new(self.w, self.z, self.x)
    }

    /// Gets this [`Vector4`] as a [`Vector3`] with values `(w, z, y)`
    pub fn wzy(self) -> Vector3<T> {
        Vector3::new(self.w, self.z, self.y)
    }

    /// Gets this [`Vector4`] as a [`Vector2`] with values `(x, y)`
    pub fn xy(self) -> Vector2<T> {
        Vector2::new(self.x, self.y)
    }

    /// Gets this [`Vector4`] as a [`Vector2`] with values `(x, z)`
    pub fn xz(self) -> Vector2<T> {
        Vector2::new(self.x, self.z)
    }

    /// Gets this [`Vector4`] as a [`Vector2`] with values `(x, w)`
    pub fn xw(self) -> Vector2<T> {
        Vector2::new(self.x, self.w)
    }

    /// Gets this [`Vector4`] as a [`Vector2`] with values `(y, x)`
    pub fn yx(self) -> Vector2<T> {
        Vector2::new(self.y, self.x)
    }

    /// Gets this [`Vector4`] as a [`Vector2`] with values `(y, z)`
    pub fn yz(self) -> Vector2<T> {
        Vector2::new(self.y, self.z)
    }

    /// Gets this [`Vector4`] as a [`Vector2`] with values `(y, w)`
    pub fn yw(self) -> Vector2<T> {
        Vector2::new(self.y, self.w)
    }

    /// Gets this [`Vector4`] as a [`Vector2`] with values `(z, x)`
    pub fn zx(self) -> Vector2<T> {
        Vector2::new(self.z, self.x)
    }

    /// Gets this [`Vector4`] as a [`Vector2`] with values `(z, y)`
    pub fn zy(self) -> Vector2<T> {
        Vector2::new(self.z, self.y)
    }

    /// Gets this [`Vector4`] as a [`Vector2`] with values `(z, w)`
    pub fn zw(self) -> Vector2<T> {
        Vector2::new(self.z, self.w)
    }

    /// Gets this [`Vector4`] as a [`Vector2`] with values `(w, x)`
    pub fn wx(self) -> Vector2<T> {
        Vector2::new(self.w, self.x)
    }

    /// Gets this [`Vector4`] as a [`Vector2`] with values `(w, y)`
    pub fn wy(self) -> Vector2<T> {
        Vector2::new(self.w, self.y)
    }

    /// Gets this [`Vector4`] as a [`Vector2`] with values `(w, z)`
    pub fn wz(self) -> Vector2<T> {
        Vector2::new(self.w, self.z)
    }
}

impl<T: Clone> Vector4<T> {
    /// Gets this [`Vector4`] as a [`Vector4`] with values `(x, x, x, x)`
    pub fn xxxx(self) -> Vector4<T> {
        Vector4::new(
            self.x.clone(),
            self.x.clone(),
            self.x.clone(),
            self.x.clone(),
        )
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(x, x, x, y)`
    pub fn xxxy(self) -> Vector4<T> {
        Vector4::new(
            self.x.clone(),
            self.x.clone(),
            self.x.clone(),
            self.y.clone(),
        )
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(x, x, x, z)`
    pub fn xxxz(self) -> Vector4<T> {
        Vector4::new(
            self.x.clone(),
            self.x.clone(),
            self.x.clone(),
            self.z.clone(),
        )
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(x, x, x, w)`
    pub fn xxxw(self) -> Vector4<T> {
        Vector4::new(
            self.x.clone(),
            self.x.clone(),
            self.x.clone(),
            self.w.clone(),
        )
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(x, x, y, x)`
    pub fn xxyx(self) -> Vector4<T> {
        Vector4::new(
            self.x.clone(),
            self.x.clone(),
            self.y.clone(),
            self.x.clone(),
        )
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(x, x, y, y)`
    pub fn xxyy(self) -> Vector4<T> {
        Vector4::new(
            self.x.clone(),
            self.x.clone(),
            self.y.clone(),
            self.y.clone(),
        )
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(x, x, y, z)`
    pub fn xxyz(self) -> Vector4<T> {
        Vector4::new(
            self.x.clone(),
            self.x.clone(),
            self.y.clone(),
            self.z.clone(),
        )
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(x, x, y, w)`
    pub fn xxyw(self) -> Vector4<T> {
        Vector4::new(
            self.x.clone(),
            self.x.clone(),
            self.y.clone(),
            self.w.clone(),
        )
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(x, x, z, x)`
    pub fn xxzx(self) -> Vector4<T> {
        Vector4::new(
            self.x.clone(),
            self.x.clone(),
            self.z.clone(),
            self.x.clone(),
        )
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(x, x, z, y)`
    pub fn xxzy(self) -> Vector4<T> {
        Vector4::new(
            self.x.clone(),
            self.x.clone(),
            self.z.clone(),
            self.y.clone(),
        )
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(x, x, z, z)`
    pub fn xxzz(self) -> Vector4<T> {
        Vector4::new(
            self.x.clone(),
            self.x.clone(),
            self.z.clone(),
            self.z.clone(),
        )
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(x, x, z, w)`
    pub fn xxzw(self) -> Vector4<T> {
        Vector4::new(
            self.x.clone(),
            self.x.clone(),
            self.z.clone(),
            self.w.clone(),
        )
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(x, x, w, x)`
    pub fn xxwx(self) -> Vector4<T> {
        Vector4::new(
            self.x.clone(),
            self.x.clone(),
            self.w.clone(),
            self.x.clone(),
        )
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(x, x, w, y)`
    pub fn xxwy(self) -> Vector4<T> {
        Vector4::new(
            self.x.clone(),
            self.x.clone(),
            self.w.clone(),
            self.y.clone(),
        )
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(x, x, w, z)`
    pub fn xxwz(self) -> Vector4<T> {
        Vector4::new(
            self.x.clone(),
            self.x.clone(),
            self.w.clone(),
            self.z.clone(),
        )
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(x, x, w, w)`
    pub fn xxww(self) -> Vector4<T> {
        Vector4::new(
            self.x.clone(),
            self.x.clone(),
            self.w.clone(),
            self.w.clone(),
        )
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(x, y, x, x)`
    pub fn xyxx(self) -> Vector4<T> {
        Vector4::new(
            self.x.clone(),
            self.y.clone(),
            self.x.clone(),
            self.x.clone(),
        )
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(x, y, x, y)`
    pub fn xyxy(self) -> Vector4<T> {
        Vector4::new(
            self.x.clone(),
            self.y.clone(),
            self.x.clone(),
            self.y.clone(),
        )
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(x, y, x, z)`
    pub fn xyxz(self) -> Vector4<T> {
        Vector4::new(
            self.x.clone(),
            self.y.clone(),
            self.x.clone(),
            self.z.clone(),
        )
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(x, y, x, w)`
    pub fn xyxw(self) -> Vector4<T> {
        Vector4::new(
            self.x.clone(),
            self.y.clone(),
            self.x.clone(),
            self.w.clone(),
        )
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(x, y, y, x)`
    pub fn xyyx(self) -> Vector4<T> {
        Vector4::new(
            self.x.clone(),
            self.y.clone(),
            self.y.clone(),
            self.x.clone(),
        )
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(x, y, y, y)`
    pub fn xyyy(self) -> Vector4<T> {
        Vector4::new(
            self.x.clone(),
            self.y.clone(),
            self.y.clone(),
            self.y.clone(),
        )
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(x, y, y, z)`
    pub fn xyyz(self) -> Vector4<T> {
        Vector4::new(
            self.x.clone(),
            self.y.clone(),
            self.y.clone(),
            self.z.clone(),
        )
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(x, y, y, w)`
    pub fn xyyw(self) -> Vector4<T> {
        Vector4::new(
            self.x.clone(),
            self.y.clone(),
            self.y.clone(),
            self.w.clone(),
        )
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(x, y, z, x)`
    pub fn xyzx(self) -> Vector4<T> {
        Vector4::new(
            self.x.clone(),
            self.y.clone(),
            self.z.clone(),
            self.x.clone(),
        )
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(x, y, z, y)`
    pub fn xyzy(self) -> Vector4<T> {
        Vector4::new(
            self.x.clone(),
            self.y.clone(),
            self.z.clone(),
            self.y.clone(),
        )
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(x, y, z, z)`
    pub fn xyzz(self) -> Vector4<T> {
        Vector4::new(
            self.x.clone(),
            self.y.clone(),
            self.z.clone(),
            self.z.clone(),
        )
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(x, y, w, x)`
    pub fn xywx(self) -> Vector4<T> {
        Vector4::new(
            self.x.clone(),
            self.y.clone(),
            self.w.clone(),
            self.x.clone(),
        )
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(x, y, w, y)`
    pub fn xywy(self) -> Vector4<T> {
        Vector4::new(
            self.x.clone(),
            self.y.clone(),
            self.w.clone(),
            self.y.clone(),
        )
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(x, y, w, w)`
    pub fn xyww(self) -> Vector4<T> {
        Vector4::new(
            self.x.clone(),
            self.y.clone(),
            self.w.clone(),
            self.w.clone(),
        )
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(x, z, x, x)`
    pub fn xzxx(self) -> Vector4<T> {
        Vector4::new(
            self.x.clone(),
            self.z.clone(),
            self.x.clone(),
            self.x.clone(),
        )
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(x, z, x, y)`
    pub fn xzxy(self) -> Vector4<T> {
        Vector4::new(
            self.x.clone(),
            self.z.clone(),
            self.x.clone(),
            self.y.clone(),
        )
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(x, z, x, z)`
    pub fn xzxz(self) -> Vector4<T> {
        Vector4::new(
            self.x.clone(),
            self.z.clone(),
            self.x.clone(),
            self.z.clone(),
        )
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(x, z, x, w)`
    pub fn xzxw(self) -> Vector4<T> {
        Vector4::new(
            self.x.clone(),
            self.z.clone(),
            self.x.clone(),
            self.w.clone(),
        )
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(x, z, y, x)`
    pub fn xzyx(self) -> Vector4<T> {
        Vector4::new(
            self.x.clone(),
            self.z.clone(),
            self.y.clone(),
            self.x.clone(),
        )
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(x, z, y, y)`
    pub fn xzyy(self) -> Vector4<T> {
        Vector4::new(
            self.x.clone(),
            self.z.clone(),
            self.y.clone(),
            self.y.clone(),
        )
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(x, z, y, z)`
    pub fn xzyz(self) -> Vector4<T> {
        Vector4::new(
            self.x.clone(),
            self.z.clone(),
            self.y.clone(),
            self.z.clone(),
        )
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(x, z, z, x)`
    pub fn xzzx(self) -> Vector4<T> {
        Vector4::new(
            self.x.clone(),
            self.z.clone(),
            self.z.clone(),
            self.x.clone(),
        )
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(x, z, z, y)`
    pub fn xzzy(self) -> Vector4<T> {
        Vector4::new(
            self.x.clone(),
            self.z.clone(),
            self.z.clone(),
            self.y.clone(),
        )
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(x, z, z, z)`
    pub fn xzzz(self) -> Vector4<T> {
        Vector4::new(
            self.x.clone(),
            self.z.clone(),
            self.z.clone(),
            self.z.clone(),
        )
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(x, z, z, w)`
    pub fn xzzw(self) -> Vector4<T> {
        Vector4::new(
            self.x.clone(),
            self.z.clone(),
            self.z.clone(),
            self.w.clone(),
        )
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(x, z, w, x)`
    pub fn xzwx(self) -> Vector4<T> {
        Vector4::new(
            self.x.clone(),
            self.z.clone(),
            self.w.clone(),
            self.x.clone(),
        )
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(x, z, w, z)`
    pub fn xzwz(self) -> Vector4<T> {
        Vector4::new(
            self.x.clone(),
            self.z.clone(),
            self.w.clone(),
            self.z.clone(),
        )
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(x, z, w, w)`
    pub fn xzww(self) -> Vector4<T> {
        Vector4::new(
            self.x.clone(),
            self.z.clone(),
            self.w.clone(),
            self.w.clone(),
        )
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(x, w, x, x)`
    pub fn xwxx(self) -> Vector4<T> {
        Vector4::new(
            self.x.clone(),
            self.w.clone(),
            self.x.clone(),
            self.x.clone(),
        )
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(x, w, x, y)`
    pub fn xwxy(self) -> Vector4<T> {
        Vector4::new(
            self.x.clone(),
            self.w.clone(),
            self.x.clone(),
            self.y.clone(),
        )
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(x, w, x, z)`
    pub fn xwxz(self) -> Vector4<T> {
        Vector4::new(
            self.x.clone(),
            self.w.clone(),
            self.x.clone(),
            self.z.clone(),
        )
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(x, w, x, w)`
    pub fn xwxw(self) -> Vector4<T> {
        Vector4::new(
            self.x.clone(),
            self.w.clone(),
            self.x.clone(),
            self.w.clone(),
        )
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(x, w, y, x)`
    pub fn xwyx(self) -> Vector4<T> {
        Vector4::new(
            self.x.clone(),
            self.w.clone(),
            self.y.clone(),
            self.x.clone(),
        )
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(x, w, y, y)`
    pub fn xwyy(self) -> Vector4<T> {
        Vector4::new(
            self.x.clone(),
            self.w.clone(),
            self.y.clone(),
            self.y.clone(),
        )
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(x, w, y, w)`
    pub fn xwyw(self) -> Vector4<T> {
        Vector4::new(
            self.x.clone(),
            self.w.clone(),
            self.y.clone(),
            self.w.clone(),
        )
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(x, w, z, x)`
    pub fn xwzx(self) -> Vector4<T> {
        Vector4::new(
            self.x.clone(),
            self.w.clone(),
            self.z.clone(),
            self.x.clone(),
        )
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(x, w, z, z)`
    pub fn xwzz(self) -> Vector4<T> {
        Vector4::new(
            self.x.clone(),
            self.w.clone(),
            self.z.clone(),
            self.z.clone(),
        )
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(x, w, z, w)`
    pub fn xwzw(self) -> Vector4<T> {
        Vector4::new(
            self.x.clone(),
            self.w.clone(),
            self.z.clone(),
            self.w.clone(),
        )
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(x, w, w, x)`
    pub fn xwwx(self) -> Vector4<T> {
        Vector4::new(
            self.x.clone(),
            self.w.clone(),
            self.w.clone(),
            self.x.clone(),
        )
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(x, w, w, y)`
    pub fn xwwy(self) -> Vector4<T> {
        Vector4::new(
            self.x.clone(),
            self.w.clone(),
            self.w.clone(),
            self.y.clone(),
        )
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(x, w, w, z)`
    pub fn xwwz(self) -> Vector4<T> {
        Vector4::new(
            self.x.clone(),
            self.w.clone(),
            self.w.clone(),
            self.z.clone(),
        )
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(x, w, w, w)`
    pub fn xwww(self) -> Vector4<T> {
        Vector4::new(
            self.x.clone(),
            self.w.clone(),
            self.w.clone(),
            self.w.clone(),
        )
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(y, x, x, x)`
    pub fn yxxx(self) -> Vector4<T> {
        Vector4::new(
            self.y.clone(),
            self.x.clone(),
            self.x.clone(),
            self.x.clone(),
        )
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(y, x, x, y)`
    pub fn yxxy(self) -> Vector4<T> {
        Vector4::new(
            self.y.clone(),
            self.x.clone(),
            self.x.clone(),
            self.y.clone(),
        )
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(y, x, x, z)`
    pub fn yxxz(self) -> Vector4<T> {
        Vector4::new(
            self.y.clone(),
            self.x.clone(),
            self.x.clone(),
            self.z.clone(),
        )
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(y, x, x, w)`
    pub fn yxxw(self) -> Vector4<T> {
        Vector4::new(
            self.y.clone(),
            self.x.clone(),
            self.x.clone(),
            self.w.clone(),
        )
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(y, x, y, x)`
    pub fn yxyx(self) -> Vector4<T> {
        Vector4::new(
            self.y.clone(),
            self.x.clone(),
            self.y.clone(),
            self.x.clone(),
        )
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(y, x, y, y)`
    pub fn yxyy(self) -> Vector4<T> {
        Vector4::new(
            self.y.clone(),
            self.x.clone(),
            self.y.clone(),
            self.y.clone(),
        )
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(y, x, y, z)`
    pub fn yxyz(self) -> Vector4<T> {
        Vector4::new(
            self.y.clone(),
            self.x.clone(),
            self.y.clone(),
            self.z.clone(),
        )
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(y, x, y, w)`
    pub fn yxyw(self) -> Vector4<T> {
        Vector4::new(
            self.y.clone(),
            self.x.clone(),
            self.y.clone(),
            self.w.clone(),
        )
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(y, x, z, x)`
    pub fn yxzx(self) -> Vector4<T> {
        Vector4::new(
            self.y.clone(),
            self.x.clone(),
            self.z.clone(),
            self.x.clone(),
        )
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(y, x, z, y)`
    pub fn yxzy(self) -> Vector4<T> {
        Vector4::new(
            self.y.clone(),
            self.x.clone(),
            self.z.clone(),
            self.y.clone(),
        )
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(y, x, z, z)`
    pub fn yxzz(self) -> Vector4<T> {
        Vector4::new(
            self.y.clone(),
            self.x.clone(),
            self.z.clone(),
            self.z.clone(),
        )
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(y, x, w, x)`
    pub fn yxwx(self) -> Vector4<T> {
        Vector4::new(
            self.y.clone(),
            self.x.clone(),
            self.w.clone(),
            self.x.clone(),
        )
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(y, x, w, y)`
    pub fn yxwy(self) -> Vector4<T> {
        Vector4::new(
            self.y.clone(),
            self.x.clone(),
            self.w.clone(),
            self.y.clone(),
        )
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(y, x, w, w)`
    pub fn yxww(self) -> Vector4<T> {
        Vector4::new(
            self.y.clone(),
            self.x.clone(),
            self.w.clone(),
            self.w.clone(),
        )
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(y, y, x, x)`
    pub fn yyxx(self) -> Vector4<T> {
        Vector4::new(
            self.y.clone(),
            self.y.clone(),
            self.x.clone(),
            self.x.clone(),
        )
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(y, y, x, y)`
    pub fn yyxy(self) -> Vector4<T> {
        Vector4::new(
            self.y.clone(),
            self.y.clone(),
            self.x.clone(),
            self.y.clone(),
        )
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(y, y, x, z)`
    pub fn yyxz(self) -> Vector4<T> {
        Vector4::new(
            self.y.clone(),
            self.y.clone(),
            self.x.clone(),
            self.z.clone(),
        )
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(y, y, x, w)`
    pub fn yyxw(self) -> Vector4<T> {
        Vector4::new(
            self.y.clone(),
            self.y.clone(),
            self.x.clone(),
            self.w.clone(),
        )
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(y, y, y, x)`
    pub fn yyyx(self) -> Vector4<T> {
        Vector4::new(
            self.y.clone(),
            self.y.clone(),
            self.y.clone(),
            self.x.clone(),
        )
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(y, y, y, y)`
    pub fn yyyy(self) -> Vector4<T> {
        Vector4::new(
            self.y.clone(),
            self.y.clone(),
            self.y.clone(),
            self.y.clone(),
        )
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(y, y, y, z)`
    pub fn yyyz(self) -> Vector4<T> {
        Vector4::new(
            self.y.clone(),
            self.y.clone(),
            self.y.clone(),
            self.z.clone(),
        )
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(y, y, y, w)`
    pub fn yyyw(self) -> Vector4<T> {
        Vector4::new(
            self.y.clone(),
            self.y.clone(),
            self.y.clone(),
            self.w.clone(),
        )
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(y, y, z, x)`
    pub fn yyzx(self) -> Vector4<T> {
        Vector4::new(
            self.y.clone(),
            self.y.clone(),
            self.z.clone(),
            self.x.clone(),
        )
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(y, y, z, y)`
    pub fn yyzy(self) -> Vector4<T> {
        Vector4::new(
            self.y.clone(),
            self.y.clone(),
            self.z.clone(),
            self.y.clone(),
        )
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(y, y, z, z)`
    pub fn yyzz(self) -> Vector4<T> {
        Vector4::new(
            self.y.clone(),
            self.y.clone(),
            self.z.clone(),
            self.z.clone(),
        )
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(y, y, z, w)`
    pub fn yyzw(self) -> Vector4<T> {
        Vector4::new(
            self.y.clone(),
            self.y.clone(),
            self.z.clone(),
            self.w.clone(),
        )
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(y, y, w, x)`
    pub fn yywx(self) -> Vector4<T> {
        Vector4::new(
            self.y.clone(),
            self.y.clone(),
            self.w.clone(),
            self.x.clone(),
        )
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(y, y, w, y)`
    pub fn yywy(self) -> Vector4<T> {
        Vector4::new(
            self.y.clone(),
            self.y.clone(),
            self.w.clone(),
            self.y.clone(),
        )
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(y, y, w, z)`
    pub fn yywz(self) -> Vector4<T> {
        Vector4::new(
            self.y.clone(),
            self.y.clone(),
            self.w.clone(),
            self.z.clone(),
        )
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(y, y, w, w)`
    pub fn yyww(self) -> Vector4<T> {
        Vector4::new(
            self.y.clone(),
            self.y.clone(),
            self.w.clone(),
            self.w.clone(),
        )
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(y, z, x, x)`
    pub fn yzxx(self) -> Vector4<T> {
        Vector4::new(
            self.y.clone(),
            self.z.clone(),
            self.x.clone(),
            self.x.clone(),
        )
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(y, z, x, y)`
    pub fn yzxy(self) -> Vector4<T> {
        Vector4::new(
            self.y.clone(),
            self.z.clone(),
            self.x.clone(),
            self.y.clone(),
        )
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(y, z, x, z)`
    pub fn yzxz(self) -> Vector4<T> {
        Vector4::new(
            self.y.clone(),
            self.z.clone(),
            self.x.clone(),
            self.z.clone(),
        )
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(y, z, y, x)`
    pub fn yzyx(self) -> Vector4<T> {
        Vector4::new(
            self.y.clone(),
            self.z.clone(),
            self.y.clone(),
            self.x.clone(),
        )
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(y, z, y, y)`
    pub fn yzyy(self) -> Vector4<T> {
        Vector4::new(
            self.y.clone(),
            self.z.clone(),
            self.y.clone(),
            self.y.clone(),
        )
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(y, z, y, z)`
    pub fn yzyz(self) -> Vector4<T> {
        Vector4::new(
            self.y.clone(),
            self.z.clone(),
            self.y.clone(),
            self.z.clone(),
        )
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(y, z, y, w)`
    pub fn yzyw(self) -> Vector4<T> {
        Vector4::new(
            self.y.clone(),
            self.z.clone(),
            self.y.clone(),
            self.w.clone(),
        )
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(y, z, z, x)`
    pub fn yzzx(self) -> Vector4<T> {
        Vector4::new(
            self.y.clone(),
            self.z.clone(),
            self.z.clone(),
            self.x.clone(),
        )
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(y, z, z, y)`
    pub fn yzzy(self) -> Vector4<T> {
        Vector4::new(
            self.y.clone(),
            self.z.clone(),
            self.z.clone(),
            self.y.clone(),
        )
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(y, z, z, z)`
    pub fn yzzz(self) -> Vector4<T> {
        Vector4::new(
            self.y.clone(),
            self.z.clone(),
            self.z.clone(),
            self.z.clone(),
        )
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(y, z, z, w)`
    pub fn yzzw(self) -> Vector4<T> {
        Vector4::new(
            self.y.clone(),
            self.z.clone(),
            self.z.clone(),
            self.w.clone(),
        )
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(y, z, w, y)`
    pub fn yzwy(self) -> Vector4<T> {
        Vector4::new(
            self.y.clone(),
            self.z.clone(),
            self.w.clone(),
            self.y.clone(),
        )
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(y, z, w, z)`
    pub fn yzwz(self) -> Vector4<T> {
        Vector4::new(
            self.y.clone(),
            self.z.clone(),
            self.w.clone(),
            self.z.clone(),
        )
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(y, z, w, w)`
    pub fn yzww(self) -> Vector4<T> {
        Vector4::new(
            self.y.clone(),
            self.z.clone(),
            self.w.clone(),
            self.w.clone(),
        )
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(y, w, x, x)`
    pub fn ywxx(self) -> Vector4<T> {
        Vector4::new(
            self.y.clone(),
            self.w.clone(),
            self.x.clone(),
            self.x.clone(),
        )
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(y, w, x, y)`
    pub fn ywxy(self) -> Vector4<T> {
        Vector4::new(
            self.y.clone(),
            self.w.clone(),
            self.x.clone(),
            self.y.clone(),
        )
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(y, w, x, w)`
    pub fn ywxw(self) -> Vector4<T> {
        Vector4::new(
            self.y.clone(),
            self.w.clone(),
            self.x.clone(),
            self.w.clone(),
        )
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(y, w, y, x)`
    pub fn ywyx(self) -> Vector4<T> {
        Vector4::new(
            self.y.clone(),
            self.w.clone(),
            self.y.clone(),
            self.x.clone(),
        )
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(y, w, y, y)`
    pub fn ywyy(self) -> Vector4<T> {
        Vector4::new(
            self.y.clone(),
            self.w.clone(),
            self.y.clone(),
            self.y.clone(),
        )
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(y, w, y, z)`
    pub fn ywyz(self) -> Vector4<T> {
        Vector4::new(
            self.y.clone(),
            self.w.clone(),
            self.y.clone(),
            self.z.clone(),
        )
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(y, w, y, w)`
    pub fn ywyw(self) -> Vector4<T> {
        Vector4::new(
            self.y.clone(),
            self.w.clone(),
            self.y.clone(),
            self.w.clone(),
        )
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(y, w, z, y)`
    pub fn ywzy(self) -> Vector4<T> {
        Vector4::new(
            self.y.clone(),
            self.w.clone(),
            self.z.clone(),
            self.y.clone(),
        )
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(y, w, z, z)`
    pub fn ywzz(self) -> Vector4<T> {
        Vector4::new(
            self.y.clone(),
            self.w.clone(),
            self.z.clone(),
            self.z.clone(),
        )
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(y, w, z, w)`
    pub fn ywzw(self) -> Vector4<T> {
        Vector4::new(
            self.y.clone(),
            self.w.clone(),
            self.z.clone(),
            self.w.clone(),
        )
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(y, w, w, x)`
    pub fn ywwx(self) -> Vector4<T> {
        Vector4::new(
            self.y.clone(),
            self.w.clone(),
            self.w.clone(),
            self.x.clone(),
        )
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(y, w, w, y)`
    pub fn ywwy(self) -> Vector4<T> {
        Vector4::new(
            self.y.clone(),
            self.w.clone(),
            self.w.clone(),
            self.y.clone(),
        )
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(y, w, w, z)`
    pub fn ywwz(self) -> Vector4<T> {
        Vector4::new(
            self.y.clone(),
            self.w.clone(),
            self.w.clone(),
            self.z.clone(),
        )
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(y, w, w, w)`
    pub fn ywww(self) -> Vector4<T> {
        Vector4::new(
            self.y.clone(),
            self.w.clone(),
            self.w.clone(),
            self.w.clone(),
        )
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(z, x, x, x)`
    pub fn zxxx(self) -> Vector4<T> {
        Vector4::new(
            self.z.clone(),
            self.x.clone(),
            self.x.clone(),
            self.x.clone(),
        )
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(z, x, x, y)`
    pub fn zxxy(self) -> Vector4<T> {
        Vector4::new(
            self.z.clone(),
            self.x.clone(),
            self.x.clone(),
            self.y.clone(),
        )
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(z, x, x, z)`
    pub fn zxxz(self) -> Vector4<T> {
        Vector4::new(
            self.z.clone(),
            self.x.clone(),
            self.x.clone(),
            self.z.clone(),
        )
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(z, x, x, w)`
    pub fn zxxw(self) -> Vector4<T> {
        Vector4::new(
            self.z.clone(),
            self.x.clone(),
            self.x.clone(),
            self.w.clone(),
        )
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(z, x, y, x)`
    pub fn zxyx(self) -> Vector4<T> {
        Vector4::new(
            self.z.clone(),
            self.x.clone(),
            self.y.clone(),
            self.x.clone(),
        )
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(z, x, y, y)`
    pub fn zxyy(self) -> Vector4<T> {
        Vector4::new(
            self.z.clone(),
            self.x.clone(),
            self.y.clone(),
            self.y.clone(),
        )
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(z, x, y, z)`
    pub fn zxyz(self) -> Vector4<T> {
        Vector4::new(
            self.z.clone(),
            self.x.clone(),
            self.y.clone(),
            self.z.clone(),
        )
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(z, x, z, x)`
    pub fn zxzx(self) -> Vector4<T> {
        Vector4::new(
            self.z.clone(),
            self.x.clone(),
            self.z.clone(),
            self.x.clone(),
        )
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(z, x, z, y)`
    pub fn zxzy(self) -> Vector4<T> {
        Vector4::new(
            self.z.clone(),
            self.x.clone(),
            self.z.clone(),
            self.y.clone(),
        )
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(z, x, z, z)`
    pub fn zxzz(self) -> Vector4<T> {
        Vector4::new(
            self.z.clone(),
            self.x.clone(),
            self.z.clone(),
            self.z.clone(),
        )
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(z, x, z, w)`
    pub fn zxzw(self) -> Vector4<T> {
        Vector4::new(
            self.z.clone(),
            self.x.clone(),
            self.z.clone(),
            self.w.clone(),
        )
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(z, x, w, x)`
    pub fn zxwx(self) -> Vector4<T> {
        Vector4::new(
            self.z.clone(),
            self.x.clone(),
            self.w.clone(),
            self.x.clone(),
        )
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(z, x, w, z)`
    pub fn zxwz(self) -> Vector4<T> {
        Vector4::new(
            self.z.clone(),
            self.x.clone(),
            self.w.clone(),
            self.z.clone(),
        )
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(z, x, w, w)`
    pub fn zxww(self) -> Vector4<T> {
        Vector4::new(
            self.z.clone(),
            self.x.clone(),
            self.w.clone(),
            self.w.clone(),
        )
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(z, y, x, x)`
    pub fn zyxx(self) -> Vector4<T> {
        Vector4::new(
            self.z.clone(),
            self.y.clone(),
            self.x.clone(),
            self.x.clone(),
        )
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(z, y, x, y)`
    pub fn zyxy(self) -> Vector4<T> {
        Vector4::new(
            self.z.clone(),
            self.y.clone(),
            self.x.clone(),
            self.y.clone(),
        )
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(z, y, x, z)`
    pub fn zyxz(self) -> Vector4<T> {
        Vector4::new(
            self.z.clone(),
            self.y.clone(),
            self.x.clone(),
            self.z.clone(),
        )
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(z, y, y, x)`
    pub fn zyyx(self) -> Vector4<T> {
        Vector4::new(
            self.z.clone(),
            self.y.clone(),
            self.y.clone(),
            self.x.clone(),
        )
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(z, y, y, y)`
    pub fn zyyy(self) -> Vector4<T> {
        Vector4::new(
            self.z.clone(),
            self.y.clone(),
            self.y.clone(),
            self.y.clone(),
        )
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(z, y, y, z)`
    pub fn zyyz(self) -> Vector4<T> {
        Vector4::new(
            self.z.clone(),
            self.y.clone(),
            self.y.clone(),
            self.z.clone(),
        )
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(z, y, y, w)`
    pub fn zyyw(self) -> Vector4<T> {
        Vector4::new(
            self.z.clone(),
            self.y.clone(),
            self.y.clone(),
            self.w.clone(),
        )
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(z, y, z, x)`
    pub fn zyzx(self) -> Vector4<T> {
        Vector4::new(
            self.z.clone(),
            self.y.clone(),
            self.z.clone(),
            self.x.clone(),
        )
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(z, y, z, y)`
    pub fn zyzy(self) -> Vector4<T> {
        Vector4::new(
            self.z.clone(),
            self.y.clone(),
            self.z.clone(),
            self.y.clone(),
        )
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(z, y, z, z)`
    pub fn zyzz(self) -> Vector4<T> {
        Vector4::new(
            self.z.clone(),
            self.y.clone(),
            self.z.clone(),
            self.z.clone(),
        )
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(z, y, z, w)`
    pub fn zyzw(self) -> Vector4<T> {
        Vector4::new(
            self.z.clone(),
            self.y.clone(),
            self.z.clone(),
            self.w.clone(),
        )
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(z, y, w, y)`
    pub fn zywy(self) -> Vector4<T> {
        Vector4::new(
            self.z.clone(),
            self.y.clone(),
            self.w.clone(),
            self.y.clone(),
        )
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(z, y, w, z)`
    pub fn zywz(self) -> Vector4<T> {
        Vector4::new(
            self.z.clone(),
            self.y.clone(),
            self.w.clone(),
            self.z.clone(),
        )
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(z, y, w, w)`
    pub fn zyww(self) -> Vector4<T> {
        Vector4::new(
            self.z.clone(),
            self.y.clone(),
            self.w.clone(),
            self.w.clone(),
        )
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(z, z, x, x)`
    pub fn zzxx(self) -> Vector4<T> {
        Vector4::new(
            self.z.clone(),
            self.z.clone(),
            self.x.clone(),
            self.x.clone(),
        )
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(z, z, x, y)`
    pub fn zzxy(self) -> Vector4<T> {
        Vector4::new(
            self.z.clone(),
            self.z.clone(),
            self.x.clone(),
            self.y.clone(),
        )
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(z, z, x, z)`
    pub fn zzxz(self) -> Vector4<T> {
        Vector4::new(
            self.z.clone(),
            self.z.clone(),
            self.x.clone(),
            self.z.clone(),
        )
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(z, z, x, w)`
    pub fn zzxw(self) -> Vector4<T> {
        Vector4::new(
            self.z.clone(),
            self.z.clone(),
            self.x.clone(),
            self.w.clone(),
        )
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(z, z, y, x)`
    pub fn zzyx(self) -> Vector4<T> {
        Vector4::new(
            self.z.clone(),
            self.z.clone(),
            self.y.clone(),
            self.x.clone(),
        )
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(z, z, y, y)`
    pub fn zzyy(self) -> Vector4<T> {
        Vector4::new(
            self.z.clone(),
            self.z.clone(),
            self.y.clone(),
            self.y.clone(),
        )
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(z, z, y, z)`
    pub fn zzyz(self) -> Vector4<T> {
        Vector4::new(
            self.z.clone(),
            self.z.clone(),
            self.y.clone(),
            self.z.clone(),
        )
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(z, z, y, w)`
    pub fn zzyw(self) -> Vector4<T> {
        Vector4::new(
            self.z.clone(),
            self.z.clone(),
            self.y.clone(),
            self.w.clone(),
        )
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(z, z, z, x)`
    pub fn zzzx(self) -> Vector4<T> {
        Vector4::new(
            self.z.clone(),
            self.z.clone(),
            self.z.clone(),
            self.x.clone(),
        )
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(z, z, z, y)`
    pub fn zzzy(self) -> Vector4<T> {
        Vector4::new(
            self.z.clone(),
            self.z.clone(),
            self.z.clone(),
            self.y.clone(),
        )
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(z, z, z, z)`
    pub fn zzzz(self) -> Vector4<T> {
        Vector4::new(
            self.z.clone(),
            self.z.clone(),
            self.z.clone(),
            self.z.clone(),
        )
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(z, z, z, w)`
    pub fn zzzw(self) -> Vector4<T> {
        Vector4::new(
            self.z.clone(),
            self.z.clone(),
            self.z.clone(),
            self.w.clone(),
        )
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(z, w, x, x)`
    pub fn zwxx(self) -> Vector4<T> {
        Vector4::new(
            self.z.clone(),
            self.w.clone(),
            self.x.clone(),
            self.x.clone(),
        )
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(z, w, x, z)`
    pub fn zwxz(self) -> Vector4<T> {
        Vector4::new(
            self.z.clone(),
            self.w.clone(),
            self.x.clone(),
            self.z.clone(),
        )
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(z, w, x, w)`
    pub fn zwxw(self) -> Vector4<T> {
        Vector4::new(
            self.z.clone(),
            self.w.clone(),
            self.x.clone(),
            self.w.clone(),
        )
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(z, w, y, y)`
    pub fn zwyy(self) -> Vector4<T> {
        Vector4::new(
            self.z.clone(),
            self.w.clone(),
            self.y.clone(),
            self.y.clone(),
        )
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(z, w, y, z)`
    pub fn zwyz(self) -> Vector4<T> {
        Vector4::new(
            self.z.clone(),
            self.w.clone(),
            self.y.clone(),
            self.z.clone(),
        )
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(z, w, y, w)`
    pub fn zwyw(self) -> Vector4<T> {
        Vector4::new(
            self.z.clone(),
            self.w.clone(),
            self.y.clone(),
            self.w.clone(),
        )
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(z, w, z, x)`
    pub fn zwzx(self) -> Vector4<T> {
        Vector4::new(
            self.z.clone(),
            self.w.clone(),
            self.z.clone(),
            self.x.clone(),
        )
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(z, w, z, y)`
    pub fn zwzy(self) -> Vector4<T> {
        Vector4::new(
            self.z.clone(),
            self.w.clone(),
            self.z.clone(),
            self.y.clone(),
        )
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(z, w, z, z)`
    pub fn zwzz(self) -> Vector4<T> {
        Vector4::new(
            self.z.clone(),
            self.w.clone(),
            self.z.clone(),
            self.z.clone(),
        )
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(z, w, z, w)`
    pub fn zwzw(self) -> Vector4<T> {
        Vector4::new(
            self.z.clone(),
            self.w.clone(),
            self.z.clone(),
            self.w.clone(),
        )
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(z, w, w, x)`
    pub fn zwwx(self) -> Vector4<T> {
        Vector4::new(
            self.z.clone(),
            self.w.clone(),
            self.w.clone(),
            self.x.clone(),
        )
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(z, w, w, y)`
    pub fn zwwy(self) -> Vector4<T> {
        Vector4::new(
            self.z.clone(),
            self.w.clone(),
            self.w.clone(),
            self.y.clone(),
        )
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(z, w, w, z)`
    pub fn zwwz(self) -> Vector4<T> {
        Vector4::new(
            self.z.clone(),
            self.w.clone(),
            self.w.clone(),
            self.z.clone(),
        )
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(z, w, w, w)`
    pub fn zwww(self) -> Vector4<T> {
        Vector4::new(
            self.z.clone(),
            self.w.clone(),
            self.w.clone(),
            self.w.clone(),
        )
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(w, x, x, x)`
    pub fn wxxx(self) -> Vector4<T> {
        Vector4::new(
            self.w.clone(),
            self.x.clone(),
            self.x.clone(),
            self.x.clone(),
        )
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(w, x, x, y)`
    pub fn wxxy(self) -> Vector4<T> {
        Vector4::new(
            self.w.clone(),
            self.x.clone(),
            self.x.clone(),
            self.y.clone(),
        )
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(w, x, x, z)`
    pub fn wxxz(self) -> Vector4<T> {
        Vector4::new(
            self.w.clone(),
            self.x.clone(),
            self.x.clone(),
            self.z.clone(),
        )
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(w, x, x, w)`
    pub fn wxxw(self) -> Vector4<T> {
        Vector4::new(
            self.w.clone(),
            self.x.clone(),
            self.x.clone(),
            self.w.clone(),
        )
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(w, x, y, x)`
    pub fn wxyx(self) -> Vector4<T> {
        Vector4::new(
            self.w.clone(),
            self.x.clone(),
            self.y.clone(),
            self.x.clone(),
        )
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(w, x, y, y)`
    pub fn wxyy(self) -> Vector4<T> {
        Vector4::new(
            self.w.clone(),
            self.x.clone(),
            self.y.clone(),
            self.y.clone(),
        )
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(w, x, y, w)`
    pub fn wxyw(self) -> Vector4<T> {
        Vector4::new(
            self.w.clone(),
            self.x.clone(),
            self.y.clone(),
            self.w.clone(),
        )
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(w, x, z, x)`
    pub fn wxzx(self) -> Vector4<T> {
        Vector4::new(
            self.w.clone(),
            self.x.clone(),
            self.z.clone(),
            self.x.clone(),
        )
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(w, x, z, z)`
    pub fn wxzz(self) -> Vector4<T> {
        Vector4::new(
            self.w.clone(),
            self.x.clone(),
            self.z.clone(),
            self.z.clone(),
        )
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(w, x, z, w)`
    pub fn wxzw(self) -> Vector4<T> {
        Vector4::new(
            self.w.clone(),
            self.x.clone(),
            self.z.clone(),
            self.w.clone(),
        )
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(w, x, w, x)`
    pub fn wxwx(self) -> Vector4<T> {
        Vector4::new(
            self.w.clone(),
            self.x.clone(),
            self.w.clone(),
            self.x.clone(),
        )
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(w, x, w, y)`
    pub fn wxwy(self) -> Vector4<T> {
        Vector4::new(
            self.w.clone(),
            self.x.clone(),
            self.w.clone(),
            self.y.clone(),
        )
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(w, x, w, z)`
    pub fn wxwz(self) -> Vector4<T> {
        Vector4::new(
            self.w.clone(),
            self.x.clone(),
            self.w.clone(),
            self.z.clone(),
        )
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(w, x, w, w)`
    pub fn wxww(self) -> Vector4<T> {
        Vector4::new(
            self.w.clone(),
            self.x.clone(),
            self.w.clone(),
            self.w.clone(),
        )
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(w, y, x, x)`
    pub fn wyxx(self) -> Vector4<T> {
        Vector4::new(
            self.w.clone(),
            self.y.clone(),
            self.x.clone(),
            self.x.clone(),
        )
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(w, y, x, y)`
    pub fn wyxy(self) -> Vector4<T> {
        Vector4::new(
            self.w.clone(),
            self.y.clone(),
            self.x.clone(),
            self.y.clone(),
        )
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(w, y, x, w)`
    pub fn wyxw(self) -> Vector4<T> {
        Vector4::new(
            self.w.clone(),
            self.y.clone(),
            self.x.clone(),
            self.w.clone(),
        )
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(w, y, y, x)`
    pub fn wyyx(self) -> Vector4<T> {
        Vector4::new(
            self.w.clone(),
            self.y.clone(),
            self.y.clone(),
            self.x.clone(),
        )
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(w, y, y, y)`
    pub fn wyyy(self) -> Vector4<T> {
        Vector4::new(
            self.w.clone(),
            self.y.clone(),
            self.y.clone(),
            self.y.clone(),
        )
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(w, y, y, z)`
    pub fn wyyz(self) -> Vector4<T> {
        Vector4::new(
            self.w.clone(),
            self.y.clone(),
            self.y.clone(),
            self.z.clone(),
        )
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(w, y, y, w)`
    pub fn wyyw(self) -> Vector4<T> {
        Vector4::new(
            self.w.clone(),
            self.y.clone(),
            self.y.clone(),
            self.w.clone(),
        )
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(w, y, z, y)`
    pub fn wyzy(self) -> Vector4<T> {
        Vector4::new(
            self.w.clone(),
            self.y.clone(),
            self.z.clone(),
            self.y.clone(),
        )
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(w, y, z, z)`
    pub fn wyzz(self) -> Vector4<T> {
        Vector4::new(
            self.w.clone(),
            self.y.clone(),
            self.z.clone(),
            self.z.clone(),
        )
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(w, y, z, w)`
    pub fn wyzw(self) -> Vector4<T> {
        Vector4::new(
            self.w.clone(),
            self.y.clone(),
            self.z.clone(),
            self.w.clone(),
        )
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(w, y, w, x)`
    pub fn wywx(self) -> Vector4<T> {
        Vector4::new(
            self.w.clone(),
            self.y.clone(),
            self.w.clone(),
            self.x.clone(),
        )
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(w, y, w, y)`
    pub fn wywy(self) -> Vector4<T> {
        Vector4::new(
            self.w.clone(),
            self.y.clone(),
            self.w.clone(),
            self.y.clone(),
        )
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(w, y, w, z)`
    pub fn wywz(self) -> Vector4<T> {
        Vector4::new(
            self.w.clone(),
            self.y.clone(),
            self.w.clone(),
            self.z.clone(),
        )
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(w, y, w, w)`
    pub fn wyww(self) -> Vector4<T> {
        Vector4::new(
            self.w.clone(),
            self.y.clone(),
            self.w.clone(),
            self.w.clone(),
        )
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(w, z, x, x)`
    pub fn wzxx(self) -> Vector4<T> {
        Vector4::new(
            self.w.clone(),
            self.z.clone(),
            self.x.clone(),
            self.x.clone(),
        )
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(w, z, x, z)`
    pub fn wzxz(self) -> Vector4<T> {
        Vector4::new(
            self.w.clone(),
            self.z.clone(),
            self.x.clone(),
            self.z.clone(),
        )
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(w, z, x, w)`
    pub fn wzxw(self) -> Vector4<T> {
        Vector4::new(
            self.w.clone(),
            self.z.clone(),
            self.x.clone(),
            self.w.clone(),
        )
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(w, z, y, y)`
    pub fn wzyy(self) -> Vector4<T> {
        Vector4::new(
            self.w.clone(),
            self.z.clone(),
            self.y.clone(),
            self.y.clone(),
        )
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(w, z, y, z)`
    pub fn wzyz(self) -> Vector4<T> {
        Vector4::new(
            self.w.clone(),
            self.z.clone(),
            self.y.clone(),
            self.z.clone(),
        )
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(w, z, y, w)`
    pub fn wzyw(self) -> Vector4<T> {
        Vector4::new(
            self.w.clone(),
            self.z.clone(),
            self.y.clone(),
            self.w.clone(),
        )
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(w, z, z, x)`
    pub fn wzzx(self) -> Vector4<T> {
        Vector4::new(
            self.w.clone(),
            self.z.clone(),
            self.z.clone(),
            self.x.clone(),
        )
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(w, z, z, y)`
    pub fn wzzy(self) -> Vector4<T> {
        Vector4::new(
            self.w.clone(),
            self.z.clone(),
            self.z.clone(),
            self.y.clone(),
        )
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(w, z, z, z)`
    pub fn wzzz(self) -> Vector4<T> {
        Vector4::new(
            self.w.clone(),
            self.z.clone(),
            self.z.clone(),
            self.z.clone(),
        )
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(w, z, z, w)`
    pub fn wzzw(self) -> Vector4<T> {
        Vector4::new(
            self.w.clone(),
            self.z.clone(),
            self.z.clone(),
            self.w.clone(),
        )
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(w, z, w, x)`
    pub fn wzwx(self) -> Vector4<T> {
        Vector4::new(
            self.w.clone(),
            self.z.clone(),
            self.w.clone(),
            self.x.clone(),
        )
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(w, z, w, y)`
    pub fn wzwy(self) -> Vector4<T> {
        Vector4::new(
            self.w.clone(),
            self.z.clone(),
            self.w.clone(),
            self.y.clone(),
        )
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(w, z, w, z)`
    pub fn wzwz(self) -> Vector4<T> {
        Vector4::new(
            self.w.clone(),
            self.z.clone(),
            self.w.clone(),
            self.z.clone(),
        )
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(w, z, w, w)`
    pub fn wzww(self) -> Vector4<T> {
        Vector4::new(
            self.w.clone(),
            self.z.clone(),
            self.w.clone(),
            self.w.clone(),
        )
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(w, w, x, x)`
    pub fn wwxx(self) -> Vector4<T> {
        Vector4::new(
            self.w.clone(),
            self.w.clone(),
            self.x.clone(),
            self.x.clone(),
        )
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(w, w, x, y)`
    pub fn wwxy(self) -> Vector4<T> {
        Vector4::new(
            self.w.clone(),
            self.w.clone(),
            self.x.clone(),
            self.y.clone(),
        )
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(w, w, x, z)`
    pub fn wwxz(self) -> Vector4<T> {
        Vector4::new(
            self.w.clone(),
            self.w.clone(),
            self.x.clone(),
            self.z.clone(),
        )
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(w, w, x, w)`
    pub fn wwxw(self) -> Vector4<T> {
        Vector4::new(
            self.w.clone(),
            self.w.clone(),
            self.x.clone(),
            self.w.clone(),
        )
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(w, w, y, x)`
    pub fn wwyx(self) -> Vector4<T> {
        Vector4::new(
            self.w.clone(),
            self.w.clone(),
            self.y.clone(),
            self.x.clone(),
        )
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(w, w, y, y)`
    pub fn wwyy(self) -> Vector4<T> {
        Vector4::new(
            self.w.clone(),
            self.w.clone(),
            self.y.clone(),
            self.y.clone(),
        )
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(w, w, y, z)`
    pub fn wwyz(self) -> Vector4<T> {
        Vector4::new(
            self.w.clone(),
            self.w.clone(),
            self.y.clone(),
            self.z.clone(),
        )
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(w, w, y, w)`
    pub fn wwyw(self) -> Vector4<T> {
        Vector4::new(
            self.w.clone(),
            self.w.clone(),
            self.y.clone(),
            self.w.clone(),
        )
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(w, w, z, x)`
    pub fn wwzx(self) -> Vector4<T> {
        Vector4::new(
            self.w.clone(),
            self.w.clone(),
            self.z.clone(),
            self.x.clone(),
        )
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(w, w, z, y)`
    pub fn wwzy(self) -> Vector4<T> {
        Vector4::new(
            self.w.clone(),
            self.w.clone(),
            self.z.clone(),
            self.y.clone(),
        )
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(w, w, z, z)`
    pub fn wwzz(self) -> Vector4<T> {
        Vector4::new(
            self.w.clone(),
            self.w.clone(),
            self.z.clone(),
            self.z.clone(),
        )
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(w, w, z, w)`
    pub fn wwzw(self) -> Vector4<T> {
        Vector4::new(
            self.w.clone(),
            self.w.clone(),
            self.z.clone(),
            self.w.clone(),
        )
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(w, w, w, x)`
    pub fn wwwx(self) -> Vector4<T> {
        Vector4::new(
            self.w.clone(),
            self.w.clone(),
            self.w.clone(),
            self.x.clone(),
        )
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(w, w, w, y)`
    pub fn wwwy(self) -> Vector4<T> {
        Vector4::new(
            self.w.clone(),
            self.w.clone(),
            self.w.clone(),
            self.y.clone(),
        )
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(w, w, w, z)`
    pub fn wwwz(self) -> Vector4<T> {
        Vector4::new(
            self.w.clone(),
            self.w.clone(),
            self.w.clone(),
            self.z.clone(),
        )
    }

    /// Gets this [`Vector4`] as a [`Vector4`] with values `(w, w, w, w)`
    pub fn wwww(self) -> Vector4<T> {
        Vector4::new(
            self.w.clone(),
            self.w.clone(),
            self.w.clone(),
            self.w.clone(),
        )
    }
}

impl<T: Clone> Vector4<T> {
    /// Gets this [`Vector4`] as a [`Vector3`] with values `(x, x, x)`
    pub fn xxx(self) -> Vector3<T> {
        Vector3::new(self.x.clone(), self.x.clone(), self.x)
    }

    /// Gets this [`Vector4`] as a [`Vector3`] with values `(x, x, y)`
    pub fn xxy(self) -> Vector3<T> {
        Vector3::new(self.x.clone(), self.x, self.y)
    }

    /// Gets this [`Vector4`] as a [`Vector3`] with values `(x, x, z)`
    pub fn xxz(self) -> Vector3<T> {
        Vector3::new(self.x.clone(), self.x, self.z)
    }

    /// Gets this [`Vector4`] as a [`Vector3`] with values `(x, x, w)`
    pub fn xxw(self) -> Vector3<T> {
        Vector3::new(self.x.clone(), self.x, self.w)
    }

    /// Gets this [`Vector4`] as a [`Vector3`] with values `(x, y, x)`
    pub fn xyx(self) -> Vector3<T> {
        Vector3::new(self.x.clone(), self.y, self.x)
    }

    /// Gets this [`Vector4`] as a [`Vector3`] with values `(x, y, y)`
    pub fn xyy(self) -> Vector3<T> {
        Vector3::new(self.x, self.y.clone(), self.y)
    }

    /// Gets this [`Vector4`] as a [`Vector3`] with values `(x, z, x)`
    pub fn xzx(self) -> Vector3<T> {
        Vector3::new(self.x.clone(), self.z, self.x)
    }

    /// Gets this [`Vector4`] as a [`Vector3`] with values `(x, z, z)`
    pub fn xzz(self) -> Vector3<T> {
        Vector3::new(self.x, self.z.clone(), self.z)
    }

    /// Gets this [`Vector4`] as a [`Vector3`] with values `(x, w, x)`
    pub fn xwx(self) -> Vector3<T> {
        Vector3::new(self.x.clone(), self.w, self.x)
    }

    /// Gets this [`Vector4`] as a [`Vector3`] with values `(x, w, w)`
    pub fn xww(self) -> Vector3<T> {
        Vector3::new(self.x, self.w.clone(), self.w)
    }

    /// Gets this [`Vector4`] as a [`Vector3`] with values `(y, x, x)`
    pub fn yxx(self) -> Vector3<T> {
        Vector3::new(self.y, self.x.clone(), self.x)
    }

    /// Gets this [`Vector4`] as a [`Vector3`] with values `(y, x, y)`
    pub fn yxy(self) -> Vector3<T> {
        Vector3::new(self.y.clone(), self.x, self.y)
    }

    /// Gets this [`Vector4`] as a [`Vector3`] with values `(y, y, y)`
    pub fn yyy(self) -> Vector3<T> {
        Vector3::new(self.y.clone(), self.y.clone(), self.y)
    }

    /// Gets this [`Vector4`] as a [`Vector3`] with values `(y, y, x)`
    pub fn yyx(self) -> Vector3<T> {
        Vector3::new(self.y.clone(), self.y, self.x)
    }

    /// Gets this [`Vector4`] as a [`Vector3`] with values `(y, y, z)`
    pub fn yyz(self) -> Vector3<T> {
        Vector3::new(self.y.clone(), self.y, self.z)
    }

    /// Gets this [`Vector4`] as a [`Vector3`] with values `(y, y, w)`
    pub fn yyw(self) -> Vector3<T> {
        Vector3::new(self.y.clone(), self.y, self.w)
    }

    /// Gets this [`Vector4`] as a [`Vector3`] with values `(y, z, y)`
    pub fn yzy(self) -> Vector3<T> {
        Vector3::new(self.y.clone(), self.z, self.y)
    }

    /// Gets this [`Vector4`] as a [`Vector3`] with values `(y, z, z)`
    pub fn yzz(self) -> Vector3<T> {
        Vector3::new(self.y, self.z.clone(), self.z)
    }

    /// Gets this [`Vector4`] as a [`Vector3`] with values `(y, w, y)`
    pub fn ywy(self) -> Vector3<T> {
        Vector3::new(self.y.clone(), self.w, self.y)
    }

    /// Gets this [`Vector4`] as a [`Vector3`] with values `(y, w, w)`
    pub fn yww(self) -> Vector3<T> {
        Vector3::new(self.y, self.w.clone(), self.w)
    }

    /// Gets this [`Vector4`] as a [`Vector3`] with values `(z, x, x)`
    pub fn zxx(self) -> Vector3<T> {
        Vector3::new(self.z, self.x.clone(), self.x)
    }

    /// Gets this [`Vector4`] as a [`Vector3`] with values `(z, x, z)`
    pub fn zxz(self) -> Vector3<T> {
        Vector3::new(self.z.clone(), self.x, self.z)
    }

    /// Gets this [`Vector4`] as a [`Vector3`] with values `(z, y, z)`
    pub fn zyz(self) -> Vector3<T> {
        Vector3::new(self.z.clone(), self.y, self.z)
    }

    /// Gets this [`Vector4`] as a [`Vector3`] with values `(z, y, y)`
    pub fn zyy(self) -> Vector3<T> {
        Vector3::new(self.z, self.y.clone(), self.y)
    }

    /// Gets this [`Vector4`] as a [`Vector3`] with values `(z, z, z)`
    pub fn zzz(self) -> Vector3<T> {
        Vector3::new(self.z.clone(), self.z.clone(), self.z)
    }

    /// Gets this [`Vector4`] as a [`Vector3`] with values `(z, z, x)`
    pub fn zzx(self) -> Vector3<T> {
        Vector3::new(self.z.clone(), self.z, self.x)
    }

    /// Gets this [`Vector4`] as a [`Vector3`] with values `(z, z, y)`
    pub fn zzy(self) -> Vector3<T> {
        Vector3::new(self.z.clone(), self.z, self.y)
    }

    /// Gets this [`Vector4`] as a [`Vector3`] with values `(z, z, w)`
    pub fn zzw(self) -> Vector3<T> {
        Vector3::new(self.z.clone(), self.z, self.w)
    }

    /// Gets this [`Vector4`] as a [`Vector3`] with values `(z, w, z)`
    pub fn zwz(self) -> Vector3<T> {
        Vector3::new(self.z.clone(), self.w, self.z)
    }

    /// Gets this [`Vector4`] as a [`Vector3`] with values `(z, w, w)`
    pub fn zww(self) -> Vector3<T> {
        Vector3::new(self.z, self.w.clone(), self.w)
    }

    /// Gets this [`Vector4`] as a [`Vector3`] with values `(w, x, x)`
    pub fn wxx(self) -> Vector3<T> {
        Vector3::new(self.w, self.x.clone(), self.x)
    }

    /// Gets this [`Vector4`] as a [`Vector3`] with values `(w, x, w)`
    pub fn wxw(self) -> Vector3<T> {
        Vector3::new(self.w.clone(), self.x, self.w)
    }

    /// Gets this [`Vector4`] as a [`Vector3`] with values `(w, y, y)`
    pub fn wyy(self) -> Vector3<T> {
        Vector3::new(self.w, self.y.clone(), self.y)
    }

    /// Gets this [`Vector4`] as a [`Vector3`] with values `(w, y, w)`
    pub fn wyw(self) -> Vector3<T> {
        Vector3::new(self.w.clone(), self.y, self.w)
    }

    /// Gets this [`Vector4`] as a [`Vector3`] with values `(w, z, z)`
    pub fn wzz(self) -> Vector3<T> {
        Vector3::new(self.w, self.z.clone(), self.z)
    }

    /// Gets this [`Vector4`] as a [`Vector3`] with values `(w, z, w)`
    pub fn wzw(self) -> Vector3<T> {
        Vector3::new(self.w.clone(), self.z, self.w)
    }

    /// Gets this [`Vector4`] as a [`Vector3`] with values `(w, w, w)`
    pub fn www(self) -> Vector3<T> {
        Vector3::new(self.w.clone(), self.w.clone(), self.w)
    }

    /// Gets this [`Vector4`] as a [`Vector3`] with values `(w, w, x)`
    pub fn wwx(self) -> Vector3<T> {
        Vector3::new(self.w.clone(), self.w, self.x)
    }

    /// Gets this [`Vector4`] as a [`Vector3`] with values `(w, w, y)`
    pub fn wwy(self) -> Vector3<T> {
        Vector3::new(self.w.clone(), self.w, self.y)
    }

    /// Gets this [`Vector4`] as a [`Vector3`] with values `(w, w, z)`
    pub fn wwz(self) -> Vector3<T> {
        Vector3::new(self.w.clone(), self.w, self.z)
    }

    /// Gets this [`Vector4`] as a [`Vector2`] with values `(x, x)`
    pub fn xx(self) -> Vector2<T> {
        Vector2::new(self.x.clone(), self.x)
    }

    /// Gets this [`Vector4`] as a [`Vector2`] with values `(y, y)`
    pub fn yy(self) -> Vector2<T> {
        Vector2::new(self.y.clone(), self.y)
    }

    /// Gets this [`Vector4`] as a [`Vector2`] with values `(z, z)`
    pub fn zz(self) -> Vector2<T> {
        Vector2::new(self.z.clone(), self.z)
    }

    /// Gets this [`Vector4`] as a [`Vector2`] with values `(w, w)`
    pub fn ww(self) -> Vector2<T> {
        Vector2::new(self.w.clone(), self.w)
    }
}
