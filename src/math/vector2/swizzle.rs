use crate::math::{Vector2, Vector3, Vector4};

impl<T> Vector2<T> {
    /// Gets this [`Vector2`] as a [`Vector2`] with values `(x, y)`
    pub const fn xy(self) -> Vector2<T> {
        self
    }

    /// Gets this [`Vector2`] as a [`Vector2`] with values `(y, x)`
    pub fn yx(self) -> Vector2<T> {
        Vector2::new(self.y, self.x)
    }
}

impl<T: Clone> Vector2<T> {
    /// Gets this [`Vector2`] as a [`Vector2`] with values `(x, x)`
    pub fn xx(self) -> Vector2<T> {
        Vector2::new(self.x.clone(), self.x)
    }

    /// Gets this [`Vector2`] as a [`Vector2`] with values `(y, y)`
    pub fn yy(self) -> Vector2<T> {
        Vector2::new(self.y.clone(), self.y)
    }

    /// Gets this [`Vector2`] as a [`Vector3`] with values `(x, x, x)`
    pub fn xxx(self) -> Vector3<T> {
        Vector3::new(self.x.clone(), self.x.clone(), self.x)
    }

    /// Gets this [`Vector2`] as a [`Vector3`] with values `(x, x, y)`
    pub fn xxy(self) -> Vector3<T> {
        Vector3::new(self.x.clone(), self.x, self.y)
    }

    /// Gets this [`Vector2`] as a [`Vector3`] with values `(x, y, x)`
    pub fn xyx(self) -> Vector3<T> {
        Vector3::new(self.x.clone(), self.y, self.x)
    }

    /// Gets this [`Vector2`] as a [`Vector3`] with values `(x, y, y)`
    pub fn xyy(self) -> Vector3<T> {
        Vector3::new(self.x, self.y.clone(), self.y)
    }

    /// Gets this [`Vector2`] as a [`Vector3`] with values `(y, x, x)`
    pub fn yxx(self) -> Vector3<T> {
        Vector3::new(self.y, self.x.clone(), self.x)
    }

    /// Gets this [`Vector2`] as a [`Vector3`] with values `(y, x, y)`
    pub fn yxy(self) -> Vector3<T> {
        Vector3::new(self.y.clone(), self.x, self.y)
    }

    /// Gets this [`Vector2`] as a [`Vector3`] with values `(y, y, x)`
    pub fn yyx(self) -> Vector3<T> {
        Vector3::new(self.y.clone(), self.y, self.x)
    }

    /// Gets this [`Vector2`] as a [`Vector3`] with values `(y, y, y)`
    pub fn yyy(self) -> Vector3<T> {
        Vector3::new(self.y.clone(), self.y.clone(), self.y)
    }

    /// Gets this [`Vector2`] as a [`Vector4`] with values `(x, x, x, x)`
    pub fn xxxx(self) -> Vector4<T> {
        Vector4::new(self.x.clone(), self.x.clone(), self.x.clone(), self.x)
    }

    /// Gets this [`Vector2`] as a [`Vector4`] with values `(x, x, x, y)`
    pub fn xxxy(self) -> Vector4<T> {
        Vector4::new(self.x.clone(), self.x.clone(), self.x.clone(), self.y)
    }

    /// Gets this [`Vector2`] as a [`Vector4`] with values `(x, x, y, x)`
    pub fn xxyx(self) -> Vector4<T> {
        Vector4::new(self.x.clone(), self.x.clone(), self.y.clone(), self.x)
    }

    /// Gets this [`Vector2`] as a [`Vector4`] with values `(x, x, y, y)`
    pub fn xxyy(self) -> Vector4<T> {
        Vector4::new(self.x.clone(), self.x.clone(), self.y.clone(), self.y)
    }

    /// Gets this [`Vector2`] as a [`Vector4`] with values `(x, y, x, x)`
    pub fn xyxx(self) -> Vector4<T> {
        Vector4::new(self.x.clone(), self.y.clone(), self.x.clone(), self.x)
    }

    /// Gets this [`Vector2`] as a [`Vector4`] with values `(x, y, x, y)`
    pub fn xyxy(self) -> Vector4<T> {
        Vector4::new(self.x.clone(), self.y.clone(), self.x.clone(), self.y)
    }

    /// Gets this [`Vector2`] as a [`Vector4`] with values `(x, y, y, x)`
    pub fn xyyx(self) -> Vector4<T> {
        Vector4::new(self.x.clone(), self.y.clone(), self.y.clone(), self.x)
    }

    /// Gets this [`Vector2`] as a [`Vector4`] with values `(x, y, y, y)`
    pub fn xyyy(self) -> Vector4<T> {
        Vector4::new(self.x.clone(), self.y.clone(), self.y.clone(), self.y)
    }

    /// Gets this [`Vector2`] as a [`Vector4`] with values `(y, x, x, x)`
    pub fn yxxx(self) -> Vector4<T> {
        Vector4::new(self.y.clone(), self.x.clone(), self.x.clone(), self.x)
    }

    /// Gets this [`Vector2`] as a [`Vector4`] with values `(y, x, x, y)`
    pub fn yxxy(self) -> Vector4<T> {
        Vector4::new(self.y.clone(), self.x.clone(), self.x.clone(), self.y)
    }

    /// Gets this [`Vector2`] as a [`Vector4`] with values `(y, x, y, x)`
    pub fn yxyx(self) -> Vector4<T> {
        Vector4::new(self.y.clone(), self.x.clone(), self.y.clone(), self.x)
    }

    /// Gets this [`Vector2`] as a [`Vector4`] with values `(y, x, y, y)`
    pub fn yxyy(self) -> Vector4<T> {
        Vector4::new(self.y.clone(), self.x.clone(), self.y.clone(), self.y)
    }

    /// Gets this [`Vector2`] as a [`Vector4`] with values `(y, y, x, x)`
    pub fn yyxx(self) -> Vector4<T> {
        Vector4::new(self.y.clone(), self.y.clone(), self.x.clone(), self.x)
    }

    /// Gets this [`Vector2`] as a [`Vector4`] with values `(y, y, x, y)`
    pub fn yyxy(self) -> Vector4<T> {
        Vector4::new(self.y.clone(), self.y.clone(), self.x.clone(), self.y)
    }

    /// Gets this [`Vector2`] as a [`Vector4`] with values `(y, y, y, x)`
    pub fn yyyx(self) -> Vector4<T> {
        Vector4::new(self.y.clone(), self.y.clone(), self.y.clone(), self.x)
    }

    /// Gets this [`Vector2`] as a [`Vector4`] with values `(y, y, y, y)`
    pub fn yyyy(self) -> Vector4<T> {
        Vector4::new(self.y.clone(), self.y.clone(), self.y.clone(), self.y)
    }
}
