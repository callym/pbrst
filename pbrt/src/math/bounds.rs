use std::cmp::{ min, max };
use std::mem;
use std::ops::Add;
use cg::prelude::*;
use cg::{ BaseNum, Point2, Point3, Vector2, Vector3 };
use num::Bounded;

use prelude::*;

#[derive(Copy, Clone, Debug)]
pub struct Bounds2<T: BaseNum> {
    pub min: Point2<T>,
    pub max: Point2<T>,
}

impl<T: BaseNum> Bounds2<T> {
    pub fn empty() -> Self where T: Bounded {
        let min = T::min_value();
        let max = T::max_value();

        Self {
            min: Point2::new(max, max),
            max: Point2::new(min, min),
        }
    }

    pub fn new(p1: Point2<T>, p2: Point2<T>) -> Self where T: Ord {
        let min = Point2 {
            x: min(p1.x, p2.x),
            y: min(p1.y, p2.y),
        };

        let max = Point2 {
            x: max(p1.x, p2.x),
            y: max(p1.y, p2.y),
        };

        Self {
            min,
            max,
        }
    }

    pub fn from_point(p: Point2<T>) -> Self {
        Self {
            min: p,
            max: p,
        }
    }

    pub fn diagonal(&self) -> Vector2<T> {
        self.max - self.min
    }

    pub fn area(&self) -> T {
        let d = self.diagonal();
        d.x * d.y
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Bounds3<T: BaseNum + Ord + Bounded> {
    pub min: Point3<T>,
    pub max: Point3<T>,
}

impl<T: BaseNum + Ord + Bounded> Bounds3<T> {
    pub fn empty() -> Self {
        let min = T::min_value();
        let max = T::max_value();

        Self {
            min: Point3::new(max, max, max),
            max: Point3::new(min, min, min),
        }
    }

    pub fn new(p1: Point3<T>, p2: Point3<T>) -> Self {
        let min = Point3 {
            x: min(p1.x, p2.x),
            y: min(p1.y, p2.y),
            z: min(p1.z, p2.z),
        };

        let max = Point3 {
            x: max(p1.x, p2.x),
            y: max(p1.y, p2.y),
            z: max(p1.z, p2.z),
        };

        Self {
            min,
            max,
        }
    }

    pub fn from_point(p: Point3<T>) -> Self {
        Self {
            min: p,
            max: p,
        }
    }

    pub fn corner(&self, corner: u8) -> Point3<T> {
        debug_assert!(corner < 8_u8);

        let x = if corner & 1 == 0 {
            self.min.x
        } else {
            self.max.x
        };

        let y = if corner & 2 == 0 {
            self.min.y
        } else {
            self.max.y
        };

        let z = if corner & 4 == 0 {
            self.min.z
        } else {
            self.max.z
        };

        Point3::new(x, y, z)
    }

    pub fn union_p(&self, p: Point3<T>) -> Self {
        Self {
            min: Point3::new(min(self.min.x, p.x), min(self.min.y, p.y), min(self.min.z, p.z)),
            max: Point3::new(max(self.min.x, p.x), max(self.min.y, p.y), max(self.min.z, p.z)),
        }
    }

    pub fn union(&self, b: Bounds3<T>) -> Self {
        Self {
            min: Point3::new(min(self.min.x, b.min.x), min(self.min.y, b.min.y), min(self.min.z, b.min.z)),
            max: Point3::new(max(self.min.x, b.max.x), max(self.min.y, b.max.y), max(self.min.z, b.max.z)),
        }
    }

    pub fn intersect(&self, b: Bounds3<T>) -> Self {
        Self {
            min: Point3::new(max(self.min.x, b.min.x), max(self.min.y, b.min.y), max(self.min.z, b.min.z)),
            max: Point3::new(min(self.min.x, b.max.x), min(self.min.y, b.max.y), min(self.min.z, b.max.z)),
        }
    }

    pub fn overlaps(&self, b: Bounds3<T>) -> bool {
        let x = self.max.x >= b.min.x && self.min.x <= b.max.x;
        let y = self.max.y >= b.min.y && self.min.y <= b.max.y;
        let z = self.max.z >= b.min.z && self.min.z <= b.max.z;
        x && y && z
    }

    pub fn inside(&self, p: Point3<T>) -> bool {
        p.x >= self.min.x && p.x <= self.max.x &&
        p.y >= self.min.y && p.y <= self.max.y &&
        p.z >= self.min.z && p.z <= self.max.z
    }

    pub fn inside_exclusive(&self, p: Point3<T>) -> bool {
        p.x >= self.min.x && p.x < self.max.x &&
        p.y >= self.min.y && p.y < self.max.y &&
        p.z >= self.min.z && p.z < self.max.z
    }

    pub fn expand(&self, delta: T) -> Self {
        let delta = Vector3::new(delta, delta, delta);

        Self {
            min: self.min - delta,
            max: self.max + delta,
        }
    }

    pub fn diagonal(&self) -> Vector3<T> {
        self.max - self.min
    }

    pub fn surface_area(&self) -> T {
        let d = self.diagonal();
        let two = T::one() + T::one();
        two * ((d.x * d.y) + (d.x * d.z) + (d.y * d.z))
    }

    pub fn volume(&self) -> T {
        let d = self.diagonal();
        d.x * d.y * d.z
    }

    pub fn maximum_extend(&self) -> Dim {
        let d = self.diagonal();

        if d.x > d.y && d.x > d.z {
            Dim::X
        } else if d.y > d.z {
            Dim::Y
        } else {
            Dim::Z
        }
    }
}

pub struct Bounds2Iterator<'a> {
    point: Point2i,
    bounds: &'a Bounds2i,
}

impl<'a> Iterator for Bounds2Iterator<'a> {
    type Item = Point2i;

    fn next(&mut self) -> Option<Point2i> {
        self.point.x += 1;

        if self.point.x == self.bounds.max.x {
            self.point.x = self.bounds.min.x;
            self.point.y += 1;
        }

        if self.point.y == self.bounds.max.y {
            None
        } else {
            Some(self.point)
        }
    }
}

impl<'a> IntoIterator for &'a Bounds2i {
    type Item = Point2i;
    type IntoIter = Bounds2Iterator<'a>;

    fn into_iter(self) -> Self::IntoIter {
        Bounds2Iterator {
            // minus one so when `next()` is called,
            // it starts on the first element
            point: Point2i::new(self.min.x - 1, self.min.y),
            bounds: self,
        }
    }
}

impl Bounds3f {
    pub fn lerp(&self, amount: Point3f) -> Point3f {
        Point3f::new(
            self.min.x.lerp(self.max.x, amount.x),
            self.min.y.lerp(self.max.y, amount.y),
            self.min.z.lerp(self.max.z, amount.z),
        )
    }

    pub fn offset(&self, amount: Point3f) -> Vector3f {
        let mut o = amount - self.min;

        if self.max.x > self.min.x {
            o.x /= self.max.x - self.min.x;
        }

        if self.max.y > self.min.y {
            o.y /= self.max.y - self.min.y;
        }

        if self.max.z > self.min.z {
            o.z /= self.max.z - self.min.z;
        }

        o
    }

    pub fn bounding_sphere(&self) -> (Point3f, Float) {
        let min = self.min.into_vector();
        let max = self.max.into_vector();

        let center = (min + max) / float(2.0);
        let radius = if self.inside(center.into_point()) {
            center.distance(self.max.into_vector())
        } else {
            float(0.0)
        };

        (center.into_point(), radius)
    }

    pub fn intersect_p(&self, ray: Ray) -> Option<(Option<Float>, Option<Float>)> {
        let mut t0 = float(0.0);
        let mut t1 = ray.max.unwrap_or(Float::infinity());

        let hit_t0: Option<Float> = None;
        let hit_t1: Option<Float> = None;

        for i in 0..3 {
            // update interval for ith bb slab
            let inv_ray_dir = 1.0 / ray.direction[i].raw();
            let mut near = (self.min[i].raw() - ray.origin[i].raw()) * inv_ray_dir;
            let mut far = (self.max[i].raw() - ray.origin[i].raw()) * inv_ray_dir;

            if near > far {
                mem::swap(&mut near, &mut far);
            }

            // update far to ensure robuse ray-bounds intersection
            far *= 1.0 + 2.0 * gamma(3);

            t0 = if near > t0.raw() { float(near) } else { t0 };
            t1 = if far < t1.raw() { float(far) } else { t1 };

            if t0 > t1 {
                return None;
            }
        }

        hit_t0.map(|_| t0);
        hit_t1.map(|_| t1);

        Some((hit_t0, hit_t1))
    }

    pub fn intersect_p_precomputed(&self, ray: Ray, inv_dir: Vector3f, dir_is_negative: [bool; 3]) -> bool {
        let max = ray.max.unwrap_or(Float::infinity());

        let is_neg = |neg: bool| if neg { self.max } else { self.min };
        let mut tx_min = (is_neg(dir_is_negative[0]).x - ray.origin.x) * inv_dir.x;
        let mut tx_max = (is_neg(!dir_is_negative[0]).x - ray.origin.x) * inv_dir.x;
        let ty_min = (is_neg(dir_is_negative[1]).y - ray.origin.y) * inv_dir.y;
        let mut ty_max = (is_neg(!dir_is_negative[1]).y - ray.origin.y) * inv_dir.y;
        let tz_min = (is_neg(dir_is_negative[2]).z - ray.origin.z) * inv_dir.z;
        let mut tz_max = (is_neg(!dir_is_negative[2]).z - ray.origin.z) * inv_dir.z;

        tx_max  *= float(1.0 + 2.0 * gamma(3));
        ty_max  *= float(1.0 + 2.0 * gamma(3));
        tz_max  *= float(1.0 + 2.0 * gamma(3));

        if tx_min  > ty_max || ty_min > tx_max || tx_min > tx_max || tz_min > ty_max {
            return false;
        }

        if ty_min > tx_min {
            tx_min = ty_min;
        }

        if ty_max < tx_max {
            tx_max = ty_max;
        }

        if tz_min > tx_min {
            tx_min = tz_min;
        }

        if tz_max < tx_max {
            tx_max = tz_max;
        }

        tx_min < max && tx_max > 0.0
    }
}
