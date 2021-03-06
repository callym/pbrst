use cgmath::prelude::*;
use derive_more::{ From, Into };
use shrinkwraprs::Shrinkwrap;

use crate::prelude::*;

#[derive(Copy, Clone, Debug, From, Into, PartialEq, Shrinkwrap)]
#[shrinkwrap(mutable, unsafe_ignore_visibility)]
pub struct Normal(Vector3f);

impl Normal {
    pub fn new(x: impl Into<Float>, y: impl Into<Float>, z: impl Into<Float>) -> Self {
        Normal(Vector3f::new(x.into(), y.into(), z.into()))
    }

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
        self.0.dot(other.0)
    }

    pub fn cross(&self, other: Self) -> Self {
        self.0.cross(other.0).into()
    }

    pub fn normalize(&self) -> Self {
        self.0.normalize().into()
    }
}

impl Default for Normal {
    fn default() -> Self {
        Self::zero()
    }
}
