use cgmath::prelude::*;
use shrinkwraprs::Shrinkwrap;
use crate::prelude::*;

pub fn offset_ray_origin(p: &Point3f, p_err: &Vector3f, n: &Normal, w: &Vector3f) -> Point3f {
    let d = (**n).abs().dot(*p_err);
    let mut offset = (**n) * d;

    if w.dot(**n) < 0.0 {
        offset = -offset;
    }

    let mut p_o = p + offset;

    for i in 0..3 {
        if offset[i] > 0.0 {
            p_o[i] = next_float_up_f(p_o[i]);
        } else if offset[i] < 0.0 {
            p_o[i] = next_float_down_f(p_o[i]);
        }
    }

    p_o
}

#[derive(Copy, Clone, Debug)]
pub struct RayData {
    pub origin: Point3f,
    pub direction: Vector3f,
}

impl RayData {
    pub fn new(origin: Point3f, direction: Vector3f) -> Self {
        Self { origin, direction }
    }

    pub fn position(&self, t: Float) -> Point3f {
        self.origin + self.direction * t
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Ray {
    pub origin: Point3f,
    pub direction: Vector3f,
    pub max: Float,
    pub time: Float,
    pub medium: Option<()>,
}

impl Ray {
    pub fn new(origin: Point3f, direction: Vector3f) -> Self {
        Self {
            origin,
            direction,
            max: Float::infinity(),
            time: float(0.0),
            medium: None,
        }
    }

    pub fn to_data(&self) -> RayData {
        RayData {
            origin: self.origin,
            direction: self.direction,
        }
    }

    pub fn position(&self, t: Float) -> Point3f {
        self.origin + self.direction * t
    }
}

#[derive(Copy, Clone, Debug, Shrinkwrap)]
#[shrinkwrap(mutable)]
pub struct RayDifferential {
    #[shrinkwrap(main_field)]
    pub ray: Ray,
    pub x: Option<RayData>,
    pub y: Option<RayData>,
}

impl RayDifferential {
    pub fn new(origin: Point3f, direction: Vector3f) -> Self {
        Self {
            ray: Ray::new(origin, direction),
            x: None,
            y: None,
        }
    }

    pub fn from_ray(ray: Ray) -> Self {
        Self {
            ray,
            x: None,
            y: None,
        }
    }

    pub fn has_differentials(&self) -> bool {
        self.x.is_some() && self.y.is_some()
    }

    pub fn scale_differentials(&mut self, scale: Float) {
        let o = self.origin;
        let d = self.direction;

        if let Some(x) = &mut self.x {
            x.origin = o + (x.origin - o) * scale;
            x.direction = d + (x.direction - d) * scale;
        }

        if let Some(y) = &mut self.y {
            y.origin = o + (y.origin - o) * scale;
            y.direction = d + (y.direction - d) * scale;
        }
    }
}
