use cg::prelude::*;

use prelude::*;

#[derive(Copy, Clone, Debug, From, Into, Shrinkwrap)]
#[shrinkwrap(mutable, unsafe_ignore_visibility)]
pub struct Normal(Vector3f);

impl Normal {
    pub fn zero() -> Self {
        Normal(Vector3f::zero())
    }

    #[inline(always)]
    pub fn face_forward(self, v: impl Into<Vector3f>) -> Self {
        let n: Vector3f = self.into();
        let v: Vector3f = v.into();

        if n.dot(v) < 0.0 {
            (-n).into()
        } else {
            n.into()
        }
    }

    pub fn abs(&self) -> Self {
        self.0.abs().into()
    }

    pub fn dot(&self, other: Self) -> Float {
        self.0.dot(other.0).into()
    }

    pub fn cross(&self, other: Self) -> Self {
        self.0.cross(other.0).into()
    }

    pub fn normalize(&self) -> Self {
        self.0.normalize().into()
    }
}
