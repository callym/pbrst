use std::cmp::{ min, max };
use num::integer::Roots as _;
use cg::prelude::*;
use cg::{ Point2, Point3, Vector3 };
use prelude::*;

pub trait VectorExt {
    type Element;

    fn min_component(&self) -> Self::Element;
    fn max_component(&self) -> Self::Element;

    fn min_dimension(&self) -> Dim;
    fn max_dimension(&self) -> Dim;

    fn length_squared(&self) -> Self::Element;
}

pub trait VectorExtFloat: Sized {
    fn floor(self) -> Self;
    fn ceil(self) -> Self;
    fn abs(self) -> Self;
}

pub trait VectorExt2d: VectorExt {
    fn into_point(self) -> Point2<Self::Element>;

    fn permute(self, x: Dim, y: Dim) -> Self;
}

pub trait VectorExt3d: VectorExt {
    type Vector;

    fn into_point(self) -> Point3<Self::Element>;

    fn coord_system(self) -> (Self::Vector, Self::Vector, Self::Vector);

    fn permute(self, x: Dim, y: Dim, z: Dim) -> Self;
}

macro_rules! vector_ext_2 {
    ($ty:ident, $elem:ident) => {
impl VectorExt for $ty {
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

    #[inline(always)]
    fn length_squared(&self) -> Self::Element {
        self.x.pow(2) + self.y.pow(2)
    }
}

impl VectorExt2d for $ty {
    #[inline(always)]
    fn into_point(self) -> Point2<Self::Element> {
        Point2::new(self.x, self.y)
    }

    #[inline(always)]
    fn permute(self, x: Dim, y: Dim) -> Self {
        let x: usize = x.into();
        let y: usize = y.into();

        Self::new(self[x], self[y])
    }
}
    };
}

macro_rules! vector_ext_2_float {
    ($ty:ident, $elem:ident) => {
impl VectorExtFloat for $ty {
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
}
    };
}

macro_rules! vector_ext_3 {
    ($ty:ident, $elem:ident) => {
impl VectorExt for $ty {
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

    #[inline(always)]
    fn length_squared(&self) -> Self::Element {
        self.x.pow(2) + self.y.pow(2) + self.z.pow(2)
    }
}

impl VectorExt3d for $ty {
    type Vector = Vector3<Self::Element>;

    #[inline(always)]
    fn into_point(self) -> Point3<Self::Element> {
        Point3::new(self.x, self.y, self.z)
    }

    fn coord_system(self) -> (Self, Self, Self) {
        let v1 = self;

        let v2 = if self.x.abs() > self.y.abs() {
            Self::new(-v1.z, Self::Element::zero(), v1.x) / (v1.x.pow(2) + v1.z.pow(2)).sqrt()
        } else {
            Self::new(Self::Element::zero(), v1.z, -v1.y) / (v1.y.pow(2) + v1.z.pow(2)).sqrt()
        };

        let v3 = v1.cross(v2);

        (v1, v2, v3)
    }

    #[inline(always)]
    fn permute(self, x: Dim, y: Dim, z: Dim) -> Self {
        let x: usize = x.into();
        let y: usize = y.into();
        let z: usize = z.into();

        Self::new(self[x], self[y], self[z])
    }
}
    };
}

macro_rules! vector_ext_3_float {
    ($ty:ident, $elem:ident) => {
impl VectorExtFloat for Vector3<$elem> {
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
}
    };
}

vector_ext_2!(Vector2f, Float);
vector_ext_2_float!(Vector2f, Float);
vector_ext_2!(Vector2i, i32);

vector_ext_3!(Vector3f, Float);
vector_ext_3_float!(Vector3f, Float);
vector_ext_3!(Vector3i, i32);
