use crate::math::{Vector2, Vector3, Vector4};

impl<T> Vector3<T> {
    /// Gets this [`Vector3`] as a [`Vector2`] with values `(x, y)`
    pub fn xy(self) -> Vector2<T> {
        Vector2::new(self.x, self.y)
    }

    /// Gets this [`Vector3`] as a [`Vector2`] with values `(x, z)`
    pub fn xz(self) -> Vector2<T> {
        Vector2::new(self.x, self.z)
    }

    /// Gets this [`Vector3`] as a [`Vector2`] with values `(y, x)`
    pub fn yx(self) -> Vector2<T> {
        Vector2::new(self.y, self.x)
    }

    /// Gets this [`Vector3`] as a [`Vector2`] with values `(y, z)`
    pub fn yz(self) -> Vector2<T> {
        Vector2::new(self.y, self.z)
    }

    /// Gets this [`Vector3`] as a [`Vector2`] with values `(z, x)`
    pub fn zx(self) -> Vector2<T> {
        Vector2::new(self.z, self.x)
    }

    /// Gets this [`Vector3`] as a [`Vector2`] with values `(z, y)`
    pub fn zy(self) -> Vector2<T> {
        Vector2::new(self.z, self.y)
    }

    /// Gets this [`Vector3`] as a [`Vector3`] with values `(x, y, z)`
    pub const fn xyz(self) -> Vector3<T> {
        self
    }

    /// Gets this [`Vector3`] as a [`Vector3`] with values `(x, z, y)`
    pub fn xzy(self) -> Vector3<T> {
        Vector3::new(self.x, self.z, self.y)
    }

    /// Gets this [`Vector3`] as a [`Vector3`] with values `(y, x, z)`
    pub fn yxz(self) -> Vector3<T> {
        Vector3::new(self.y, self.x, self.z)
    }

    /// Gets this [`Vector3`] as a [`Vector3`] with values `(y, z, x)`
    pub fn yzx(self) -> Vector3<T> {
        Vector3::new(self.y, self.z, self.x)
    }

    /// Gets this [`Vector3`] as a [`Vector3`] with values `(z, x, y)`
    pub fn zxy(self) -> Vector3<T> {
        Vector3::new(self.z, self.x, self.y)
    }

    /// Gets this [`Vector3`] as a [`Vector3`] with values `(z, y, x)`
    pub fn zyx(self) -> Vector3<T> {
        Vector3::new(self.z, self.y, self.x)
    }
}

impl<T: Clone> Vector3<T> {
    /// Gets this [`Vector3`] as a [`Vector2`] with values `(x, x)`
    pub fn xx(self) -> Vector2<T> {
        Vector2::new(self.x.clone(), self.x)
    }

    /// Gets this [`Vector3`] as a [`Vector2`] with values `(y, y)`
    pub fn yy(self) -> Vector2<T> {
        Vector2::new(self.y.clone(), self.y)
    }

    /// Gets this [`Vector3`] as a [`Vector2`] with values `(z, z)`
    pub fn zz(self) -> Vector2<T> {
        Vector2::new(self.z.clone(), self.z)
    }

    /// Gets this [`Vector3`] as a [`Vector3`] with values `(x, x, x)`
    pub fn xxx(self) -> Vector3<T> {
        Vector3::new(self.x.clone(), self.x.clone(), self.x)
    }

    /// Gets this [`Vector3`] as a [`Vector3`] with values `(x, x, y)`
    pub fn xxy(self) -> Vector3<T> {
        Vector3::new(self.x.clone(), self.x, self.y)
    }

    /// Gets this [`Vector3`] as a [`Vector3`] with values `(x, x, z)`
    pub fn xxz(self) -> Vector3<T> {
        Vector3::new(self.x.clone(), self.x, self.z)
    }

    /// Gets this [`Vector3`] as a [`Vector3`] with values `(x, y, x)`
    pub fn xyx(self) -> Vector3<T> {
        Vector3::new(self.x.clone(), self.y, self.x)
    }

    /// Gets this [`Vector3`] as a [`Vector3`] with values `(x, y, y)`
    pub fn xyy(self) -> Vector3<T> {
        Vector3::new(self.x, self.y.clone(), self.y)
    }

    /// Gets this [`Vector3`] as a [`Vector3`] with values `(x, y, z)`
    /// (alias of [`Self::xyz`])
    pub fn xyz_clone(self) -> Vector3<T> {
        // included only for symmetry; prefer `xyz()` which is `const` and clone-free
        Vector3::new(self.x, self.y, self.z)
    }

    /// Gets this [`Vector3`] as a [`Vector3`] with values `(x, z, x)`
    pub fn xzx(self) -> Vector3<T> {
        Vector3::new(self.x.clone(), self.z, self.x)
    }

    /// Gets this [`Vector3`] as a [`Vector3`] with values `(x, z, z)`
    pub fn xzz(self) -> Vector3<T> {
        Vector3::new(self.x, self.z.clone(), self.z)
    }

    /// Gets this [`Vector3`] as a [`Vector3`] with values `(y, x, x)`
    pub fn yxx(self) -> Vector3<T> {
        Vector3::new(self.y, self.x.clone(), self.x)
    }

    /// Gets this [`Vector3`] as a [`Vector3`] with values `(y, x, y)`
    pub fn yxy(self) -> Vector3<T> {
        Vector3::new(self.y.clone(), self.x, self.y)
    }

    /// Gets this [`Vector3`] as a [`Vector3`] with values `(y, x, z)`
    pub fn yxz_clone(self) -> Vector3<T> {
        // alias of `yxz()`; provided only for completeness
        Vector3::new(self.y, self.x, self.z)
    }

    /// Gets this [`Vector3`] as a [`Vector3`] with values `(y, y, x)`
    pub fn yyx(self) -> Vector3<T> {
        Vector3::new(self.y.clone(), self.y, self.x)
    }

    /// Gets this [`Vector3`] as a [`Vector3`] with values `(y, y, y)`
    pub fn yyy(self) -> Vector3<T> {
        Vector3::new(self.y.clone(), self.y.clone(), self.y)
    }

    /// Gets this [`Vector3`] as a [`Vector3`] with values `(y, y, z)`
    pub fn yyz(self) -> Vector3<T> {
        Vector3::new(self.y.clone(), self.y, self.z)
    }

    /// Gets this [`Vector3`] as a [`Vector3`] with values `(y, z, x)`
    pub fn yzx_clone(self) -> Vector3<T> {
        // alias of `yzx()`; provided only for completeness
        Vector3::new(self.y, self.z, self.x)
    }

    /// Gets this [`Vector3`] as a [`Vector3`] with values `(y, z, y)`
    pub fn yzy(self) -> Vector3<T> {
        Vector3::new(self.y.clone(), self.z, self.y)
    }

    /// Gets this [`Vector3`] as a [`Vector3`] with values `(y, z, z)`
    pub fn yzz(self) -> Vector3<T> {
        Vector3::new(self.y, self.z.clone(), self.z)
    }

    /// Gets this [`Vector3`] as a [`Vector3`] with values `(z, x, x)`
    pub fn zxx(self) -> Vector3<T> {
        Vector3::new(self.z, self.x.clone(), self.x)
    }

    /// Gets this [`Vector3`] as a [`Vector3`] with values `(z, x, y)`
    pub fn zxy_clone(self) -> Vector3<T> {
        // alias of `zxy()`; provided only for completeness
        Vector3::new(self.z, self.x, self.y)
    }

    /// Gets this [`Vector3`] as a [`Vector3`] with values `(z, x, z)`
    pub fn zxz(self) -> Vector3<T> {
        Vector3::new(self.z.clone(), self.x, self.z)
    }

    /// Gets this [`Vector3`] as a [`Vector3`] with values `(z, y, x)`
    pub fn zyx_clone(self) -> Vector3<T> {
        // alias of `zyx()`; provided only for completeness
        Vector3::new(self.z, self.y, self.x)
    }

    /// Gets this [`Vector3`] as a [`Vector3`] with values `(z, y, y)`
    pub fn zyy(self) -> Vector3<T> {
        Vector3::new(self.z, self.y.clone(), self.y)
    }

    /// Gets this [`Vector3`] as a [`Vector3`] with values `(z, y, z)`
    pub fn zyz(self) -> Vector3<T> {
        Vector3::new(self.z.clone(), self.y, self.z)
    }

    /// Gets this [`Vector3`] as a [`Vector3`] with values `(z, z, x)`
    pub fn zzx(self) -> Vector3<T> {
        Vector3::new(self.z.clone(), self.z, self.x)
    }

    /// Gets this [`Vector3`] as a [`Vector3`] with values `(z, z, y)`
    pub fn zzy(self) -> Vector3<T> {
        Vector3::new(self.z.clone(), self.z, self.y)
    }

    /// Gets this [`Vector3`] as a [`Vector3`] with values `(z, z, z)`
    pub fn zzz(self) -> Vector3<T> {
        Vector3::new(self.z.clone(), self.z.clone(), self.z)
    }

    /// Gets this [`Vector3`] as a [`Vector4`] with values `(x, x, x, x)`
    pub fn xxxx(self) -> Vector4<T> {
        Vector4::new(
            self.x.clone(),
            self.x.clone(),
            self.x.clone(),
            self.x.clone(),
        )
    }

    /// Gets this [`Vector3`] as a [`Vector4`] with values `(x, x, x, y)`
    pub fn xxxy(self) -> Vector4<T> {
        Vector4::new(
            self.x.clone(),
            self.x.clone(),
            self.x.clone(),
            self.y.clone(),
        )
    }

    /// Gets this [`Vector3`] as a [`Vector4`] with values `(x, x, x, z)`
    pub fn xxxz(self) -> Vector4<T> {
        Vector4::new(
            self.x.clone(),
            self.x.clone(),
            self.x.clone(),
            self.z.clone(),
        )
    }

    /// Gets this [`Vector3`] as a [`Vector4`] with values `(x, x, y, x)`
    pub fn xxyx(self) -> Vector4<T> {
        Vector4::new(
            self.x.clone(),
            self.x.clone(),
            self.y.clone(),
            self.x.clone(),
        )
    }

    /// Gets this [`Vector3`] as a [`Vector4`] with values `(x, x, y, y)`
    pub fn xxyy(self) -> Vector4<T> {
        Vector4::new(
            self.x.clone(),
            self.x.clone(),
            self.y.clone(),
            self.y.clone(),
        )
    }

    /// Gets this [`Vector3`] as a [`Vector4`] with values `(x, x, y, z)`
    pub fn xxyz(self) -> Vector4<T> {
        Vector4::new(
            self.x.clone(),
            self.x.clone(),
            self.y.clone(),
            self.z.clone(),
        )
    }

    /// Gets this [`Vector3`] as a [`Vector4`] with values `(x, x, z, x)`
    pub fn xxzx(self) -> Vector4<T> {
        Vector4::new(
            self.x.clone(),
            self.x.clone(),
            self.z.clone(),
            self.x.clone(),
        )
    }

    /// Gets this [`Vector3`] as a [`Vector4`] with values `(x, x, z, y)`
    pub fn xxzy(self) -> Vector4<T> {
        Vector4::new(
            self.x.clone(),
            self.x.clone(),
            self.z.clone(),
            self.y.clone(),
        )
    }

    /// Gets this [`Vector3`] as a [`Vector4`] with values `(x, x, z, z)`
    pub fn xxzz(self) -> Vector4<T> {
        Vector4::new(
            self.x.clone(),
            self.x.clone(),
            self.z.clone(),
            self.z.clone(),
        )
    }

    /// Gets this [`Vector3`] as a [`Vector4`] with values `(x, y, x, x)`
    pub fn xyxx(self) -> Vector4<T> {
        Vector4::new(
            self.x.clone(),
            self.y.clone(),
            self.x.clone(),
            self.x.clone(),
        )
    }

    /// Gets this [`Vector3`] as a [`Vector4`] with values `(x, y, x, y)`
    pub fn xyxy(self) -> Vector4<T> {
        Vector4::new(
            self.x.clone(),
            self.y.clone(),
            self.x.clone(),
            self.y.clone(),
        )
    }

    /// Gets this [`Vector3`] as a [`Vector4`] with values `(x, y, x, z)`
    pub fn xyxz(self) -> Vector4<T> {
        Vector4::new(
            self.x.clone(),
            self.y.clone(),
            self.x.clone(),
            self.z.clone(),
        )
    }

    /// Gets this [`Vector3`] as a [`Vector4`] with values `(x, y, y, x)`
    pub fn xyyx(self) -> Vector4<T> {
        Vector4::new(
            self.x.clone(),
            self.y.clone(),
            self.y.clone(),
            self.x.clone(),
        )
    }

    /// Gets this [`Vector3`] as a [`Vector4`] with values `(x, y, y, y)`
    pub fn xyyy(self) -> Vector4<T> {
        Vector4::new(
            self.x.clone(),
            self.y.clone(),
            self.y.clone(),
            self.y.clone(),
        )
    }

    /// Gets this [`Vector3`] as a [`Vector4`] with values `(x, y, y, z)`
    pub fn xyyz(self) -> Vector4<T> {
        Vector4::new(
            self.x.clone(),
            self.y.clone(),
            self.y.clone(),
            self.z.clone(),
        )
    }

    /// Gets this [`Vector3`] as a [`Vector4`] with values `(x, y, z, x)`
    pub fn xyzx(self) -> Vector4<T> {
        Vector4::new(
            self.x.clone(),
            self.y.clone(),
            self.z.clone(),
            self.x.clone(),
        )
    }

    /// Gets this [`Vector3`] as a [`Vector4`] with values `(x, y, z, y)`
    pub fn xyzy(self) -> Vector4<T> {
        Vector4::new(
            self.x.clone(),
            self.y.clone(),
            self.z.clone(),
            self.y.clone(),
        )
    }

    /// Gets this [`Vector3`] as a [`Vector4`] with values `(x, y, z, z)`
    pub fn xyzz(self) -> Vector4<T> {
        Vector4::new(
            self.x.clone(),
            self.y.clone(),
            self.z.clone(),
            self.z.clone(),
        )
    }

    /// Gets this [`Vector3`] as a [`Vector4`] with values `(x, z, x, x)`
    pub fn xzxx(self) -> Vector4<T> {
        Vector4::new(
            self.x.clone(),
            self.z.clone(),
            self.x.clone(),
            self.x.clone(),
        )
    }

    /// Gets this [`Vector3`] as a [`Vector4`] with values `(x, z, x, y)`
    pub fn xzxy(self) -> Vector4<T> {
        Vector4::new(
            self.x.clone(),
            self.z.clone(),
            self.x.clone(),
            self.y.clone(),
        )
    }

    /// Gets this [`Vector3`] as a [`Vector4`] with values `(x, z, x, z)`
    pub fn xzxz(self) -> Vector4<T> {
        Vector4::new(
            self.x.clone(),
            self.z.clone(),
            self.x.clone(),
            self.z.clone(),
        )
    }

    /// Gets this [`Vector3`] as a [`Vector4`] with values `(x, z, y, x)`
    pub fn xzyx(self) -> Vector4<T> {
        Vector4::new(
            self.x.clone(),
            self.z.clone(),
            self.y.clone(),
            self.x.clone(),
        )
    }

    /// Gets this [`Vector3`] as a [`Vector4`] with values `(x, z, y, y)`
    pub fn xzyy(self) -> Vector4<T> {
        Vector4::new(
            self.x.clone(),
            self.z.clone(),
            self.y.clone(),
            self.y.clone(),
        )
    }

    /// Gets this [`Vector3`] as a [`Vector4`] with values `(x, z, y, z)`
    pub fn xzyz(self) -> Vector4<T> {
        Vector4::new(
            self.x.clone(),
            self.z.clone(),
            self.y.clone(),
            self.z.clone(),
        )
    }

    /// Gets this [`Vector3`] as a [`Vector4`] with values `(x, z, z, x)`
    pub fn xzzx(self) -> Vector4<T> {
        Vector4::new(
            self.x.clone(),
            self.z.clone(),
            self.z.clone(),
            self.x.clone(),
        )
    }

    /// Gets this [`Vector3`] as a [`Vector4`] with values `(x, z, z, y)`
    pub fn xzzy(self) -> Vector4<T> {
        Vector4::new(
            self.x.clone(),
            self.z.clone(),
            self.z.clone(),
            self.y.clone(),
        )
    }

    /// Gets this [`Vector3`] as a [`Vector4`] with values `(x, z, z, z)`
    pub fn xzzz(self) -> Vector4<T> {
        Vector4::new(
            self.x.clone(),
            self.z.clone(),
            self.z.clone(),
            self.z.clone(),
        )
    }

    /// Gets this [`Vector3`] as a [`Vector4`] with values `(y, x, x, x)`
    pub fn yxxx(self) -> Vector4<T> {
        Vector4::new(
            self.y.clone(),
            self.x.clone(),
            self.x.clone(),
            self.x.clone(),
        )
    }

    /// Gets this [`Vector3`] as a [`Vector4`] with values `(y, x, x, y)`
    pub fn yxxy(self) -> Vector4<T> {
        Vector4::new(
            self.y.clone(),
            self.x.clone(),
            self.x.clone(),
            self.y.clone(),
        )
    }

    /// Gets this [`Vector3`] as a [`Vector4`] with values `(y, x, x, z)`
    pub fn yxxz(self) -> Vector4<T> {
        Vector4::new(
            self.y.clone(),
            self.x.clone(),
            self.x.clone(),
            self.z.clone(),
        )
    }

    /// Gets this [`Vector3`] as a [`Vector4`] with values `(y, x, y, x)`
    pub fn yxyx(self) -> Vector4<T> {
        Vector4::new(
            self.y.clone(),
            self.x.clone(),
            self.y.clone(),
            self.x.clone(),
        )
    }

    /// Gets this [`Vector3`] as a [`Vector4`] with values `(y, x, y, y)`
    pub fn yxyy(self) -> Vector4<T> {
        Vector4::new(
            self.y.clone(),
            self.x.clone(),
            self.y.clone(),
            self.y.clone(),
        )
    }

    /// Gets this [`Vector3`] as a [`Vector4`] with values `(y, x, y, z)`
    pub fn yxyz(self) -> Vector4<T> {
        Vector4::new(
            self.y.clone(),
            self.x.clone(),
            self.y.clone(),
            self.z.clone(),
        )
    }

    /// Gets this [`Vector3`] as a [`Vector4`] with values `(y, x, z, x)`
    pub fn yxzx(self) -> Vector4<T> {
        Vector4::new(
            self.y.clone(),
            self.x.clone(),
            self.z.clone(),
            self.x.clone(),
        )
    }

    /// Gets this [`Vector3`] as a [`Vector4`] with values `(y, x, z, y)`
    pub fn yxzy(self) -> Vector4<T> {
        Vector4::new(
            self.y.clone(),
            self.x.clone(),
            self.z.clone(),
            self.y.clone(),
        )
    }

    /// Gets this [`Vector3`] as a [`Vector4`] with values `(y, x, z, z)`
    pub fn yxzz(self) -> Vector4<T> {
        Vector4::new(
            self.y.clone(),
            self.x.clone(),
            self.z.clone(),
            self.z.clone(),
        )
    }

    /// Gets this [`Vector3`] as a [`Vector4`] with values `(y, y, x, x)`
    pub fn yyxx(self) -> Vector4<T> {
        Vector4::new(
            self.y.clone(),
            self.y.clone(),
            self.x.clone(),
            self.x.clone(),
        )
    }

    /// Gets this [`Vector3`] as a [`Vector4`] with values `(y, y, x, y)`
    pub fn yyxy(self) -> Vector4<T> {
        Vector4::new(
            self.y.clone(),
            self.y.clone(),
            self.x.clone(),
            self.y.clone(),
        )
    }

    /// Gets this [`Vector3`] as a [`Vector4`] with values `(y, y, x, z)`
    pub fn yyxz(self) -> Vector4<T> {
        Vector4::new(
            self.y.clone(),
            self.y.clone(),
            self.x.clone(),
            self.z.clone(),
        )
    }

    /// Gets this [`Vector3`] as a [`Vector4`] with values `(y, y, y, x)`
    pub fn yyyx(self) -> Vector4<T> {
        Vector4::new(
            self.y.clone(),
            self.y.clone(),
            self.y.clone(),
            self.x.clone(),
        )
    }

    /// Gets this [`Vector3`] as a [`Vector4`] with values `(y, y, y, y)`
    pub fn yyyy(self) -> Vector4<T> {
        Vector4::new(
            self.y.clone(),
            self.y.clone(),
            self.y.clone(),
            self.y.clone(),
        )
    }

    /// Gets this [`Vector3`] as a [`Vector4`] with values `(y, y, y, z)`
    pub fn yyyz(self) -> Vector4<T> {
        Vector4::new(
            self.y.clone(),
            self.y.clone(),
            self.y.clone(),
            self.z.clone(),
        )
    }

    /// Gets this [`Vector3`] as a [`Vector4`] with values `(y, y, z, x)`
    pub fn yyzx(self) -> Vector4<T> {
        Vector4::new(
            self.y.clone(),
            self.y.clone(),
            self.z.clone(),
            self.x.clone(),
        )
    }

    /// Gets this [`Vector3`] as a [`Vector4`] with values `(y, y, z, y)`
    pub fn yyzy(self) -> Vector4<T> {
        Vector4::new(
            self.y.clone(),
            self.y.clone(),
            self.z.clone(),
            self.y.clone(),
        )
    }

    /// Gets this [`Vector3`] as a [`Vector4`] with values `(y, y, z, z)`
    pub fn yyzz(self) -> Vector4<T> {
        Vector4::new(
            self.y.clone(),
            self.y.clone(),
            self.z.clone(),
            self.z.clone(),
        )
    }

    /// Gets this [`Vector3`] as a [`Vector4`] with values `(y, z, x, x)`
    pub fn yzxx(self) -> Vector4<T> {
        Vector4::new(
            self.y.clone(),
            self.z.clone(),
            self.x.clone(),
            self.x.clone(),
        )
    }

    /// Gets this [`Vector3`] as a [`Vector4`] with values `(y, z, x, y)`
    pub fn yzxy(self) -> Vector4<T> {
        Vector4::new(
            self.y.clone(),
            self.z.clone(),
            self.x.clone(),
            self.y.clone(),
        )
    }

    /// Gets this [`Vector3`] as a [`Vector4`] with values `(y, z, x, z)`
    pub fn yzxz(self) -> Vector4<T> {
        Vector4::new(
            self.y.clone(),
            self.z.clone(),
            self.x.clone(),
            self.z.clone(),
        )
    }

    /// Gets this [`Vector3`] as a [`Vector4`] with values `(y, z, y, x)`
    pub fn yzyx(self) -> Vector4<T> {
        Vector4::new(
            self.y.clone(),
            self.z.clone(),
            self.y.clone(),
            self.x.clone(),
        )
    }

    /// Gets this [`Vector3`] as a [`Vector4`] with values `(y, z, y, y)`
    pub fn yzyy(self) -> Vector4<T> {
        Vector4::new(
            self.y.clone(),
            self.z.clone(),
            self.y.clone(),
            self.y.clone(),
        )
    }

    /// Gets this [`Vector3`] as a [`Vector4`] with values `(y, z, y, z)`
    pub fn yzyz(self) -> Vector4<T> {
        Vector4::new(
            self.y.clone(),
            self.z.clone(),
            self.y.clone(),
            self.z.clone(),
        )
    }

    /// Gets this [`Vector3`] as a [`Vector4`] with values `(y, z, z, x)`
    pub fn yzzx(self) -> Vector4<T> {
        Vector4::new(
            self.y.clone(),
            self.z.clone(),
            self.z.clone(),
            self.x.clone(),
        )
    }

    /// Gets this [`Vector3`] as a [`Vector4`] with values `(y, z, z, y)`
    pub fn yzzy(self) -> Vector4<T> {
        Vector4::new(
            self.y.clone(),
            self.z.clone(),
            self.z.clone(),
            self.y.clone(),
        )
    }

    /// Gets this [`Vector3`] as a [`Vector4`] with values `(y, z, z, z)`
    pub fn yzzz(self) -> Vector4<T> {
        Vector4::new(
            self.y.clone(),
            self.z.clone(),
            self.z.clone(),
            self.z.clone(),
        )
    }

    /// Gets this [`Vector3`] as a [`Vector4`] with values `(z, x, x, x)`
    pub fn zxxx(self) -> Vector4<T> {
        Vector4::new(
            self.z.clone(),
            self.x.clone(),
            self.x.clone(),
            self.x.clone(),
        )
    }

    /// Gets this [`Vector3`] as a [`Vector4`] with values `(z, x, x, y)`
    pub fn zxxy(self) -> Vector4<T> {
        Vector4::new(
            self.z.clone(),
            self.x.clone(),
            self.x.clone(),
            self.y.clone(),
        )
    }

    /// Gets this [`Vector3`] as a [`Vector4`] with values `(z, x, x, z)`
    pub fn zxxz(self) -> Vector4<T> {
        Vector4::new(
            self.z.clone(),
            self.x.clone(),
            self.x.clone(),
            self.z.clone(),
        )
    }

    /// Gets this [`Vector3`] as a [`Vector4`] with values `(z, x, y, x)`
    pub fn zxyx(self) -> Vector4<T> {
        Vector4::new(
            self.z.clone(),
            self.x.clone(),
            self.y.clone(),
            self.x.clone(),
        )
    }

    /// Gets this [`Vector3`] as a [`Vector4`] with values `(z, x, y, y)`
    pub fn zxyy(self) -> Vector4<T> {
        Vector4::new(
            self.z.clone(),
            self.x.clone(),
            self.y.clone(),
            self.y.clone(),
        )
    }

    /// Gets this [`Vector3`] as a [`Vector4`] with values `(z, x, y, z)`
    pub fn zxyz(self) -> Vector4<T> {
        Vector4::new(
            self.z.clone(),
            self.x.clone(),
            self.y.clone(),
            self.z.clone(),
        )
    }

    /// Gets this [`Vector3`] as a [`Vector4`] with values `(z, x, z, x)`
    pub fn zxzx(self) -> Vector4<T> {
        Vector4::new(
            self.z.clone(),
            self.x.clone(),
            self.z.clone(),
            self.x.clone(),
        )
    }

    /// Gets this [`Vector3`] as a [`Vector4`] with values `(z, x, z, y)`
    pub fn zxzy(self) -> Vector4<T> {
        Vector4::new(
            self.z.clone(),
            self.x.clone(),
            self.z.clone(),
            self.y.clone(),
        )
    }

    /// Gets this [`Vector3`] as a [`Vector4`] with values `(z, x, z, z)`
    pub fn zxzz(self) -> Vector4<T> {
        Vector4::new(
            self.z.clone(),
            self.x.clone(),
            self.z.clone(),
            self.z.clone(),
        )
    }

    /// Gets this [`Vector3`] as a [`Vector4`] with values `(z, y, x, x)`
    pub fn zyxx(self) -> Vector4<T> {
        Vector4::new(
            self.z.clone(),
            self.y.clone(),
            self.x.clone(),
            self.x.clone(),
        )
    }

    /// Gets this [`Vector3`] as a [`Vector4`] with values `(z, y, x, y)`
    pub fn zyxy(self) -> Vector4<T> {
        Vector4::new(
            self.z.clone(),
            self.y.clone(),
            self.x.clone(),
            self.y.clone(),
        )
    }

    /// Gets this [`Vector3`] as a [`Vector4`] with values `(z, y, x, z)`
    pub fn zyxz(self) -> Vector4<T> {
        Vector4::new(
            self.z.clone(),
            self.y.clone(),
            self.x.clone(),
            self.z.clone(),
        )
    }

    /// Gets this [`Vector3`] as a [`Vector4`] with values `(z, y, y, x)`
    pub fn zyyx(self) -> Vector4<T> {
        Vector4::new(
            self.z.clone(),
            self.y.clone(),
            self.y.clone(),
            self.x.clone(),
        )
    }

    /// Gets this [`Vector3`] as a [`Vector4`] with values `(z, y, y, y)`
    pub fn zyyy(self) -> Vector4<T> {
        Vector4::new(
            self.z.clone(),
            self.y.clone(),
            self.y.clone(),
            self.y.clone(),
        )
    }

    /// Gets this [`Vector3`] as a [`Vector4`] with values `(z, y, y, z)`
    pub fn zyyz(self) -> Vector4<T> {
        Vector4::new(
            self.z.clone(),
            self.y.clone(),
            self.y.clone(),
            self.z.clone(),
        )
    }

    /// Gets this [`Vector3`] as a [`Vector4`] with values `(z, y, z, x)`
    pub fn zyzx(self) -> Vector4<T> {
        Vector4::new(
            self.z.clone(),
            self.y.clone(),
            self.z.clone(),
            self.x.clone(),
        )
    }

    /// Gets this [`Vector3`] as a [`Vector4`] with values `(z, y, z, y)`
    pub fn zyzy(self) -> Vector4<T> {
        Vector4::new(
            self.z.clone(),
            self.y.clone(),
            self.z.clone(),
            self.y.clone(),
        )
    }

    /// Gets this [`Vector3`] as a [`Vector4`] with values `(z, y, z, z)`
    pub fn zyzz(self) -> Vector4<T> {
        Vector4::new(
            self.z.clone(),
            self.y.clone(),
            self.z.clone(),
            self.z.clone(),
        )
    }

    /// Gets this [`Vector3`] as a [`Vector4`] with values `(z, z, x, x)`
    pub fn zzxx(self) -> Vector4<T> {
        Vector4::new(
            self.z.clone(),
            self.z.clone(),
            self.x.clone(),
            self.x.clone(),
        )
    }

    /// Gets this [`Vector3`] as a [`Vector4`] with values `(z, z, x, y)`
    pub fn zzxy(self) -> Vector4<T> {
        Vector4::new(
            self.z.clone(),
            self.z.clone(),
            self.x.clone(),
            self.y.clone(),
        )
    }

    /// Gets this [`Vector3`] as a [`Vector4`] with values `(z, z, x, z)`
    pub fn zzxz(self) -> Vector4<T> {
        Vector4::new(
            self.z.clone(),
            self.z.clone(),
            self.x.clone(),
            self.z.clone(),
        )
    }

    /// Gets this [`Vector3`] as a [`Vector4`] with values `(z, z, y, x)`
    pub fn zzyx(self) -> Vector4<T> {
        Vector4::new(
            self.z.clone(),
            self.z.clone(),
            self.y.clone(),
            self.x.clone(),
        )
    }

    /// Gets this [`Vector3`] as a [`Vector4`] with values `(z, z, y, y)`
    pub fn zzyy(self) -> Vector4<T> {
        Vector4::new(
            self.z.clone(),
            self.z.clone(),
            self.y.clone(),
            self.y.clone(),
        )
    }

    /// Gets this [`Vector3`] as a [`Vector4`] with values `(z, z, y, z)`
    pub fn zzyz(self) -> Vector4<T> {
        Vector4::new(
            self.z.clone(),
            self.z.clone(),
            self.y.clone(),
            self.z.clone(),
        )
    }

    /// Gets this [`Vector3`] as a [`Vector4`] with values `(z, z, z, x)`
    pub fn zzzx(self) -> Vector4<T> {
        Vector4::new(
            self.z.clone(),
            self.z.clone(),
            self.z.clone(),
            self.x.clone(),
        )
    }

    /// Gets this [`Vector3`] as a [`Vector4`] with values `(z, z, z, y)`
    pub fn zzzy(self) -> Vector4<T> {
        Vector4::new(
            self.z.clone(),
            self.z.clone(),
            self.z.clone(),
            self.y.clone(),
        )
    }

    /// Gets this [`Vector3`] as a [`Vector4`] with values `(z, z, z, z)`
    pub fn zzzz(self) -> Vector4<T> {
        Vector4::new(
            self.z.clone(),
            self.z.clone(),
            self.z.clone(),
            self.z.clone(),
        )
    }
}
