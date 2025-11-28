use crate::math::{Quaternion, Vector3, number::One};
use std::ops::{Add, Mul, Sub};

impl<T: Add<Output = T> + Sub<Output = T> + Mul<Output = T> + One + Clone> Quaternion<T> {
    /// Get the right basis vector
    pub fn right(self) -> Vector3<T> {
        let two = T::ONE + T::ONE;

        let yy = self.y.clone() * self.y.clone();
        let zz = self.z.clone() * self.z.clone();
        let xy = self.x.clone() * self.y.clone();
        let xz = self.x * self.z.clone();
        let wy = self.w.clone() * self.y;
        let wz = self.w * self.z;

        Vector3::new(
            T::ONE - two.clone() * (yy.clone() + zz.clone()),
            two.clone() * (xy.clone() - wz.clone()),
            two.clone() * (xz.clone() + wy.clone()),
        )
    }

    /// Get the up basis vector
    pub fn up(self) -> Vector3<T> {
        let two = T::ONE + T::ONE;

        let xx = self.x.clone() * self.x.clone();
        let zz = self.z.clone() * self.z.clone();
        let xy = self.x.clone() * self.y.clone();
        let yz = self.y * self.z.clone();
        let wx = self.w.clone() * self.x;
        let wz = self.w * self.z;

        Vector3::new(
            two.clone() * (xy + wz),
            T::ONE - two.clone() * (xx.clone() + zz),
            two.clone() * (yz.clone() - wx.clone()),
        )
    }

    /// Get the forward basis vector
    pub fn forward(self) -> Vector3<T> {
        let two = T::ONE + T::ONE;

        let xx = self.x.clone() * self.x.clone();
        let yy = self.y.clone() * self.y.clone();
        let xz = self.x.clone() * self.z.clone();
        let yz = self.y.clone() * self.z;
        let wx = self.w.clone() * self.x;
        let wy = self.w * self.y;

        Vector3::new(
            two.clone() * (xz - wy),
            two.clone() * (yz + wx),
            T::ONE - two * (xx + yy),
        )
    }
}
