use std::cmp::{ min, max };
use num::traits::Zero as Zero;
use cg::prelude::*;
use cg::{ Point2, Point3, Vector2, Vector3 };
use prelude::*;

pub trait PointExt: Sized {
    type Element;

    fn min_component(&self) -> Self::Element;
    fn max_component(&self) -> Self::Element;

    fn min_dimension(&self) -> Dim;
    fn max_dimension(&self) -> Dim;
}

pub trait PointExtFloat: Sized {
    fn floor(self) -> Self;
    fn ceil(self) -> Self;
    fn abs(self) -> Self;

    fn lerp(self, other: Self, amount: Float) -> Self;
}

pub trait PointExt2d: PointExt {
    fn zero() -> Self where Self::Element: Zero;
    fn into_vector(self) -> Vector2<Self::Element>;
    fn permute(self, x: Dim, y: Dim) -> Self;
}

impl<T> PointExt2d for Point2<T> where Point2<T>: PointExt<Element = T>, T: Copy {
    #[inline(always)]
    fn zero() -> Self where Self::Element: Zero {
        Self::new(
            Self::Element::zero(),
            Self::Element::zero(),
        )
    }

    #[inline(always)]
    fn into_vector(self) -> Vector2<T> {
        Vector2::new(self.x, self.y)
    }

    #[inline(always)]
    fn permute(self, x: Dim, y: Dim) -> Self {
        let x: usize = x.into();
        let y: usize = y.into();

        Self::new(self[x], self[y])
    }
}

pub trait PointExt3d: PointExt {
    fn zero() -> Self where Self::Element: Zero;
    fn into_vector(self) -> Vector3<Self::Element>;
    fn permute(self, x: Dim, y: Dim, z: Dim) -> Self;
}

impl<T> PointExt3d for Point3<T> where Point3<T>: PointExt<Element = T>, T: Copy {
    #[inline(always)]
    fn zero() -> Self where Self::Element: Zero {
        Self::new(
            Self::Element::zero(),
            Self::Element::zero(),
            Self::Element::zero(),
        )
    }

    #[inline(always)]
    fn into_vector(self) -> Vector3<T> {
        Vector3::new(self.x, self.y, self.z)
    }

    #[inline(always)]
    fn permute(self, x: Dim, y: Dim, z: Dim) -> Self {
        let x: usize = x.into();
        let y: usize = y.into();
        let z: usize = z.into();

        Self::new(self[x], self[y], self[z])
    }
}

macro_rules! point_ext_2 {
    ($ty:ident, $elem:ident) => {
impl PointExt for $ty {
    type Element = $elem;

    #[inline(always)]
    fn min_component(&self) -> Self::Element {
        min(self.x, self.y)
    }

    #[inline(always)]
    fn max_component(&self) -> Self::Element {
        max(self.x, self.y)
    }

    #[inline(always)]
    fn min_dimension(&self) -> Dim {
        if self.x < self.y {
            Dim::X
        } else {
            Dim::Y
        }
    }

    #[inline(always)]
    fn max_dimension(&self) -> Dim {
        if self.x > self.y {
            Dim::X
        } else {
            Dim::Y
        }
    }
}
    };
}

macro_rules! point_ext_2_float {
    ($ty:ident, $elem:ident) => {
impl PointExtFloat for $ty {
    #[inline(always)]
    fn floor(self) -> Self {
        Self::new(self.x.floor(), self.y.floor())
    }

    #[inline(always)]
    fn ceil(self) -> Self {
        Self::new(self.x.ceil(), self.y.ceil())
    }

    #[inline(always)]
    fn abs(self) -> Self {
        Self::new(self.x.abs(), self.y.abs())
    }

    #[inline(always)]
    fn lerp(self, other: Self, amount: Float) -> Self {
        let s = self.into_vector();
        let o = other.into_vector();
        let l = s.lerp(o, amount);
        Point2::new(l.x, l.y)
    }
}
    };
}

macro_rules! point_ext_3 {
    ($ty:ident, $elem:ident) => {
impl PointExt for $ty {
    type Element = $elem;

    #[inline(always)]
    fn min_component(&self) -> Self::Element {
        min(self.x, min(self.y, self.z))
    }

    #[inline(always)]
    fn max_component(&self) -> Self::Element {
        max(self.x, max(self.y, self.z))
    }

    #[inline(always)]
    fn min_dimension(&self) -> Dim {
        if self.x < self.y {
            if self.x < self.z {
                Dim::X
            } else {
                Dim::Z
            }
        } else {
            if self.y < self.z {
                Dim::Y
            } else {
                Dim::Z
            }
        }
    }

    #[inline(always)]
    fn max_dimension(&self) -> Dim {
        if self.x > self.y {
            if self.x > self.z {
                Dim::X
            } else {
                Dim::Z
            }
        } else {
            if self.y > self.z {
                Dim::Y
            } else {
                Dim::Z
            }
        }
    }
}
    };
}

macro_rules! point_ext_3_float {
    ($ty:ident, $elem:ident) => {
impl PointExtFloat for Point3<$elem> {
    #[inline(always)]
    fn floor(self) -> Self {
        Self::new(self.x.floor(), self.y.floor(), self.z.floor())
    }

    #[inline(always)]
    fn ceil(self) -> Self {
        Self::new(self.x.ceil(), self.y.ceil(), self.z.ceil())
    }

    #[inline(always)]
    fn abs(self) -> Self {
        Self::new(self.x.abs(), self.y.abs(), self.z.abs())
    }

    #[inline(always)]
    fn lerp(self, other: Self, amount: Float) -> Self {
        let s = self.into_vector();
        let o = other.into_vector();
        let l = s.lerp(o, amount);
        Point3::new(l.x, l.y, l.z)
    }
}
    };
}

point_ext_2!(Point2f, Float);
point_ext_2_float!(Point2f, Float);
point_ext_2!(Point2i, i32);

point_ext_3!(Point3f, Float);
point_ext_3_float!(Point3f, Float);
point_ext_3!(Point3i, i32);
